//! Task Executor
//!
//! This module handles the actual execution of tasks based on the understanding
//! and planning phases.

use crate::errors::AgentError;
use crate::task_helpers;
use crate::types::ExecutionResult;

/// Task Executor
///
/// Responsible for executing tasks based on the understanding provided by the
/// understanding engine. It uses pattern matching to identify task types and
/// delegates to appropriate helper functions.
pub struct TaskExecutor {
    // Future: Add execution context, state management, etc.
}

impl TaskExecutor {
    /// Create a new task executor
    pub fn new() -> Self {
        Self {}
    }

    /// Execute a task based on understanding
    ///
    /// # Arguments
    ///
    /// * `task_request` - The original task request
    /// * `task_understanding` - The AI's understanding of the task
    ///
    /// # Returns
    ///
    /// An `ExecutionResult` containing the outcome of the execution
    pub async fn execute_task(
        &self,
        _task_request: &str,
        task_understanding: &str,
    ) -> Result<ExecutionResult, AgentError> {
        tracing::info!("Executing task based on understanding: {}", task_understanding);

        // Check if the task mentions file operations
        let lower_understanding = task_understanding.to_lowercase();

        // Pattern 1: Read file
        if lower_understanding.contains("read") && lower_understanding.contains("file") {
            return self.execute_read_file(task_understanding).await;
        }

        // Pattern 2: List files
        if lower_understanding.contains("list") && lower_understanding.contains("file") {
            return self.execute_list_files(task_understanding).await;
        }

        // Pattern 3: Run command
        if lower_understanding.contains("run") && lower_understanding.contains("command") {
            return self.execute_run_command(task_understanding).await;
        }

        // Default: Return the understanding as the result
        Ok(ExecutionResult {
            success: true,
            summary: "Task completed".to_string(),
            details: format!("AI Analysis: {}", task_understanding),
            execution_time: 1,
        })
    }

    /// Execute file reading operation
    async fn execute_read_file(&self, task_understanding: &str) -> Result<ExecutionResult, AgentError> {
        if let Some(file_path) = task_helpers::extract_file_path(task_understanding) {
            match task_helpers::read_file(&file_path).await {
                Ok(content) => {
                    Ok(ExecutionResult {
                        success: true,
                        summary: format!("Successfully read file: {}", file_path),
                        details: content,
                        execution_time: 2,
                    })
                }
                Err(e) => {
                    Ok(ExecutionResult {
                        success: false,
                        summary: format!("Failed to read file: {}", file_path),
                        details: format!("Error: {}", e),
                        execution_time: 1,
                    })
                }
            }
        } else {
            Ok(ExecutionResult {
                success: false,
                summary: "Could not extract file path".to_string(),
                details: "Please specify a file path".to_string(),
                execution_time: 1,
            })
        }
    }

    /// Execute file listing operation
    async fn execute_list_files(&self, task_understanding: &str) -> Result<ExecutionResult, AgentError> {
        let path = task_helpers::extract_directory_path(task_understanding)
            .unwrap_or_else(|| ".".to_string());
        
        match task_helpers::list_files(&path).await {
            Ok(files) => {
                Ok(ExecutionResult {
                    success: true,
                    summary: "Successfully listed files".to_string(),
                    details: files,
                    execution_time: 1,
                })
            }
            Err(e) => {
                Ok(ExecutionResult {
                    success: false,
                    summary: "Failed to list files".to_string(),
                    details: format!("Error: {}", e),
                    execution_time: 1,
                })
            }
        }
    }

    /// Execute command running operation
    async fn execute_run_command(&self, task_understanding: &str) -> Result<ExecutionResult, AgentError> {
        if let Some(command) = task_helpers::extract_command(task_understanding) {
            match task_helpers::run_command(&command).await {
                Ok(output) => {
                    Ok(ExecutionResult {
                        success: true,
                        summary: format!("Successfully ran command: {}", command),
                        details: output,
                        execution_time: 3,
                    })
                }
                Err(e) => {
                    Ok(ExecutionResult {
                        success: false,
                        summary: format!("Failed to run command: {}", command),
                        details: format!("Error: {}", e),
                        execution_time: 1,
                    })
                }
            }
        } else {
            Ok(ExecutionResult {
                success: false,
                summary: "Could not extract command".to_string(),
                details: "Please specify a command to run".to_string(),
                execution_time: 1,
            })
        }
    }
}

impl Default for TaskExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_executor_creation() {
        let executor = TaskExecutor::new();
        let result = executor.execute_task(
            "test task",
            "This is a simple task"
        ).await;
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_execute_read_file() {
        let executor = TaskExecutor::new();
        let result = executor.execute_task(
            "read file",
            "Read the file Cargo.toml"
        ).await;
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert!(result.details.contains("task-runner"));
    }

    #[tokio::test]
    async fn test_execute_list_files() {
        let executor = TaskExecutor::new();
        let result = executor.execute_task(
            "list files",
            "List files in the current directory"
        ).await;
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
    }
}

