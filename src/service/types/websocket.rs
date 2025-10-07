//! WebSocket message types
//!
//! This module contains types for WebSocket communication.

use serde::{Deserialize, Serialize};
use super::task::{TaskResponse, ExecutionStep};

/// WebSocket message types
///
/// Note: Large variants are boxed to reduce enum size and improve performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebSocketMessage {
    /// Task started notification
    TaskStarted {
        task_id: String,
    },
    /// Task progress update
    ///
    /// ExecutionStep is boxed to reduce enum size
    TaskProgress {
        task_id: String,
        #[serde(flatten)]
        step: Box<ExecutionStep>,
    },
    /// Task completed notification
    ///
    /// TaskResponse is boxed to reduce enum size (664 bytes -> pointer size)
    TaskCompleted {
        task_id: String,
        #[serde(flatten)]
        response: Box<TaskResponse>,
    },
    /// Task failed notification
    TaskFailed {
        task_id: String,
        error: String,
    },
}

