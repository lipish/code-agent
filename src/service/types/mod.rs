//! Service API Types
//!
//! This module provides well-organized types for the AI Agent Service API.
//!
//! # Module Organization
//!
//! - `task` - Task-related types (requests, responses, execution)
//! - `batch` - Batch processing types
//! - `service` - Service configuration and status types
//! - `websocket` - WebSocket message types
//!
//! # Examples
//!
//! ```rust
//! use task_runner::service::types::task::{TaskRequest, TaskPriority};
//! use task_runner::service::types::service::ServiceConfig;
//!
//! // Create a task request
//! let request = TaskRequest {
//!     task: "List files in current directory".to_string(),
//!     task_id: None,
//!     context: None,
//!     priority: Some(TaskPriority::Normal),
//!     metadata: None,
//! };
//!
//! // Use default service configuration
//! let config = ServiceConfig::default();
//! ```

pub mod task;
pub mod batch;
pub mod service;
pub mod websocket;

// Re-export commonly used types for convenience
pub use task::{
    TaskRequest, TaskResponse, TaskStatus, TaskResult, TaskPriority,
    TaskContext, TaskConstraints,
    ExecutionStep, StepType, StepStatus, TaskMetrics,
    TaskArtifact, ArtifactType, ServiceError,
};

// Re-export unified types from core
pub use crate::types::{TaskPlan, TaskComplexity};

pub use batch::{
    BatchTaskRequest, BatchTaskResponse, BatchExecutionMode, BatchStatistics,
};

pub use service::{
    ServiceConfig, ServiceStatus, ServiceHealth,
    SystemMetrics, NetworkMetrics,
    CorsConfig, RateLimitConfig,
};

pub use websocket::WebSocketMessage;

