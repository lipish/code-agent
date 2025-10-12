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
        // 针对不同类型的任务提供更真实的mock响应
        let response = if prompt.contains("代理商License管理") || prompt.contains("License管理系统") {
            format!(r#"UNDERSTANDING: 需要为软件公司构建一个完整的代理商License管理系统，支持多级代理商架构、License全生命周期管理、权限控制和安全验证机制。
APPROACH: 采用微服务架构设计，使用数据库存储代理商层次结构和License信息，实现RESTful API，集成JWT认证，设计License加密算法，开发Web管理界面和移动端应用。
COMPLEXITY: Complex
REQUIREMENTS: 数据库设计、加密算法、API开发、认证系统、前端界面、移动端开发"#)
        } else if prompt.contains("投资组合") || prompt.contains("portfolio") || prompt.contains("金融") {
            format!(r#"UNDERSTANDING: 构建智能投资组合分析系统，支持多资产类别管理、实时市场数据处理、风险评估和投资策略优化，需要处理大量金融数据并提供实时分析。
APPROACH: 使用大数据架构处理实时市场数据，集成机器学习算法进行预测分析，设计风险管理模块，开发数据可视化界面，实现多语言报告生成系统。
COMPLEXITY: Complex
REQUIREMENTS: 大数据处理、机器学习框架、实时数据流、风险计算模型、数据可视化、多语言支持"#)
        } else if prompt.contains("会议室") || prompt.contains("预定") || prompt.contains("booking") {
            format!(r#"UNDERSTANDING: 开发多分支机构会议室预定管理系统，支持智能预定、冲突检测、审批流程、实时通知和移动端管理，需要处理多地点多用户的复杂业务场景。
APPROACH: 设计分布式架构支持多分支，实现智能调度算法，集成多种通知渠道，开发移动端APP，设计权限管理体系，实现与企业系统集成。
COMPLEXITY: Moderate
REQUIREMENTS: 分布式架构、调度算法、通知系统、移动端开发、权限管理、系统集成"#)
        } else if prompt.contains("简单") || prompt.contains("读取") || prompt.contains("配置") {
            format!(r#"UNDERSTANDING: 执行简单的文件读取和配置处理任务。
APPROACH: 使用标准文件操作库读取配置文件并解析内容。
COMPLEXITY: Simple
REQUIREMENTS: 文件系统访问"#)
        } else {
            format!(r#"UNDERSTANDING: 分析并理解用户提出的任务需求，确定实现方案和技术路线。
APPROACH: 根据任务复杂度选择合适的技术栈和架构模式，制定分步实施计划。
COMPLEXITY: Moderate
REQUIREMENTS: 根据具体需求确定"#)
        };
        
        Ok(ModelResponse::text(response))
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
    /// Note: llm-connector 0.3.1 changed the API.
    /// Now all providers use LlmClient::openai(api_key, Some(endpoint)) or LlmClient::ollama(Some(endpoint))
    fn create_client(config: &ModelConfig) -> Result<LlmClient, ModelError> {
        // Handle providers that don't need API keys first
        match &config.provider {
            ModelProvider::Ollama => {
                // Ollama local server - no API key required
                let endpoint = config.endpoint.as_deref();
                return Ok(LlmClient::ollama(endpoint));
            }
            _ => {}
        }

        // All other providers require API key
        let api_key = config.api_key.as_ref()
            .ok_or_else(|| ModelError::ConfigError("API key required".into()))?;

        match &config.provider {
            ModelProvider::Anthropic => {
                // Anthropic uses its own protocol
                Ok(LlmClient::anthropic(api_key))
            }
            ModelProvider::Xinference => {
                // Xinference local server - OpenAI-compatible
                let endpoint = config.endpoint.as_deref().or(Some("http://localhost:9997/v1"));
                Ok(LlmClient::openai(api_key, endpoint))
            }
            ModelProvider::Local(endpoint) => {
                // Generic local server - assume OpenAI-compatible
                Ok(LlmClient::openai(api_key, Some(endpoint)))
            }
            ModelProvider::OpenAI => {
                // Standard OpenAI client
                let endpoint = config.endpoint.as_deref().or(Some("https://api.openai.com/v1"));
                Ok(LlmClient::openai(api_key, endpoint))
            }
            ModelProvider::Zhipu => {
                // Zhipu AI (智谱AI) - OpenAI-compatible
                let endpoint = config.endpoint.as_deref().or(Some("https://open.bigmodel.cn/api/paas/v4"));
                Ok(LlmClient::openai(api_key, endpoint))
            }
            ModelProvider::DeepSeek => {
                // DeepSeek - OpenAI-compatible 
                let endpoint = config.endpoint.as_deref().or(Some("https://api.deepseek.com/v1"));
                Ok(LlmClient::openai(api_key, endpoint))
            }
            ModelProvider::Moonshot => {
                // Moonshot AI - OpenAI-compatible
                let endpoint = config.endpoint.as_deref().or(Some("https://api.moonshot.cn/v1"));
                Ok(LlmClient::openai(api_key, endpoint))
            }
            ModelProvider::Aliyun => {
                // Aliyun DashScope - OpenAI-compatible mode
                let endpoint = config.endpoint.as_deref().or(Some("https://dashscope.aliyuncs.com/compatible-mode/v1"));
                Ok(LlmClient::openai(api_key, endpoint))
            }
            ModelProvider::LongCat => {
                // LongCat - OpenAI-compatible
                let endpoint = config.endpoint.as_deref().or(Some("https://api.longcat.chat/openai"));
                Ok(LlmClient::openai(api_key, endpoint))
            }
            ModelProvider::VolcEngine => {
                // VolcEngine - OpenAI-compatible (requires valid endpoint ID)
                // Note: The endpoint ID must be created and enabled in VolcEngine console
                let endpoint = config.endpoint.as_deref().or(Some("https://ark.cn-beijing.volces.com/api/v3"));
                Ok(LlmClient::openai(api_key, endpoint))
            }
            ModelProvider::Ollama => {
                // This should be unreachable as Ollama is handled above
                unreachable!("Ollama should be handled before API key check")
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
            ModelProvider::OpenAI | ModelProvider::Anthropic | ModelProvider::Zhipu |
            ModelProvider::DeepSeek | ModelProvider::Moonshot | ModelProvider::Aliyun |
            ModelProvider::Xinference // Xinference supports tools if the underlying model does
        )
    }
}

