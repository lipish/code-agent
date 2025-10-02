//! Language model abstraction and implementations

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::ModelError;

/// Language model trait
#[async_trait]
pub trait LanguageModel: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError>;
    async fn complete_with_tools(&self, prompt: &str, tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError>;
    fn model_name(&self) -> &str;
    fn supports_tools(&self) -> bool;
}

/// Model response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub usage: Option<TokenUsage>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ModelResponse {
    pub fn text(content: String) -> Self {
        Self {
            content,
            tool_calls: vec![],
            usage: None,
            metadata: HashMap::new(),
        }
    }
}

/// Tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
    pub id: Option<String>,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Model capabilities
#[derive(Debug, Clone)]
pub struct ModelCapabilities {
    pub max_tokens: u32,
    pub supports_tools: bool,
    pub supports_streaming: bool,
    pub supports_vision: bool,
}

impl Default for ModelCapabilities {
    fn default() -> Self {
        Self {
            max_tokens: 4096,
            supports_tools: false,
            supports_streaming: false,
            supports_vision: false,
        }
    }
}

// Mock model for testing
pub struct MockModel {
    name: String,
}

impl MockModel {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait]
impl LanguageModel for MockModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        Ok(ModelResponse::text(format!("Mock response from {}: {}", self.name, prompt)))
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        let response = format!("Mock response with tools from {}: {}", self.name, prompt);
        Ok(ModelResponse {
            content: response,
            tool_calls: vec![],
            usage: None,
            metadata: HashMap::new(),
        })
    }

    fn model_name(&self) -> &str {
        &self.name
    }

    fn supports_tools(&self) -> bool {
        true
    }
}

// OpenAI model implementation (placeholder)
pub struct OpenAIModel {
    _api_key: String,
    model: String,
}

impl OpenAIModel {
    pub fn new(api_key: String, model: String) -> Self {
        Self { _api_key: api_key, model }
    }
}

#[async_trait]
impl LanguageModel for OpenAIModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        // TODO: Implement OpenAI API call
        Ok(ModelResponse::text(format!("OpenAI response from {}: {}", self.model, prompt)))
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // TODO: Implement OpenAI API call with tools
        Ok(ModelResponse::text(format!("OpenAI response with tools from {}: {}", self.model, prompt)))
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn supports_tools(&self) -> bool {
        true
    }
}

// Zhipu model implementation (placeholder)
pub struct ZhipuModel {
    _api_key: String,
    model: String,
    endpoint: Option<String>,
}

impl ZhipuModel {
    pub fn new(api_key: String, model: String, endpoint: Option<String>) -> Self {
        Self { _api_key: api_key, model, endpoint }
    }
}

#[async_trait]
impl LanguageModel for ZhipuModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        let client = reqwest::Client::new();
        let endpoint = self.endpoint.as_ref()
            .map(|e| format!("{}/chat/completions", e))
            .unwrap_or_else(|| "https://open.bigmodel.cn/api/paas/v4/chat/completions".to_string());

        let request_body = serde_json::json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 4000,
            "temperature": 0.7
        });

        let response = client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", self._api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ModelError::APIError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ModelError::APIError(format!("API request failed with status {}: {}", status, error_text)));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ModelError::APIError(e.to_string()))?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| ModelError::APIError("Invalid response format".to_string()))?;

        Ok(ModelResponse::text(content.to_string()))
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // For now, just use the regular complete method
        self.complete(prompt).await
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn supports_tools(&self) -> bool {
        true
    }
}

// Anthropic model implementation (placeholder)
pub struct AnthropicModel {
    _api_key: String,
    model: String,
}

impl AnthropicModel {
    pub fn new(api_key: String, model: String) -> Self {
        Self { _api_key: api_key, model }
    }
}

#[async_trait]
impl LanguageModel for AnthropicModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        // TODO: Implement Anthropic API call
        Ok(ModelResponse::text(format!("Anthropic response from {}: {}", self.model, prompt)))
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // TODO: Implement Anthropic API call with tools
        Ok(ModelResponse::text(format!("Anthropic response with tools from {}: {}", self.model, prompt)))
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn supports_tools(&self) -> bool {
        true
    }
}

// Local model implementation (placeholder)
pub struct LocalModel {
    _endpoint: String,
    model: String,
}

impl LocalModel {
    pub fn new(endpoint: String, model: String) -> Self {
        Self { _endpoint: endpoint, model }
    }
}

#[async_trait]
impl LanguageModel for LocalModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        // TODO: Implement local model API call
        Ok(ModelResponse::text(format!("Local model response from {}: {}", self.model, prompt)))
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // TODO: Implement local model API call with tools
        Ok(ModelResponse::text(format!("Local model response with tools from {}: {}", self.model, prompt)))
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn supports_tools(&self) -> bool {
        false
    }
}