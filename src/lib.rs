//! AI-Native Code Agent
//!
//! A minimal, AI-native code assistant that maximizes AI autonomy
//! while providing reliable execution capabilities.
//!
//! ## Features
//!
//! - **AI-Native Design**: Built from the ground up for AI autonomy
//! - **Multiple AI Models**: Support for OpenAI, Anthropic, Zhipu, and local models
//! - **Tool System**: Extensible tool system for file operations, command execution, etc.
//! - **Service Architecture**: Optional HTTP service with REST API (requires "service" feature)
//! - **Metrics & Monitoring**: Comprehensive metrics and health checking (requires "service" feature)

pub mod agent;
pub mod config;
pub mod models;
pub mod tools;
pub mod types;
pub mod errors;
pub mod understanding;
pub mod execution;

// Service modules (optional, enabled with "service" feature)
#[cfg(feature = "service")]
pub mod service_types;

#[cfg(feature = "service")]
pub mod service;

// CLI module (always available but optional for library usage)
pub mod cli;

// Re-export main types and functions for convenience
pub use agent::CodeAgent;
pub use config::AgentConfig;
pub use models::LanguageModel;
pub use tools::Tool;
pub use types::*;
pub use errors::AgentError;

// Service exports (only available with "service" feature)
#[cfg(feature = "service")]
pub use service_types::{
    TaskRequest, TaskResponse, BatchTaskRequest, BatchTaskResponse,
    TaskContext, TaskPriority, TaskStatus, TaskResult,
    ServiceConfig, ServiceStatus, ServiceError,
};

#[cfg(feature = "service")]
pub use service::{
    AiAgentService, AiAgentApi, AiAgentClient, ApiClientBuilder, ServiceResult,
    MetricsSnapshot,
};