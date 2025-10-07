//! Batch processing types for the service API
//!
//! This module contains types for batch task execution.

use serde::{Deserialize, Serialize};
use super::task::{TaskRequest, TaskResponse};

/// Batch task request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTaskRequest {
    /// List of tasks to execute
    pub tasks: Vec<TaskRequest>,
    /// Execution mode
    pub mode: BatchExecutionMode,
    /// Batch metadata
    pub metadata: Option<serde_json::Value>,

    // Legacy field for backward compatibility
    #[serde(default)]
    pub continue_on_error: bool,
}

/// Batch execution mode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BatchExecutionMode {
    /// Execute tasks sequentially
    Sequential,
    /// Execute tasks in parallel
    Parallel,
}

/// Batch task response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTaskResponse {
    /// Batch ID
    pub batch_id: String,
    /// Individual task responses
    pub results: Vec<TaskResponse>,
    /// Batch statistics
    pub statistics: BatchStatistics,

    // Legacy field for backward compatibility
    #[serde(alias = "responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<TaskResponse>>,
}

/// Batch execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStatistics {
    /// Total number of tasks
    pub total_tasks: usize,
    /// Number of successful tasks
    pub successful_tasks: usize,
    /// Number of failed tasks
    pub failed_tasks: usize,
    /// Total execution time in milliseconds
    pub total_time_ms: u64,
    /// Average time per task in milliseconds
    pub average_time_ms: u64,

    // Legacy fields for backward compatibility
    #[serde(alias = "completed_tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_tasks: Option<usize>,
    #[serde(alias = "total_execution_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_time: Option<u64>,
    #[serde(alias = "average_execution_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_execution_time: Option<u64>,
}

