//! Language model abstraction and implementations
//!
//! This module provides a unified interface for different LLM providers using llm-connector.
//!
//! # Design Philosophy
//!
//! Instead of creating separate Model structs for each provider (OpenAI, Zhipu, Anthropic),
//! we use a single `LlmModel` that wraps `llm-connector::LlmClient`. This eliminates code
//! duplication and leverages llm-connector's unified interface.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::ModelError;
use crate::config::{ModelConfig, ModelProvider};
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

/// Unified LLM model implementation using llm-connector
///
/// This single struct handles all LLM providers (OpenAI, Anthropic, Zhipu, Local)
/// by wrapping llm-connector's unified client interface.
pub struct LlmModel {
    client: LlmClient,
    config: ModelConfig,
}

impl LlmModel {
    /// Create a new LlmModel from configuration
    pub fn from_config(config: ModelConfig) -> Result<Self, ModelError> {
        let client = Self::create_client(&config)?;
        Ok(Self { client, config })
    }

    /// Create llm-connector client based on provider
    ///
    /// Note: llm-connector is protocol-based, not provider-based.
    /// Most providers (Zhipu, DeepSeek, Moonshot, etc.) use OpenAI-compatible protocol.
    ///
    /// For custom endpoints, set OPENAI_API_BASE environment variable before creating client.
    fn create_client(config: &ModelConfig) -> Result<LlmClient, ModelError> {
        let api_key = config.api_key.as_ref()
            .ok_or_else(|| ModelError::ConfigError("API key required".into()))?;

        // Set custom endpoint via environment variable if provided
        // This is the standard way to use custom OpenAI-compatible endpoints
        if let Some(endpoint) = &config.endpoint {
            std::env::set_var("OPENAI_API_BASE", endpoint);
        }

        match &config.provider {
            ModelProvider::Anthropic => {
                // Anthropic uses its own protocol
                Ok(LlmClient::anthropic(api_key))
            }
            ModelProvider::Local(endpoint) => {
                // Local Ollama
                Ok(LlmClient::ollama_at(endpoint))
            }
            // All OpenAI-compatible providers (OpenAI, Zhipu, DeepSeek, Moonshot, etc.)
            _ => {
                // Use OpenAI protocol (works with any OpenAI-compatible API)
                Ok(LlmClient::openai(api_key))
            }
        }
    }

    /// Format model name for llm-connector
    ///
    /// llm-connector uses the model name directly, not with provider prefix.
    /// The protocol is determined by the client type, not the model name.
    fn format_model_name(&self) -> String {
        self.config.model_name.clone()
    }

    /// Convert llm-connector response to our ModelResponse
    fn convert_response(response: llm_connector::ChatResponse) -> Result<ModelResponse, ModelError> {
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
}

#[async_trait]
impl LanguageModel for LlmModel {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError> {
        let request = ChatRequest {
            model: self.format_model_name(),
            messages: vec![LlmMessage::user(prompt)],
            ..Default::default()
        };

        let response = self.client.chat(&request)
            .await
            .map_err(|e| ModelError::APIError(e.to_string()))?;

        Self::convert_response(response)
    }

    async fn complete_with_tools(&self, prompt: &str, _tools: &[ToolDefinition]) -> Result<ModelResponse, ModelError> {
        // TODO: Implement tool calling support
        self.complete(prompt).await
    }

    fn model_name(&self) -> &str {
        &self.config.model_name
    }

    fn supports_tools(&self) -> bool {
        // Most modern LLMs support tools
        matches!(
            self.config.provider,
            ModelProvider::OpenAI | ModelProvider::Anthropic | ModelProvider::Zhipu
        )
    }
}

