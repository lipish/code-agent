//! Task-related types for the service API
//!
//! This module contains types for task requests, responses, and execution.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Request to execute a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequest {
    /// Task description or prompt
    pub task: String,
    /// Optional task ID (will be generated if not provided)
    pub task_id: Option<String>,
    /// Execution context
    pub context: Option<TaskContext>,
    /// Priority level
    pub priority: Option<TaskPriority>,
    /// Request metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Task execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContext {
    /// Working directory
    pub working_directory: Option<String>,
    /// Environment variables
    pub environment: Option<HashMap<String, String>>,
    /// Available tools
    pub tools: Option<Vec<String>>,
    /// Execution constraints
    pub constraints: Option<TaskConstraints>,
}

/// Task execution constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConstraints {
    /// Maximum execution time in seconds
    pub max_execution_time: Option<u64>,
    /// Maximum number of steps
    pub max_steps: Option<u32>,
    /// Allowed file paths
    pub allowed_paths: Option<Vec<String>>,
    /// Forbidden operations
    pub forbidden_operations: Option<Vec<String>>,
}

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}

/// Task execution response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    /// Task ID
    pub task_id: String,
    /// Execution status
    pub status: TaskStatus,
    /// Task result (if completed)
    pub result: Option<TaskResult>,
    /// Execution plan that was generated
    pub plan: Option<TaskPlan>,
    /// Execution steps taken
    pub steps: Vec<ExecutionStep>,
    /// Metrics and timing information
    pub metrics: TaskMetrics,
    /// Error information (if failed)
    pub error: Option<ServiceError>,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Task status in service context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

/// Task result from service
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskResult {
    /// Success status
    #[serde(default)]
    pub success: bool,
    /// Result summary
    #[serde(default)]
    pub summary: String,
    /// Detailed result
    #[serde(default)]
    pub details: Option<String>,
    /// Generated files or artifacts
    #[serde(default)]
    pub artifacts: Vec<TaskArtifact>,
    /// Execution time in seconds
    #[serde(default)]
    pub execution_time: u64,
}

/// Task artifact (generated file, output, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskArtifact {
    /// Artifact type
    pub artifact_type: ArtifactType,
    /// Artifact name/path
    pub name: String,
    /// Artifact content or reference
    pub content: Option<String>,
    /// Artifact size in bytes
    pub size: Option<u64>,
    /// Artifact metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Artifact type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactType {
    File,
    Directory,
    Output,
    Log,
    Report,
    Other,
}

/// Task execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPlan {
    /// Task understanding
    pub understanding: String,
    /// Execution approach
    pub approach: String,
    /// Estimated complexity
    pub complexity: TaskComplexity,
    /// Planned steps
    pub steps: Vec<String>,
    /// Required tools
    pub required_tools: Vec<String>,
    /// Estimated time in seconds
    pub estimated_time: Option<u64>,

    // Legacy fields for backward compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_steps: Option<u32>,
    #[serde(default)]
    pub requirements: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

/// Task complexity level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskComplexity {
    Simple,
    Medium,
    Complex,
}

/// Execution step information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// Step number
    pub step_number: u32,
    /// Step type
    pub step_type: StepType,
    /// Step description
    pub description: String,
    /// Step status
    pub status: StepStatus,
    /// Step output
    pub output: Option<String>,
    /// Step error (if failed)
    pub error: Option<String>,
    /// Step start time
    pub started_at: Option<DateTime<Utc>>,
    /// Step completion time
    pub completed_at: Option<DateTime<Utc>>,
    /// Step duration in milliseconds
    pub duration_ms: Option<u64>,

    // Legacy fields for backward compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

/// Step type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    FileRead,
    FileWrite,
    CommandExecution,
    Analysis,
    Planning,
    Execution,    // Added for backward compatibility
    Completion,   // Added for backward compatibility
    Other,
}

/// Step status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Task execution metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskMetrics {
    /// Total execution time in milliseconds
    pub total_time_ms: u64,
    /// Time spent in AI model calls
    pub model_time_ms: u64,
    /// Time spent in tool execution
    pub tool_time_ms: u64,
    /// Number of steps executed
    pub steps_executed: u32,
    /// Number of tool calls
    pub tool_calls: u32,
    /// Number of model calls
    pub model_calls: u32,
    /// Total tokens used (if applicable)
    pub tokens_used: Option<u64>,

    // Legacy fields for backward compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planning_time_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools_used: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_usage_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_usage_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metrics: Option<HashMap<String, serde_json::Value>>,
}

/// Service error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error details
    pub details: Option<String>,
    /// Error timestamp
    pub timestamp: DateTime<Utc>,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
}

