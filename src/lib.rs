//! AI-Native Code Agent
//!
//! A minimal, AI-native code assistant that maximizes AI autonomy
//! while providing reliable execution capabilities.
//!
//! ## Features
//!
//! - **AI-Native Design**: Built from the ground up for AI autonomy
//! - **Service Architecture**: Can run as both CLI tool and HTTP service
//! - **Multiple AI Models**: Support for OpenAI, Anthropic, Zhipu, and local models
//! - **Tool System**: Extensible tool system for file operations, command execution, etc.
//! - **REST API**: Standard HTTP interface for integration with other applications
//! - **Rust API**: Direct Rust API for in-process usage
//! - **Real-time Updates**: WebSocket support for task progress monitoring
//! - **Metrics & Monitoring**: Comprehensive metrics and health checking

pub mod agent;
pub mod config;
pub mod models;
pub mod tools;
pub mod cli;
pub mod types;
pub mod errors;
pub mod understanding;
pub mod execution;
pub mod service_types;
pub mod service;

// Re-export main types and functions for convenience
pub use agent::CodeAgent;
pub use config::AgentConfig;
pub use models::LanguageModel;
pub use tools::Tool;
pub use types::*;
pub use errors::AgentError;

// Service exports
pub use service_types::{
    TaskRequest, TaskResponse, BatchTaskRequest, BatchTaskResponse,
    TaskContext, TaskPriority, TaskStatus, TaskResult,
    ServiceConfig, ServiceStatus, ServiceError,
};
pub use service::{
    AiAgentService, AiAgentApi, AiAgentClient, ApiClientBuilder, ServiceResult,
    MetricsSnapshot,
};