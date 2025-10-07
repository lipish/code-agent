//! WebSocket message types
//!
//! This module contains types for WebSocket communication.

use serde::{Deserialize, Serialize};
use super::task::{TaskResponse, ExecutionStep};

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebSocketMessage {
    /// Task started notification
    TaskStarted {
        task_id: String,
    },
    /// Task progress update
    TaskProgress {
        task_id: String,
        step: ExecutionStep,
    },
    /// Task completed notification
    TaskCompleted {
        task_id: String,
        response: TaskResponse,
    },
    /// Task failed notification
    TaskFailed {
        task_id: String,
        error: String,
    },
}

