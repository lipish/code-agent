//! Service-specific error types

use thiserror::Error;
use crate::service_types::{ServiceError, TaskStatus};
use chrono::Utc;
use uuid::Uuid;

/// Service result type
pub type ServiceResult<T> = Result<T, ServiceError>;

/// Service error types
#[derive(Debug, Error, Clone)]
pub enum ServiceErrorType {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Task execution failed: {0}")]
    TaskExecutionFailed(String),

    #[error("Task timeout: {0}")]
    TaskTimeout(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Model error: {0}")]
    ModelError(String),

    #[error("Tool error: {0}")]
    ToolError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),
}

impl ServiceErrorType {
    /// Convert to ServiceError with timestamp
    pub fn to_service_error(self) -> ServiceError {
        ServiceError {
            code: self.error_code(),
            message: self.to_string(),
            details: None,
            stack_trace: None,
            timestamp: Utc::now(),
        }
    }

    /// Get error code
    pub fn error_code(&self) -> String {
        match self {
            ServiceErrorType::InvalidRequest(_) => "INVALID_REQUEST".to_string(),
            ServiceErrorType::TaskNotFound(_) => "TASK_NOT_FOUND".to_string(),
            ServiceErrorType::TaskExecutionFailed(_) => "TASK_EXECUTION_FAILED".to_string(),
            ServiceErrorType::TaskTimeout(_) => "TASK_TIMEOUT".to_string(),
            ServiceErrorType::ConfigurationError(_) => "CONFIGURATION_ERROR".to_string(),
            ServiceErrorType::ModelError(_) => "MODEL_ERROR".to_string(),
            ServiceErrorType::ToolError(_) => "TOOL_ERROR".to_string(),
            ServiceErrorType::RateLimitExceeded => "RATE_LIMIT_EXCEEDED".to_string(),
            ServiceErrorType::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE".to_string(),
            ServiceErrorType::InternalServerError(_) => "INTERNAL_SERVER_ERROR".to_string(),
            ServiceErrorType::AuthenticationError(_) => "AUTHENTICATION_ERROR".to_string(),
            ServiceErrorType::AuthorizationError(_) => "AUTHORIZATION_ERROR".to_string(),
        }
    }
}

/// From implementations for converting from other error types
impl From<crate::errors::AgentError> for ServiceErrorType {
    fn from(error: crate::errors::AgentError) -> Self {
        match error {
            crate::errors::AgentError::ModelError(e) => ServiceErrorType::ModelError(e.to_string()),
            crate::errors::AgentError::ToolError(e) => ServiceErrorType::ToolError(e.to_string()),
            crate::errors::AgentError::NetworkError(e) => ServiceErrorType::ServiceUnavailable(e),
            crate::errors::AgentError::TimeoutError => ServiceErrorType::TaskTimeout("Task execution timeout".to_string()),
            crate::errors::AgentError::ConfigError(e) => ServiceErrorType::ConfigurationError(e),
            crate::errors::AgentError::UnknownError(e) => ServiceErrorType::InternalError(e),
        }
    }
}

impl From<serde_json::Error> for ServiceErrorType {
    fn from(error: serde_json::Error) -> Self {
        ServiceErrorType::InvalidRequest(format!("JSON serialization error: {}", error))
    }
}

impl From<tokio::task::JoinError> for ServiceErrorType {
    fn from(error: tokio::task::JoinError) -> Self {
        ServiceErrorType::InternalServerError(format!("Task join error: {}", error))
    }
}

impl From<std::io::Error> for ServiceErrorType {
    fn from(error: std::io::Error) -> Self {
        ServiceErrorType::InternalServerError(format!("IO error: {}", error))
    }
}

/// HTTP status code mapping
impl ServiceError {
    /// Get HTTP status code for this error
    pub fn http_status_code(&self) -> u16 {
        match self.code.as_str() {
            "INVALID_REQUEST" => 400,
            "AUTHENTICATION_ERROR" => 401,
            "AUTHORIZATION_ERROR" => 403,
            "TASK_NOT_FOUND" => 404,
            "RATE_LIMIT_EXCEEDED" => 429,
            "TASK_TIMEOUT" => 408,
            "SERVICE_UNAVAILABLE" => 503,
            "CONFIGURATION_ERROR" => 500,
            "MODEL_ERROR" => 502,
            "TOOL_ERROR" => 502,
            "INTERNAL_SERVER_ERROR" => 500,
            _ => 500,
        }
    }

    /// Check if this is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        self.http_status_code() >= 400 && self.http_status_code() < 500
    }

    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        self.http_status_code() >= 500
    }
}

/// Error builder for creating service errors
pub struct ErrorBuilder;

impl ErrorBuilder {
    /// Create an invalid request error
    pub fn invalid_request(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::InvalidRequest(message.into()).to_service_error()
    }

    /// Create a task not found error
    pub fn task_not_found(task_id: impl Into<String>) -> ServiceError {
        ServiceErrorType::TaskNotFound(task_id.into()).to_service_error()
    }

    /// Create a task execution failed error
    pub fn task_execution_failed(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::TaskExecutionFailed(message.into()).to_service_error()
    }

    /// Create a task timeout error
    pub fn task_timeout(task_id: impl Into<String>) -> ServiceError {
        ServiceErrorType::TaskTimeout(task_id.into()).to_service_error()
    }

    /// Create a configuration error
    pub fn configuration_error(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::ConfigurationError(message.into()).to_service_error()
    }

    /// Create a model error
    pub fn model_error(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::ModelError(message.into()).to_service_error()
    }

    /// Create a tool error
    pub fn tool_error(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::ToolError(message.into()).to_service_error()
    }

    /// Create a rate limit exceeded error
    pub fn rate_limit_exceeded() -> ServiceError {
        ServiceErrorType::RateLimitExceeded.to_service_error()
    }

    /// Create a service unavailable error
    pub fn service_unavailable(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::ServiceUnavailable(message.into()).to_service_error()
    }

    /// Create an internal server error
    pub fn internal_server_error(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::InternalServerError(message.into()).to_service_error()
    }

    /// Create an authentication error
    pub fn authentication_error(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::AuthenticationError(message.into()).to_service_error()
    }

    /// Create an authorization error
    pub fn authorization_error(message: impl Into<String>) -> ServiceError {
        ServiceErrorType::AuthorizationError(message.into()).to_service_error()
    }
}

/// Result extension trait for convenient error handling
pub trait ResultExt<T> {
    /// Convert Result to ServiceResult
    fn to_service_result(self) -> ServiceResult<T>;

    /// Map error to ServiceError with context
    fn with_context(self, context: impl Into<String>) -> ServiceResult<T>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<ServiceErrorType>,
{
    fn to_service_result(self) -> ServiceResult<T> {
        self.map_err(|e| e.into().to_service_error())
    }

    fn with_context(self, context: impl Into<String>) -> ServiceResult<T> {
        self.map_err(|e| {
            let service_error = e.into().to_service_error();
            ServiceError {
                details: Some(context.into()),
                ..service_error
            }
        })
    }
}