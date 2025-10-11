//! Task Agent Module
//!
//! This module provides the core Task Agent implementation, which coordinates
//! task understanding, planning, and execution.

mod executor;
mod planner;

pub use executor::TaskExecutor;
pub use planner::TaskPlanner;

use crate::config::AgentConfig;
use crate::errors::AgentError;
use crate::models::LanguageModel;
use crate::planning::PlanningEngine;
use crate::tools::ToolRegistry;
use crate::types::{Task, TaskResult, TaskStatus};
use std::sync::Arc;

/// Main AI-Native Task Agent
///
/// A general-purpose agent that can handle various types of tasks including:
/// - Code generation and refactoring
/// - File operations
/// - Command execution
/// - Documentation
/// - Testing
/// - And any other tasks defined through the tool system
///
/// # Architecture
///
/// The agent is composed of several components:
/// - **Understanding Engine**: Analyzes and understands task requirements
/// - **Task Planner**: Creates execution plans based on understanding
/// - **Task Executor**: Executes the planned tasks
/// - **Tool Registry**: Manages available tools for task execution
pub struct TaskAgent {
    model: Arc<dyn LanguageModel>,
    tools: Arc<ToolRegistry>,  // Removed Mutex - ToolRegistry has internal locking
    config: AgentConfig,
    planning_engine: PlanningEngine,
    _planner: TaskPlanner,  // Future: Use for advanced planning
    executor: TaskExecutor,
    _error_handler: crate::errors::ErrorHandler,
}



impl std::fmt::Debug for TaskAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskAgent")
            .field("config", &self.config)
            .field("tools", &"<ToolRegistry>")
            .field("model", &"<LanguageModel>")
            .finish()
    }
}

impl TaskAgent {
    /// Create a new task agent with the given model and configuration
    pub fn new(model: Box<dyn LanguageModel>, config: AgentConfig) -> Self {
        let _error_handler = crate::errors::ErrorHandler::new(
            config.execution.max_retries,
            config.execution.retry_delay_seconds,
        );

        // Convert Box to Arc for shared ownership
        let model_arc: Arc<dyn LanguageModel> = model.into();
        let planning_engine = PlanningEngine::new(Arc::clone(&model_arc));
        let planner = TaskPlanner::new();
        let executor = TaskExecutor::new();

        Self {
            model: model_arc,
            tools: Arc::new(ToolRegistry::new()),  // No Mutex needed
            config,
            planning_engine,
            _planner: planner,
            executor,
            _error_handler,
        }
    }

    /// Process a task from start to finish
    ///
    /// This is the main entry point for task execution. It coordinates:
    /// 1. Task understanding - AI analyzes the request
    /// 2. Task planning - Creates execution strategy
    /// 3. Task execution - Executes the plan
    ///
    /// # Arguments
    ///
    /// * `request` - The task request in natural language
    ///
    /// # Returns
    ///
    /// * `Ok(TaskResult)` - Task execution result with success status and details
    /// * `Err(AgentError)` - Error if task execution fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use task_runner::agent::TaskAgent;
    /// use task_runner::config::AgentConfig;
    /// use task_runner::models::MockModel;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let model = Box::new(MockModel::new("gpt-4".to_string()));
    /// let config = AgentConfig::default();
    /// let mut agent = TaskAgent::new(model, config);
    ///
    /// let result = agent.process_task("List files in current directory").await?;
    /// println!("Task completed: {}", result.success);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn process_task(&mut self, request: &str) -> Result<TaskResult, AgentError> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let task = Task {
            id: task_id.clone(),
            request: request.to_string(),
            status: TaskStatus::Pending,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            result: None,
        };

        self.execute_task_internal(task).await
    }

    /// Internal task execution workflow
    async fn execute_task_internal(&mut self, mut task: Task) -> Result<TaskResult, AgentError> {
        task.status = TaskStatus::InProgress;
        task.updated_at = chrono::Utc::now();

        // 1. Understanding phase - analyze task requirements
        let plan = self.planning_engine.analyze_task(&task.request).await?;

        tracing::info!(
            "Task plan created: {} steps estimated",
            plan.estimated_steps.unwrap_or(0)
        );

        // 2. Execution phase - delegate to executor
        let execution_result = self.executor.execute_task(&task.request, &plan.understanding).await?;

        // 3. Build result
        task.status = if execution_result.success {
            TaskStatus::Completed
        } else {
            TaskStatus::Failed
        };
        task.updated_at = chrono::Utc::now();

        Ok(TaskResult {
            success: execution_result.success,
            summary: execution_result.summary,
            details: Some(execution_result.details),
            execution_time: Some(execution_result.execution_time),
            task_plan: Some(plan),
        })
    }

    /// Register a tool with the agent
    ///
    /// Tools extend the agent's capabilities by providing specific operations
    /// like file reading, command execution, etc.
    ///
    /// # Arguments
    ///
    /// * `tool` - The tool to register, must implement the `Tool` trait
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use task_runner::agent::TaskAgent;
    /// use task_runner::config::AgentConfig;
    /// use task_runner::models::MockModel;
    /// use task_runner::tools::ReadFileTool;
    ///
    /// # async fn example() {
    /// let model = Box::new(MockModel::new("gpt-4".to_string()));
    /// let config = AgentConfig::default();
    /// let agent = TaskAgent::new(model, config);
    ///
    /// agent.register_tool(ReadFileTool).await;
    /// assert!(agent.has_tool("read_file").await);
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// This is an async method since ToolRegistry uses internal async locking.
    pub async fn register_tool<T: crate::tools::Tool + 'static>(&self, tool: T) {
        self.tools.register(tool).await;
    }

    /// Get reference to the tools registry
    pub fn get_tools(&self) -> Arc<ToolRegistry> {
        Arc::clone(&self.tools)
    }

    /// Check if a tool is registered
    pub async fn has_tool(&self, name: &str) -> bool {
        self.tools.has_tool(name).await
    }

    /// Get the number of registered tools
    pub async fn tool_count(&self) -> usize {
        self.tools.tool_count().await
    }

    /// Get reference to the language model
    pub fn get_model(&self) -> &Arc<dyn LanguageModel> {
        &self.model
    }

    /// Get reference to the configuration
    pub fn get_config(&self) -> &AgentConfig {
        &self.config
    }
}

/// Factory function to create an agent with default tools
pub fn create_agent_with_default_tools(
    model: Box<dyn LanguageModel>,
    config: AgentConfig,
) -> TaskAgent {
    TaskAgent::new(model, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MockModel;
    use crate::tools::ReadFileTool;

    #[tokio::test]
    async fn test_agent_creation() {
        let model = Box::new(MockModel::new("test".to_string()));
        let config = AgentConfig::default();
        let agent = TaskAgent::new(model, config);

        assert_eq!(agent.get_model().model_name(), "test");
    }

    #[tokio::test]
    async fn test_tool_registration() {
        let model = Box::new(MockModel::new("test".to_string()));
        let config = AgentConfig::default();
        let agent = TaskAgent::new(model, config);

        agent.register_tool(ReadFileTool).await;

        let tools = agent.get_tools();
        let tool_names = tools.get_tool_names().await;
        assert!(tool_names.contains(&"read_file".to_string()));
        assert!(agent.has_tool("read_file").await);
        assert_eq!(agent.tool_count().await, 1);
    }
}

