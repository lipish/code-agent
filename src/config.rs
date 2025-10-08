//! Configuration management for the AI-Native Code Agent

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Main agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub model: ModelConfig,
    pub execution: ExecutionConfig,
    pub safety: SafetyConfig,
    pub tools: ToolConfig,
    pub logging: LoggingConfig,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: ModelProvider,
    pub model_name: String,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Model provider enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelProvider {
    OpenAI,
    Anthropic,
    Zhipu,
    DeepSeek,
    Moonshot,
    Local(String),
}

/// Execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub max_steps: u32,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
}

/// Safety configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    pub enable_safety_checks: bool,
    pub allowed_directories: Vec<String>,
    pub blocked_commands: Vec<String>,
}

/// Tools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub auto_discovery: bool,
    pub custom_tools_path: Option<String>,
    pub enabled_tools: Vec<String>,
    pub disabled_tools: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
    pub console: bool,
    pub format: LogFormat,
}

/// Log format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Pretty,
    Json,
    Compact,
}

impl AgentConfig {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let mut config: AgentConfig = toml::from_str(&content)?;

        // Process environment variable substitutions
        if let Some(ref api_key) = config.model.api_key {
            if api_key.starts_with("${") && api_key.ends_with("}") {
                let env_var = &api_key[2..api_key.len()-1];
                config.model.api_key = std::env::var(env_var).ok();
            }
        }

        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let provider = if std::env::var("DEEPSEEK_API_KEY").is_ok() {
            ModelProvider::DeepSeek
        } else if std::env::var("MOONSHOT_API_KEY").is_ok() {
            ModelProvider::Moonshot
        } else if std::env::var("ZHIPU_API_KEY").is_ok() {
            ModelProvider::Zhipu
        } else if std::env::var("ANTHROPIC_API_KEY").is_ok() {
            ModelProvider::Anthropic
        } else {
            ModelProvider::OpenAI
        };

        let model_name = match provider {
            ModelProvider::DeepSeek => "deepseek-chat",
            ModelProvider::Moonshot => "moonshot-v1-8k",
            ModelProvider::Zhipu => "GLM-4.6",
            ModelProvider::Anthropic => "claude-3-sonnet-20240229",
            _ => "gpt-4-turbo-preview",
        };

        let api_key = match provider {
            ModelProvider::DeepSeek => std::env::var("DEEPSEEK_API_KEY").ok(),
            ModelProvider::Moonshot => std::env::var("MOONSHOT_API_KEY").ok(),
            ModelProvider::Zhipu => std::env::var("ZHIPU_API_KEY").ok(),
            ModelProvider::Anthropic => std::env::var("ANTHROPIC_API_KEY").ok(),
            ModelProvider::OpenAI => std::env::var("OPENAI_API_KEY").ok(),
            ModelProvider::Local(_) => std::env::var("API_KEY").ok(),
        };

        let endpoint = match provider {
            ModelProvider::DeepSeek => Some("https://api.deepseek.com".to_string()),
            ModelProvider::Moonshot => Some("https://api.moonshot.cn/v1".to_string()),
            ModelProvider::Zhipu => Some("https://open.bigmodel.cn/api/paas/v4/".to_string()),
            _ => std::env::var("MODEL_ENDPOINT").ok(),
        };

        Ok(AgentConfig {
            model: ModelConfig {
                provider,
                model_name: model_name.to_string(),
                api_key,
                endpoint,
                max_tokens: 4000,
                temperature: 0.7,
            },
            execution: ExecutionConfig {
                max_steps: 50,
                timeout_seconds: 300,
                max_retries: 3,
                retry_delay_seconds: 2,
            },
            safety: SafetyConfig {
                enable_safety_checks: true,
                allowed_directories: vec![".".to_string(), "/tmp".to_string()],
                blocked_commands: vec![
                    "rm -rf /".to_string(),
                    "format".to_string(),
                    "fdisk".to_string(),
                    "dd if=".to_string(),
                ],
            },
            tools: ToolConfig {
                auto_discovery: true,
                custom_tools_path: None,
                enabled_tools: vec![
                    "read_file".to_string(),
                    "write_file".to_string(),
                    "run_command".to_string(),
                    "list_files".to_string(),
                ],
                disabled_tools: vec![],
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: Some("agent.log".to_string()),
                console: true,
                format: LogFormat::Pretty,
            },
        })
    }

    /// Load with fallback: try file first, then environment
    pub fn load_with_fallback<P: AsRef<Path>>(config_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        match Self::from_file(&config_path) {
            Ok(config) => Ok(config),
            Err(_) => {
                tracing::warn!("Failed to load config file {:?}, using environment", config_path.as_ref());
                Self::from_env()
            }
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            model: ModelConfig {
                provider: ModelProvider::OpenAI,
                model_name: "gpt-3.5-turbo".to_string(),
                api_key: None,
                endpoint: None,
                max_tokens: 4000,
                temperature: 0.7,
            },
            execution: ExecutionConfig {
                max_steps: 50,
                timeout_seconds: 300,
                max_retries: 3,
                retry_delay_seconds: 2,
            },
            safety: SafetyConfig {
                enable_safety_checks: true,
                allowed_directories: vec![".".to_string()],
                blocked_commands: vec![],
            },
            tools: ToolConfig {
                auto_discovery: true,
                custom_tools_path: None,
                enabled_tools: vec![],
                disabled_tools: vec![],
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
                console: true,
                format: LogFormat::Pretty,
            },
        }
    }
}