//! AI Agent Service API and Types

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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Normal
    }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(PartialEq, Eq)]
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
    /// Artifact content (for small artifacts)
    pub content: Option<String>,
    /// Artifact size in bytes
    pub size: Option<u64>,
    /// Artifact metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Artifact types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactType {
    File,
    Text,
    Json,
    Image,
    Log,
    Report,
    Other(String),
}

/// Task execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPlan {
    /// Understanding of the task
    pub understanding: String,
    /// Approach to solve the task
    pub approach: String,
    /// Estimated complexity
    pub complexity: TaskComplexity,
    /// Estimated number of steps
    pub estimated_steps: u32,
    /// Required tools or resources
    pub requirements: Vec<String>,
    /// Plan created timestamp
    pub created_at: DateTime<Utc>,
}

/// Task complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskComplexity {
    Simple,
    Moderate,
    Complex,
}

/// Individual execution step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// Step number
    pub step_number: u32,
    /// Step type
    pub step_type: StepType,
    /// Step description
    pub description: String,
    /// Input data for the step
    pub input: Option<serde_json::Value>,
    /// Output data from the step
    pub output: Option<serde_json::Value>,
    /// Step status
    pub status: StepStatus,
    /// Error if step failed
    pub error: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Step types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepType {
    Analysis,
    Planning,
    ToolUse,
    Execution,
    Verification,
    Completion,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetrics {
    /// Total execution time in seconds
    pub total_execution_time: u64,
    /// Time spent in planning phase
    pub planning_time_ms: u64,
    /// Time spent in execution phase
    pub execution_time_ms: u64,
    /// Number of steps executed
    pub steps_executed: u32,
    /// Number of tools used
    pub tools_used: u32,
    /// Memory usage metrics
    pub memory_usage_mb: Option<f64>,
    /// CPU usage percentage
    pub cpu_usage_percent: Option<f64>,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
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
    /// Error stack trace (for debugging)
    pub stack_trace: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;
        if let Some(details) = &self.details {
            write!(f, ": {}", details)?;
        }
        Ok(())
    }
}

impl std::error::Error for ServiceError {}

/// Service status and health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Service name
    pub name: String,
    /// Service version
    pub version: String,
    /// Service status
    pub status: ServiceHealth,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Number of active tasks
    pub active_tasks: u32,
    /// Number of completed tasks
    pub completed_tasks: u64,
    /// Number of failed tasks
    pub failed_tasks: u64,
    /// Available tools
    pub available_tools: Vec<String>,
    /// System metrics
    pub system_metrics: SystemMetrics,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in MB
    pub memory_usage_mb: f64,
    /// Disk usage in MB
    pub disk_usage_mb: f64,
    /// Network I/O
    pub network_io: NetworkMetrics,
}

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Bytes received
    pub bytes_received: u64,
    /// Bytes sent
    pub bytes_sent: u64,
    /// Active connections
    pub active_connections: u32,
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            bytes_received: 0,
            bytes_sent: 0,
            active_connections: 0,
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            disk_usage_mb: 0.0,
            network_io: NetworkMetrics::default(),
        }
    }
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: u32,
    /// Default task timeout in seconds
    pub default_task_timeout: u64,
    /// Maximum task timeout in seconds
    pub max_task_timeout: u64,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Log level
    pub log_level: String,
    /// CORS settings
    pub cors: CorsConfig,
    /// Rate limiting
    pub rate_limiting: Option<RateLimitConfig>,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Allow credentials
    pub allow_credentials: bool,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
    /// Cleanup interval in seconds
    pub cleanup_interval_seconds: u64,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 10,
            default_task_timeout: 300, // 5 minutes
            max_task_timeout: 3600,    // 1 hour
            enable_metrics: true,
            log_level: "info".to_string(),
            cors: CorsConfig {
                allowed_origins: vec!["*".to_string()],
                allowed_methods: vec!["GET".to_string(), "POST".to_string(), "DELETE".to_string()],
                allowed_headers: vec!["*".to_string()],
                allow_credentials: false,
            },
            rate_limiting: Some(RateLimitConfig {
                requests_per_minute: 60,
                burst_size: 10,
                cleanup_interval_seconds: 300,
            }),
        }
    }
}

/// Batch task request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTaskRequest {
    /// List of tasks to execute
    pub tasks: Vec<TaskRequest>,
    /// Execution mode
    pub mode: BatchExecutionMode,
    /// Continue on error
    pub continue_on_error: bool,
}

/// Batch execution modes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BatchExecutionMode {
    Sequential,
    Parallel,
}

/// Batch task response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTaskResponse {
    /// Batch ID
    pub batch_id: String,
    /// Individual task responses
    pub responses: Vec<TaskResponse>,
    /// Batch statistics
    pub statistics: BatchStatistics,
}

/// Batch execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStatistics {
    /// Total tasks
    pub total_tasks: u32,
    /// Completed tasks
    pub completed_tasks: u32,
    /// Failed tasks
    pub failed_tasks: u32,
    /// Total execution time in seconds
    pub total_execution_time: u64,
    /// Average execution time per task
    pub average_execution_time: f64,
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    /// Task status update
    TaskUpdate { task_id: String, status: TaskStatus },
    /// Task progress update
    TaskProgress { task_id: String, progress: f64, message: Option<String> },
    /// Task step update
    StepUpdate { task_id: String, step: ExecutionStep },
    /// Task completed
    TaskCompleted { task_id: String, result: TaskResult },
    /// Task failed
    TaskFailed { task_id: String, error: ServiceError },
    /// Service status update
    ServiceStatus { status: ServiceStatus },
    /// Heartbeat
    Heartbeat { timestamp: DateTime<Utc> },
}