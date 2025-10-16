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

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

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

    #[error("Configuration error: {0}")]
    ConfigError(String),

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

    #[error("File operation error: {0}")]
    FileOperation(#[from] FileOperationError),

    #[error("Command operation error: {0}")]
    CommandOperation(#[from] CommandOperationError),
}

/// File operation specific errors
#[derive(Debug, Error, Clone)]
pub enum FileOperationError {
    #[error("File not found: {path}")]
    NotFound { path: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("File too large: {size} bytes, max: {max_size} bytes")]
    FileTooLarge { size: u64, max_size: u64 },

    #[error("Invalid file path: {path}")]
    InvalidPath { path: String },

    #[error("Directory not found: {path}")]
    DirectoryNotFound { path: String },

    #[error("IO error for '{path}': {message}")]
    IoError { path: String, message: String },

    #[error("File already exists: {path}")]
    AlreadyExists { path: String },
}

/// Command operation specific errors
#[derive(Debug, Error, Clone)]
pub enum CommandOperationError {
    #[error("Command not found: {command}")]
    CommandNotFound { command: String },

    #[error("Command failed with exit code {code}: {stderr}")]
    ExecutionFailed { code: i32, stderr: String },

    #[error("Command timeout after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("Invalid command: {command}")]
    InvalidCommand { command: String },

    #[error("Permission denied for command: {command}")]
    PermissionDenied { command: String },

    #[error("Command output too large: {size} bytes, max: {max_size} bytes")]
    OutputTooLarge { size: usize, max_size: usize },

    #[error("IO error executing command '{command}': {message}")]
    IoError { command: String, message: String },

    #[error("Security error: {0}")]
    Security(#[from] SecurityError),
}

/// Security-related errors
#[derive(Debug, Error, Clone)]
pub enum SecurityError {
    #[error("Empty command")]
    EmptyCommand,

    #[error("Unauthorized command: {command}")]
    UnauthorizedCommand { command: String },

    #[error("Command contains dangerous patterns: {pattern}")]
    DangerousPattern { pattern: String },

    #[error("Path traversal attempt detected: {path}")]
    PathTraversal { path: String },

    #[error("Resource limit exceeded: {resource}")]
    ResourceLimitExceeded { resource: String },

    #[error("Suspicious command arguments: {args}")]
    SuspiciousArguments { args: String },
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
        matches!(
            error,
            AgentError::NetworkError(_)
            | AgentError::TimeoutError
            | AgentError::ModelError(ModelError::RateLimited)
        )
    }
}