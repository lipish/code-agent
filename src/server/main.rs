//! Code Agent Service HTTP Server

use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, HeaderMap},
    response::{Json, Response},
    routing::{delete, get, post},
    Router,
    middleware,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use code_agent::service::{CodeAgentService, ServiceConfig, TaskRequest, BatchTaskRequest, TaskResponse, BatchTaskResponse, ServiceStatus, MetricsSnapshot, ServiceError};
use code_agent::config::AgentConfig;

#[derive(Clone)]
struct AppState {
    service: Arc<CodeAgentService>,
    config: Arc<tokio::sync::RwLock<AgentConfig>>,
}

#[derive(Deserialize)]
struct TaskQuery {
    #[serde(default)]
    verbose: bool,
}

#[derive(Deserialize)]
struct PaginationQuery {
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default = "default_offset")]
    offset: usize,
}

#[derive(Deserialize)]
struct ModelConfigUpdate {
    provider: Option<String>,
    model_name: Option<String>,
    api_key: Option<String>,
    endpoint: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct ConfigValidationRequest {
    config: serde_json::Value,
}

#[derive(Serialize)]
struct ConfigValidationResponse {
    valid: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
}

fn default_limit() -> usize { 50 }
fn default_offset() -> usize { 0 }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Code Agent Service HTTP Server");

    // Load configuration
    let service_config = load_service_config().await?;
    let agent_config = load_agent_config().await?;

    // Create service
    let config = Arc::new(tokio::sync::RwLock::new(agent_config.clone()));
    let service = Arc::new(CodeAgentService::new(service_config.clone(), agent_config).await?);

    // Create router
    let app = create_router(service.clone(), config.clone(), service_config.clone());

    // Start server
    let bind_addr = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let addr: SocketAddr = bind_addr.parse()
        .map_err(|_| anyhow::anyhow!("Invalid bind address: {}", bind_addr))?;

    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(service: Arc<CodeAgentService>, config: Arc<tokio::sync::RwLock<AgentConfig>>, service_config: ServiceConfig) -> Router {
    let state = AppState { service, config };

    Router::new()
        // Health check
        .route("/health", get(health_check))
        .route("/healthz", get(health_check))

        // API routes
        .route("/api/v1/status", get(get_service_status))
        .route("/api/v1/metrics", get(get_metrics))
        .route("/api/v1/tools", get(list_tools))

        // Task management
        .route("/api/v1/tasks", post(execute_task))
        .route("/api/v1/tasks/batch", post(execute_batch))
        .route("/api/v1/tasks/:task_id", get(get_task_status))
        .route("/api/v1/tasks/:task_id", delete(cancel_task))

        // Configuration management
        .route("/api/v1/config", get(get_config))
        .route("/api/v1/config", put(update_config))
        .route("/api/v1/config/model", put(update_model_config))
        .route("/api/v1/config/validate", post(validate_config))

        // Legacy routes for backward compatibility
        .route("/tasks", post(execute_task_legacy))
        .route("/config", get(get_service_status))

        .nest_service("/metrics", axum::routing::get(prometheus_metrics_handler))

        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(middleware::from_fn(request_id_middleware))
        )
        .with_state(state)
}

async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "service": "code-agent-service",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

async fn get_service_status(State(state): State<AppState>) -> Result<Json<ServiceStatus>, ServiceError> {
    state.service.get_service_status().await.map(Json)
}

async fn get_metrics(State(state): State<AppState>) -> Result<Json<MetricsSnapshot>, ServiceError> {
    state.service.get_metrics().await.map(Json)
}

async fn list_tools() -> Result<Json<serde_json::Value>, ServiceError> {
    Ok(Json(serde_json::json!({
        "tools": [
            {
                "name": "read_file",
                "description": "Read the contents of a file",
                "parameters": ["path"]
            },
            {
                "name": "write_file",
                "description": "Write content to a file",
                "parameters": ["path", "content"]
            },
            {
                "name": "run_command",
                "description": "Execute a shell command",
                "parameters": ["command", "working_dir"]
            },
            {
                "name": "list_files",
                "description": "List files and directories",
                "parameters": ["path"]
            }
        ]
    })))
}

async fn execute_task(
    State(state): State<AppState>,
    Json(request): Json<TaskRequest>,
) -> Result<Json<TaskResponse>, ServiceError> {
    info!("Executing task: {}", request.task);
    state.service.execute_task(request).await.map(Json)
}

async fn execute_task_legacy(
    State(state): State<AppState>,
    Json(request): serde_json::Value,
) -> Result<Json<serde_json::Value>, ServiceError> {
    // Legacy format support for backward compatibility
    let task = request.get("task")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ServiceError {
            code: "INVALID_REQUEST".to_string(),
            message: "Missing 'task' field in request".to_string(),
            details: None,
            stack_trace: None,
            timestamp: chrono::Utc::now(),
        })?;

    let task_request = TaskRequest {
        task: task.to_string(),
        task_id: request.get("task_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
        context: None,
        priority: None,
        metadata: None,
    };

    let response = state.service.execute_task(task_request).await?;

    Ok(Json(serde_json::json!({
        "success": response.result.as_ref().map(|r| r.success).unwrap_or(false),
        "summary": response.result.as_ref().map(|r| r.summary.clone()).unwrap_or_default(),
        "details": response.result.as_ref().and_then(|r| r.details.clone()),
        "task_id": response.task_id,
        "status": response.status,
        "execution_time": response.result.as_ref().map(|r| r.execution_time).unwrap_or(0)
    })))
}

async fn execute_batch(
    State(state): State<AppState>,
    Json(request): Json<BatchTaskRequest>,
) -> Result<Json<BatchTaskResponse>, ServiceError> {
    info!("Executing batch with {} tasks", request.tasks.len());
    state.service.execute_batch(request).await.map(Json)
}

async fn get_task_status(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<TaskResponse>, ServiceError> {
    state.service.get_task_status(&task_id).await.map(Json)
}

async fn cancel_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<StatusCode, ServiceError> {
    state.service.cancel_task(&task_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Configuration management endpoints

async fn get_config(State(state): State<AppState>) -> Result<Json<serde_json::Value>, ServiceError> {
    let config = state.config.read().await;
    Ok(Json(serde_json::to_value(&*config).map_err(|e| ServiceError {
        code: "SERIALIZATION_ERROR".to_string(),
        message: format!("Failed to serialize config: {}", e),
        details: None,
        stack_trace: None,
        timestamp: chrono::Utc::now(),
    })?))
}

async fn update_config(
    State(state): State<AppState>,
    Json(new_config): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    // Validate and parse new config
    let parsed_config: AgentConfig = serde_json::from_value(new_config).map_err(|e| ServiceError {
        code: "INVALID_CONFIG".to_string(),
        message: format!("Invalid configuration format: {}", e),
        details: None,
        stack_trace: None,
        timestamp: chrono::Utc::now(),
    })?;

    // Update the configuration
    {
        let mut config = state.config.write().await;
        *config = parsed_config;
    }

    Ok(Json(serde_json::json!({
        "message": "Configuration updated successfully",
        "timestamp": chrono::Utc::now()
    })))
}

async fn update_model_config(
    State(state): State<AppState>,
    Json(model_update): Json<ModelConfigUpdate>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let mut config = state.config.write().await;

    // Update model configuration fields
    if let Some(provider) = model_update.provider {
        config.model.provider = match provider.as_str() {
            "zhipu" => code_agent::config::ModelProvider::Zhipu,
            "openai" => code_agent::config::ModelProvider::OpenAI,
            "anthropic" => code_agent::config::ModelProvider::Anthropic,
            "local" => code_agent::config::ModelProvider::Local(
                model_update.endpoint.clone().unwrap_or_else(|| "http://localhost:8081".to_string())
            ),
            _ => return Err(ServiceError {
                code: "INVALID_PROVIDER".to_string(),
                message: format!("Unsupported model provider: {}", provider),
                details: None,
                stack_trace: None,
                timestamp: chrono::Utc::now(),
            }),
        };
    }

    if let Some(model_name) = model_update.model_name {
        config.model.model_name = model_name;
    }

    if let Some(api_key) = model_update.api_key {
        config.model.api_key = Some(api_key);
    }

    if let Some(endpoint) = model_update.endpoint {
        config.model.endpoint = Some(endpoint);
    }

    if let Some(max_tokens) = model_update.max_tokens {
        config.model.max_tokens = max_tokens;
    }

    if let Some(temperature) = model_update.temperature {
        config.model.temperature = temperature;
    }

    info!("Model configuration updated via API");

    Ok(Json(serde_json::json!({
        "message": "Model configuration updated successfully",
        "config": serde_json::to_value(&*config).map_err(|e| ServiceError {
            code: "SERIALIZATION_ERROR".to_string(),
            message: format!("Failed to serialize config: {}", e),
            details: None,
            stack_trace: None,
            timestamp: chrono::Utc::now(),
        })?,
        "timestamp": chrono::Utc::now()
    })))
}

async fn validate_config(
    _State(_state): State<AppState>,
    Json(request): Json<ConfigValidationRequest>,
) -> Result<Json<ConfigValidationResponse>, ServiceError> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Try to parse the configuration
    let config: AgentConfig = match serde_json::from_value(request.config) {
        Ok(config) => config,
        Err(e) => {
            errors.push(format!("Invalid configuration format: {}", e));
            return Ok(Json(ConfigValidationResponse {
                valid: false,
                errors,
                warnings,
            }));
        }
    };

    // Validate model configuration
    if config.model.api_key.is_none() || config.model.api_key.as_ref().unwrap().is_empty() {
        errors.push("API key is required".to_string());
    }

    if config.model.model_name.is_empty() {
        errors.push("Model name is required".to_string());
    }

    if config.model.max_tokens == 0 {
        warnings.push("Max tokens is set to 0, this may limit the response length".to_string());
    }

    if config.model.temperature < 0.0 || config.model.temperature > 2.0 {
        warnings.push("Temperature should be between 0.0 and 2.0 for best results".to_string());
    }

    // Validate execution configuration
    if config.execution.timeout_seconds == 0 {
        warnings.push("Timeout is set to 0, tasks may run indefinitely".to_string());
    }

    if config.execution.max_steps == 0 {
        warnings.push("Max steps is set to 0, tasks may not complete properly".to_string());
    }

    // Validate safety configuration
    if config.safety.allowed_directories.is_empty() {
        warnings.push("No allowed directories configured, file access may be restricted".to_string());
    }

    let valid = errors.is_empty();

    Ok(Json(ConfigValidationResponse {
        valid,
        errors,
        warnings,
    }))
}

async fn request_id_middleware<B>(
    request: axum::extract::Request<B>,
    next: axum::middleware::Next<B>,
) -> axum::response::Response {
    let request_id = uuid::Uuid::new_v4().to_string();

    // Add request ID to response headers
    let mut response = next.run(request).await;
    response.headers_mut().insert("x-request-id", request_id.parse().unwrap());

    response
}

async fn prometheus_metrics_handler() -> Result<String, StatusCode> {
    // This is a placeholder for Prometheus metrics
    // In a real implementation, you would use the metrics-exporter-prometheus crate
    Ok("# HELP code_agent_requests_total Total number of requests
# TYPE code_agent_requests_total counter
code_agent_requests_total 0

# HELP code_agent_request_duration_seconds Request duration
# TYPE code_agent_request_duration_seconds histogram
code_agent_request_duration_seconds_bucket{le=\"1.0\"} 0
code_agent_request_duration_seconds_bucket{le=\"+Inf\"} 0
code_agent_request_duration_seconds_count 0
code_agent_request_duration_seconds_sum 0

# HELP code_agent_active_tasks Number of active tasks
# TYPE code_agent_active_tasks gauge
code_agent_active_tasks 0

# HELP code_agent_completed_tasks_total Total number of completed tasks
# TYPE code_agent_completed_tasks_total counter
code_agent_completed_tasks_total 0

# HELP code_agent_failed_tasks_total Total number of failed tasks
# TYPE code_agent_failed_tasks_total counter
code_agent_failed_tasks_total 0".to_string())
}

async fn load_service_config() -> Result<ServiceConfig, anyhow::Error> {
    // Try to load from environment or use defaults
    let max_concurrent_tasks = std::env::var("CODE_AGENT_MAX_CONCURRENT_TASKS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let default_task_timeout = std::env::var("CODE_AGENT_DEFAULT_TASK_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(300);

    let enable_metrics = std::env::var("CODE_AGENT_ENABLE_METRICS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(true);

    Ok(ServiceConfig {
        max_concurrent_tasks,
        default_task_timeout,
        max_task_timeout: 3600,
        enable_metrics,
        log_level: std::env::var("CODE_AGENT_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        cors: code_agent::service::CorsConfig {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "DELETE".to_string()],
            allowed_headers: vec!["*".to_string()],
            allow_credentials: false,
        },
        rate_limiting: None,
    })
}

async fn load_agent_config() -> Result<AgentConfig, anyhow::Error> {
    // Try to load from config file or environment
    let config_path = std::env::var("CODE_AGENT_CONFIG_FILE")
        .unwrap_or_else(|_| "config.toml".to_string());

    if std::path::Path::new(&config_path).exists() {
        AgentConfig::load_with_fallback(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))
    } else {
        // Create default config from environment
        let provider = std::env::var("CODE_AGENT_MODEL_PROVIDER")
            .unwrap_or_else(|_| "zhipu".to_string());

        let model_name = std::env::var("CODE_AGENT_MODEL_NAME")
            .unwrap_or_else(|_| "glm-4".to_string());

        let api_key = std::env::var("CODE_AGENT_API_KEY")
            .ok_or_else(|| anyhow::anyhow!("CODE_AGENT_API_KEY environment variable is required"))?;

        let provider_config = match provider.as_str() {
            "zhipu" => code_agent::config::ModelProvider::Zhipu,
            "openai" => code_agent::config::ModelProvider::OpenAI,
            "anthropic" => code_agent::config::ModelProvider::Anthropic,
            "local" => code_agent::config::ModelProvider::Local(
                std::env::var("CODE_AGENT_LOCAL_ENDPOINT").unwrap_or_else(|_| "http://localhost:8081".to_string())
            ),
            _ => return Err(anyhow::anyhow!("Unsupported model provider: {}", provider)),
        };

        Ok(AgentConfig {
            model: code_agent::config::ModelConfig {
                provider: provider_config,
                model_name,
                api_key: Some(api_key),
                endpoint: std::env::var("CODE_AGENT_MODEL_ENDPOINT").ok(),
                max_tokens: std::env::var("CODE_AGENT_MAX_TOKENS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(4000),
                temperature: std::env::var("CODE_AGENT_TEMPERATURE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.7),
            },
            execution: code_agent::config::ExecutionConfig {
                max_steps: std::env::var("CODE_AGENT_MAX_STEPS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
                max_retries: std::env::var("CODE_AGENT_MAX_RETRIES")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3),
                retry_delay_seconds: std::env::var("CODE_AGENT_RETRY_DELAY")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1),
                timeout_seconds: std::env::var("CODE_AGENT_TIMEOUT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300),
            },
            tools: code_agent::config::ToolsConfig {
                enable_file_operations: std::env::var("CODE_AGENT_ENABLE_FILE_OPS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
                enable_command_execution: std::env::var("CODE_AGENT_ENABLE_COMMANDS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
                working_directory: std::env::var("CODE_AGENT_WORK_DIR").ok(),
                allowed_paths: std::env::var("CODE_AGENT_ALLOWED_PATHS")
                    .ok()
                    .map(|s| s.split(',').map(|p| p.trim().to_string()).collect()),
                forbidden_commands: std::env::var("CODE_AGENT_FORBIDDEN_COMMANDS")
                    .ok()
                    .map(|s| s.split(',').map(|c| c.trim().to_string()).collect()),
            },
        })
    }
}