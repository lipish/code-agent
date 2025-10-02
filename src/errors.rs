//! Error types for the AI-Native Code Agent

use thiserror::Error;

/// Main agent error type
#[derive(Debug, Error, Clone)]
pub enum AgentError {
    #[error("Model error: {0}")]
    ModelError(#[from] ModelError),

    #[error("Tool error: {0}")]
    ToolError(#[from] ToolError),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Timeout error")]
    TimeoutError,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

/// Model-related errors
#[derive(Debug, Error, Clone)]
pub enum ModelError {
    #[error("API error: {0}")]
    APIError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Model not supported: {0}")]
    ModelNotSupported(String),

    #[error("Quota exceeded")]
    QuotaExceeded,

    #[error("Network error: {0}")]
    NetworkError(String),
}

/// Tool-related errors
#[derive(Debug, Error, Clone)]
pub enum ToolError {
    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Timeout error")]
    TimeoutError,
}

/// Error handler with retry logic
pub struct ErrorHandler {
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
}

impl ErrorHandler {
    pub fn new(max_retries: u32, retry_delay_seconds: u64) -> Self {
        Self {
            max_retries,
            retry_delay_seconds,
        }
    }

    pub async fn handle_with_retry<F, T, Fut>(&self, operation: F) -> Result<T, AgentError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, AgentError>>,
    {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = Some(error.clone());

                    if attempt < self.max_retries && self.should_retry(&error) {
                        tokio::time::sleep(tokio::time::Duration::from_secs(
                            self.retry_delay_seconds * (attempt as u64 + 1)
                        )).await;
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }

        Err(last_error.unwrap_or(AgentError::UnknownError("Unknown error".to_string())))
    }

    fn should_retry(&self, error: &AgentError) -> bool {
        match error {
            AgentError::NetworkError(_) => true,
            AgentError::TimeoutError => true,
            AgentError::ModelError(ModelError::RateLimited) => true,
            _ => false,
        }
    }
}