//! Common utilities for test files

use task_runner::config::{ModelConfig, ModelProvider};

/// Get test model configuration with secure API key handling
pub fn get_test_zhipu_config() -> ModelConfig {
    ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some(
            std::env::var("ZHIPU_API_KEY")
                .unwrap_or_else(|_| "your-api-key-here".to_string())
        ),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 4000,
        temperature: 0.6,
    }
}