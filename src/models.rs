//! Language model abstraction and implementations
//!
//! This module now uses llm-connector for unified LLM provider access

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::ModelError;
use llm_connector::{LlmClient, ChatRequest, Message as LlmMessage};

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

// OpenAI model implementation using llm-connector
pub struct OpenAIModel {
    client: LlmClient,
    model: String,
}

impl OpenAIModel {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: LlmClient::openai(&api_key),
            model
        }
    }
}

#[async_trait]
impl LanguageModel for OpenAIModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        let request = ChatRequest {
            model: format!("openai/{}", self.model),
            messages: vec![LlmMessage::user(prompt)],
            ..Default::default()
        };

        let response = self.client.chat(&request)
            .await
            .map_err(|e| ModelError::APIError(e.to_string()))?;

        let content = response.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let usage = response.usage.map(|u| TokenUsage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });

        Ok(ModelResponse {
            content,
            tool_calls: vec![],
            usage,
            metadata: HashMap::new(),
        })
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // TODO: Implement tool calling support
        self.complete(prompt).await
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn supports_tools(&self) -> bool {
        true
    }
}

// Zhipu model implementation using llm-connector
pub struct ZhipuModel {
    client: LlmClient,
    model: String,
}

impl ZhipuModel {
    pub fn new(api_key: String, model: String, _endpoint: Option<String>) -> Self {
        // Note: llm-connector 0.2.2 doesn't have a dedicated zhipu method
        // Zhipu uses OpenAI-compatible API, so we use openai method with custom endpoint
        Self {
            client: LlmClient::openai(&api_key),
            model
        }
    }
}

#[async_trait]
impl LanguageModel for ZhipuModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        let request = ChatRequest {
            model: format!("zhipu/{}", self.model),
            messages: vec![LlmMessage::user(prompt)],
            ..Default::default()
        };

        let response = self.client.chat(&request)
            .await
            .map_err(|e| ModelError::APIError(e.to_string()))?;

        let content = response.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let usage = response.usage.map(|u| TokenUsage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });

        Ok(ModelResponse {
            content,
            tool_calls: vec![],
            usage,
            metadata: HashMap::new(),
        })
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // TODO: Implement tool calling support
        self.complete(prompt).await
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn supports_tools(&self) -> bool {
        true
    }
}

// Anthropic model implementation using llm-connector
pub struct AnthropicModel {
    client: LlmClient,
    model: String,
}

impl AnthropicModel {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: LlmClient::anthropic(&api_key),
            model
        }
    }
}

#[async_trait]
impl LanguageModel for AnthropicModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        let request = ChatRequest {
            model: format!("anthropic/{}", self.model),
            messages: vec![LlmMessage::user(prompt)],
            ..Default::default()
        };

        let response = self.client.chat(&request)
            .await
            .map_err(|e| ModelError::APIError(e.to_string()))?;

        let content = response.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let usage = response.usage.map(|u| TokenUsage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });

        Ok(ModelResponse {
            content,
            tool_calls: vec![],
            usage,
            metadata: HashMap::new(),
        })
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // TODO: Implement tool calling support
        self.complete(prompt).await
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn supports_tools(&self) -> bool {
        true
    }
}

// Local model implementation using llm-connector
pub struct LocalModel {
    client: LlmClient,
    model: String,
}

impl LocalModel {
    pub fn new(endpoint: String, model: String) -> Self {
        // Use ollama_at for custom local endpoints
        Self {
            client: LlmClient::ollama_at(&endpoint),
            model
        }
    }
}

#[async_trait]
impl LanguageModel for LocalModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![LlmMessage::user(prompt)],
            ..Default::default()
        };

        let response = self.client.chat(&request)
            .await
            .map_err(|e| ModelError::APIError(e.to_string()))?;

        let content = response.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let usage = response.usage.map(|u| TokenUsage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });

        Ok(ModelResponse {
            content,
            tool_calls: vec![],
            usage,
            metadata: HashMap::new(),
        })
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // Local models typically don't support tool calling
        self.complete(prompt).await
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn supports_tools(&self) -> bool {
        false
    }
}