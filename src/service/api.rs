//! Service API traits and client implementations

use std::collections::HashMap;
use std::sync::Arc;
use reqwest::Client;
use serde_json;
use futures::stream::{Stream};

use crate::service::types::{
    TaskRequest, TaskResponse, BatchTaskRequest, BatchTaskResponse,
    ServiceStatus, WebSocketMessage, TaskContext, BatchExecutionMode,
    ServiceError,
};
use crate::service::{ServiceResult, CodeAgentService, MetricsSnapshot};
use crate::service::error::ErrorBuilder;

/// Main API trait for Code Agent Service
#[async_trait::async_trait]
pub trait CodeAgentApi: Send + Sync {
    /// Execute a single task
    async fn execute_task(&self, request: TaskRequest) -> ServiceResult<TaskResponse>;

    /// Execute a batch of tasks
    async fn execute_batch(&self, request: BatchTaskRequest) -> ServiceResult<BatchTaskResponse>;

    /// Get task status
    async fn get_task_status(&self, task_id: &str) -> ServiceResult<TaskResponse>;

    /// Cancel a running task
    async fn cancel_task(&self, task_id: &str) -> ServiceResult<()>;

    /// Get service status
    async fn get_service_status(&self) -> ServiceResult<ServiceStatus>;

    /// Get metrics
    async fn get_metrics(&self) -> ServiceResult<MetricsSnapshot>;

    /// Subscribe to task updates (WebSocket-like)
    async fn subscribe_to_task_updates(&self, task_id: &str) -> ServiceResult<Box<dyn Stream<Item = WebSocketMessage> + Send>>;
}

/// In-process API implementation
pub struct InProcessApi {
    service: Arc<CodeAgentService>,
}

impl InProcessApi {
    /// Create a new in-process API
    pub fn new(service: Arc<CodeAgentService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CodeAgentApi for InProcessApi {
    async fn execute_task(&self, request: TaskRequest) -> ServiceResult<TaskResponse> {
        self.service.execute_task(request).await
    }

    async fn execute_batch(&self, request: BatchTaskRequest) -> ServiceResult<BatchTaskResponse> {
        self.service.execute_batch(request).await
    }

    async fn get_task_status(&self, task_id: &str) -> ServiceResult<TaskResponse> {
        self.service.get_task_status(task_id).await
    }

    async fn cancel_task(&self, task_id: &str) -> ServiceResult<()> {
        self.service.cancel_task(task_id).await
    }

    async fn get_service_status(&self) -> ServiceResult<ServiceStatus> {
        self.service.get_service_status().await
    }

    async fn get_metrics(&self) -> ServiceResult<MetricsSnapshot> {
        self.service.get_metrics().await
    }

    async fn subscribe_to_task_updates(&self, _task_id: &str) -> ServiceResult<Box<dyn Stream<Item = WebSocketMessage> + Send>> {
        // TODO: Implement in-process event streaming
        Err(ErrorBuilder::service_unavailable("Task subscription not available for in-process API"))
    }
}

/// HTTP Client API implementation
pub struct HttpClientApi {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl HttpClientApi {
    /// Create a new HTTP client API
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            api_key: None,
        }
    }

    /// Set API key for authentication
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Build a request with authentication
    async fn build_request(&self, method: reqwest::Method, path: &str) -> ServiceResult<reqwest::RequestBuilder> {
        let url = format!("{}/api/v1{}", self.base_url.trim_end_matches('/'), path);
        let mut request = self.client.request(method, &url);

        // Add authentication header if API key is provided
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        request = request.header("Content-Type", "application/json");

        Ok(request)
    }

    /// Handle HTTP response
    async fn handle_response<T>(&self, response: reqwest::Response) -> ServiceResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let status = response.status();
        let response_text = response.text().await
            .map_err(|e| ErrorBuilder::service_unavailable(format!("Failed to read response: {}", e)))?;

        if status.is_success() {
            serde_json::from_str(&response_text)
                .map_err(|e| ErrorBuilder::service_unavailable(format!("Failed to parse response: {}", e)))
        } else {
            let service_error: ServiceError = serde_json::from_str(&response_text)
                .unwrap_or_else(|_| ServiceError {
                    code: "HTTP_ERROR".to_string(),
                    message: format!("HTTP {}: {}", status.as_u16(), response_text),
                    details: None,
                    stack_trace: None,
                    timestamp: chrono::Utc::now(),
                });

            Err(service_error)
        }
    }
}

#[async_trait::async_trait]
impl CodeAgentApi for HttpClientApi {
    async fn execute_task(&self, request: TaskRequest) -> ServiceResult<TaskResponse> {
        let request_builder = self.build_request(reqwest::Method::POST, "/tasks").await?;
        let response = request_builder.json(&request).send().await
            .map_err(|e| ErrorBuilder::service_unavailable(format!("Failed to send request: {}", e)))?;

        self.handle_response(response).await
    }

    async fn execute_batch(&self, request: BatchTaskRequest) -> ServiceResult<BatchTaskResponse> {
        let request_builder = self.build_request(reqwest::Method::POST, "/tasks/batch").await?;
        let response = request_builder.json(&request).send().await
            .map_err(|e| ErrorBuilder::service_unavailable(format!("Failed to send request: {}", e)))?;

        self.handle_response(response).await
    }

    async fn get_task_status(&self, task_id: &str) -> ServiceResult<TaskResponse> {
        let request_builder = self.build_request(reqwest::Method::GET, &format!("/tasks/{}", task_id)).await?;
        let response = request_builder.send().await
            .map_err(|e| ErrorBuilder::service_unavailable(format!("Failed to send request: {}", e)))?;

        self.handle_response(response).await
    }

    async fn cancel_task(&self, task_id: &str) -> ServiceResult<()> {
        let request_builder = self.build_request(reqwest::Method::DELETE, &format!("/tasks/{}", task_id)).await?;
        let response = request_builder.send().await
            .map_err(|e| ErrorBuilder::service_unavailable(format!("Failed to send request: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let response_text = response.text().await.unwrap_or_default();
            Err(ErrorBuilder::service_unavailable(format!("Failed to cancel task: {}", response_text)))
        }
    }

    async fn get_service_status(&self) -> ServiceResult<ServiceStatus> {
        let request_builder = self.build_request(reqwest::Method::GET, "/status").await?;
        let response = request_builder.send().await
            .map_err(|e| ErrorBuilder::service_unavailable(format!("Failed to send request: {}", e)))?;

        self.handle_response(response).await
    }

    async fn get_metrics(&self) -> ServiceResult<MetricsSnapshot> {
        let request_builder = self.build_request(reqwest::Method::GET, "/metrics").await?;
        let response = request_builder.send().await
            .map_err(|e| ErrorBuilder::service_unavailable(format!("Failed to send request: {}", e)))?;

        self.handle_response(response).await
    }

    async fn subscribe_to_task_updates(&self, _task_id: &str) -> ServiceResult<Box<dyn Stream<Item = WebSocketMessage> + Send>> {
        // TODO: Implement WebSocket client for task updates
        Err(ErrorBuilder::service_unavailable("WebSocket client not implemented"))
    }
}

/// Builder for creating API clients
pub struct ApiClientBuilder;

impl ApiClientBuilder {
    /// Create an in-process API client
    pub fn in_process(service: Arc<CodeAgentService>) -> Box<dyn CodeAgentApi> {
        Box::new(InProcessApi::new(service))
    }

    /// Create an HTTP client
    pub fn http(base_url: impl Into<String>) -> HttpClientApi {
        HttpClientApi::new(base_url)
    }

    /// Create an HTTP client with authentication
    pub fn http_with_auth(base_url: impl Into<String>, api_key: impl Into<String>) -> HttpClientApi {
        HttpClientApi::new(base_url).with_api_key(api_key)
    }

    /// Create a client from environment configuration
    pub fn from_env() -> ServiceResult<Box<dyn CodeAgentApi>> {
        let base_url = std::env::var("CODE_AGENT_API_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());

        let api_key = std::env::var("CODE_AGENT_API_KEY").ok();

        if let Some(key) = api_key {
            Ok(Box::new(Self::http_with_auth(base_url, key)))
        } else {
            Ok(Box::new(Self::http(base_url)))
        }
    }
}

/// Simple client wrapper for convenient usage
pub struct CodeAgentClient {
    api: Box<dyn CodeAgentApi>,
}

impl CodeAgentClient {
    /// Create a new client
    pub fn new(api: Box<dyn CodeAgentApi>) -> Self {
        Self { api }
    }

    /// Execute a task with minimal parameters
    pub async fn execute_simple_task(&self, task: &str) -> ServiceResult<TaskResponse> {
        let request = TaskRequest {
            task: task.to_string(),
            task_id: None,
            context: None,
            priority: None,
            metadata: None,
        };

        self.api.execute_task(request).await
    }

    /// Execute a task with custom context
    pub async fn execute_task_with_context(
        &self,
        task: &str,
        working_directory: Option<&str>,
        environment: Option<HashMap<String, String>>,
    ) -> ServiceResult<TaskResponse> {
        let request = TaskRequest {
            task: task.to_string(),
            task_id: None,
            context: Some(TaskContext {
                working_directory: working_directory.map(|s| s.to_string()),
                environment,
                tools: None,
                constraints: None,
            }),
            priority: None,
            metadata: None,
        };

        self.api.execute_task(request).await
    }

    /// Get a reference to the underlying API
    pub fn api(&self) -> &dyn CodeAgentApi {
        self.api.as_ref()
    }
}

impl std::ops::Deref for CodeAgentClient {
    type Target = dyn CodeAgentApi;

    fn deref(&self) -> &Self::Target {
        self.api.as_ref()
    }
}

/// Example usage functions
pub mod examples {
    use super::*;

    /// Example of using the in-process API
    pub async fn in_process_example() -> ServiceResult<()> {
        // This would typically be created from the service
        // let service = Arc::new(AiAgentService::new(service_config, agent_config).await?);
        // let client = CodeAgentClient::new(ApiClientBuilder::in_process(service));

        // Execute a simple task
        // let response = client.execute_simple_task("Read the README.md file and summarize it").await?;
        // println!("Task result: {}", response.result.unwrap_or_default().summary);

        Ok(())
    }

    /// Example of using the HTTP API
    pub async fn http_example() -> ServiceResult<()> {
        let client = CodeAgentClient::new(
            Box::new(ApiClientBuilder::http_with_auth("http://localhost:8080", "your-api-key"))
        );

        // Get service status
        let status = client.get_service_status().await?;
        println!("Service status: {:?}", status);

        // Execute a task
        let response = client.execute_simple_task("List files in current directory").await?;
        println!("Task result: {}", response.result.unwrap_or_default().summary);

        Ok(())
    }

    /// Example of batch processing
    pub async fn batch_example() -> ServiceResult<()> {
        let client = CodeAgentClient::new(ApiClientBuilder::from_env()?);

        let batch_request = BatchTaskRequest {
            tasks: vec![
                TaskRequest {
                    task: "Read Cargo.toml".to_string(),
                    task_id: None,
                    context: None,
                    priority: None,
                    metadata: None,
                },
                TaskRequest {
                    task: "List source files".to_string(),
                    task_id: None,
                    context: None,
                    priority: None,
                    metadata: None,
                },
            ],
            mode: BatchExecutionMode::Parallel,
            metadata: None,
            continue_on_error: true,
        };

        let batch_response = client.execute_batch(batch_request).await?;
        println!("Batch completed: {} tasks", batch_response.statistics.total_tasks);

        Ok(())
    }
}