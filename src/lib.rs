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

// Core modules
pub mod agent;          // Task agent (now a directory module)
pub mod config;         // Configuration management
pub mod errors;         // Error types and handling
pub mod models;         // AI model interfaces
pub mod prompts;        // Prompt engineering system
pub mod security;       // Security features (NEW)
pub mod tools;          // Tool system
pub mod types;          // Core type definitions

// Execution modules
pub mod execution;      // Execution operations (file, command)
pub mod planning;       // Task planning and analysis (formerly understanding)

// Helper modules
pub mod text_parser;   // Task helper functions

// Service modules (optional, enabled with "service" feature)
#[cfg(feature = "service")]
pub mod service;

// CLI module (always available but optional for library usage)
pub mod cli;

// Re-export main types and functions for convenience
pub use agent::TaskAgent;

// Re-export deprecated alias (allow deprecated warning for the export itself)
#[allow(deprecated)]
pub use agent::CodeAgent; // Deprecated: Use TaskAgent instead

pub use config::AgentConfig;
pub use models::LanguageModel;
pub use tools::Tool;
pub use types::*;
pub use errors::AgentError;

// Service exports (only available with "service" feature)
#[cfg(feature = "service")]
pub use service::types::{
    // Task types
    TaskRequest, TaskResponse, TaskStatus, TaskResult, TaskPriority,
    TaskContext, TaskConstraints, TaskPlan, TaskComplexity,
    ExecutionStep, StepType, StepStatus, TaskMetrics,
    TaskArtifact, ArtifactType, ServiceError,
    // Batch types
    BatchTaskRequest, BatchTaskResponse, BatchExecutionMode, BatchStatistics,
    // Service types
    ServiceConfig, ServiceStatus, ServiceHealth,
    SystemMetrics, NetworkMetrics,
    // WebSocket types
    WebSocketMessage,
};

#[cfg(feature = "service")]
pub use service::{
    CodeAgentService, CodeAgentApi, CodeAgentClient, ApiClientBuilder, ServiceResult,
    MetricsSnapshot,
};