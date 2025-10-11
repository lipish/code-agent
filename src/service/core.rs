//! Core service implementation

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use tokio::time::timeout;
use chrono::Utc;
use uuid::Uuid;
use tracing::info;
use dashmap::DashMap;

use crate::agent::TaskAgent;
use crate::config::AgentConfig;
use crate::models::{LanguageModel, LlmModel};
use crate::service::types::{
    self as service_types,
    TaskRequest, TaskResponse, TaskStatus, TaskPlan, TaskMetrics, TaskComplexity,
    BatchTaskRequest, BatchTaskResponse, BatchExecutionMode, BatchStatistics,
    StepType, StepStatus, ExecutionStep,
    ServiceConfig, ServiceStatus,
};
use crate::service::error::{ServiceResult, ServiceErrorType, ErrorBuilder};
use crate::service::metrics_simple::{MetricsCollector, MetricsSnapshot};

/// Task Agent Service
///
/// A general-purpose service for executing various types of tasks through AI agents.
#[derive(Debug)]
pub struct TaskAgentService {
    /// Service configuration
    config: ServiceConfig,
    /// Metrics collector
    metrics: Arc<MetricsCollector>,
    /// Agent instance
    agent: Arc<RwLock<TaskAgent>>,
    /// Active tasks - using DashMap for lock-free concurrent access
    active_tasks: Arc<DashMap<String, TaskContext>>,
    /// Semaphore for limiting concurrent tasks
    task_semaphore: Arc<Semaphore>,
    /// Available tools
    available_tools: Vec<String>,
}

/// Task execution context
#[derive(Debug)]
#[allow(dead_code)]
struct TaskContext {
    /// Task ID
    task_id: String,
    /// Task request
    request: TaskRequest,
    /// Current status
    status: TaskStatus,
    /// Execution steps
    steps: Vec<ExecutionStep>,
    /// Start time
    start_time: Instant,
    /// Execution plan
    plan: Option<TaskPlan>,
    /// Metrics for this task
    metrics: TaskMetrics,
    /// Current step number
    current_step: u32,
}

impl TaskAgentService {
    /// Create a new service instance
    pub async fn new(config: ServiceConfig, agent_config: AgentConfig) -> ServiceResult<Self> {
        info!("Creating AI Agent Service with config: {:?}", config);

        // Create the agent
        let model = create_model_from_config(&agent_config)?;
        let agent = TaskAgent::new(model, agent_config.clone());

        // Register basic tools
        register_basic_tools(&agent).await?;

        let service = Self {
            available_tools: get_available_tools(),
            task_semaphore: Arc::new(Semaphore::new(config.max_concurrent_tasks as usize)),
            metrics: Arc::new(MetricsCollector::new()),
            agent: Arc::new(RwLock::new(agent)),
            active_tasks: Arc::new(DashMap::new()),
            config,
        };

        info!("AI Agent Service created successfully");
        Ok(service)
    }

    /// Execute a single task
    pub async fn execute_task(&self, request: TaskRequest) -> ServiceResult<TaskResponse> {
        let task_id = request.task_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
        info!("Executing task: {}", task_id);

        // Check rate limiting
        if let Some(_rate_limit) = &self.config.rate_limiting {
            // TODO: Implement rate limiting logic
        }

        // Acquire semaphore for concurrent task limit
        let _permit = self.task_semaphore.acquire()
            .await
            .map_err(|_| ErrorBuilder::service_unavailable("Service at capacity"))?;

        // Record task start
        self.metrics.record_task_start().await;

        // Create task context
        let task_context = TaskContext {
            task_id: task_id.clone(),
            request: request.clone(),
            status: TaskStatus::Running,
            steps: Vec::new(),
            start_time: Instant::now(),
            plan: None,
            metrics: TaskMetrics {
                total_time_ms: 0,
                model_time_ms: 0,
                tool_time_ms: 0,
                steps_executed: 0,
                tool_calls: 0,
                model_calls: 0,
                tokens_used: None,
                // Legacy fields
                total_execution_time: Some(0),
                planning_time_ms: Some(0),
                execution_time_ms: Some(0),
                tools_used: Some(0),
                memory_usage_mb: None,
                cpu_usage_percent: None,
                custom_metrics: Some(HashMap::new()),
            },
            current_step: 0,
        };

        // Register active task
        self.active_tasks.insert(task_id.clone(), task_context);

        // Execute the task with timeout
        let task_timeout = Duration::from_secs(
            request.context
                .as_ref()
                .and_then(|c| c.constraints.as_ref())
                .and_then(|c| c.max_execution_time)
                .unwrap_or(self.config.default_task_timeout.unwrap_or(self.config.request_timeout_seconds))
        );

        let result = timeout(task_timeout, self.execute_task_internal(task_id.clone())).await;

        // Remove from active tasks
        self.active_tasks.remove(&task_id);

        match result {
            Ok(task_result) => {
                self.metrics.record_task_completion(
                    task_result.metrics.total_execution_time.unwrap_or(task_result.metrics.total_time_ms) as f64,
                    task_result.status == TaskStatus::Completed,
                ).await;
                Ok(task_result)
            }
            Err(_) => {
                let error = ErrorBuilder::task_timeout(&task_id);
                self.metrics.record_task_completion(0.0, false).await;
                self.metrics.record_error("timeout").await;

                let (plan, steps, metrics) = if let Some(context) = self.active_tasks.get(&task_id) {
                    (context.plan.clone(), context.steps.clone(), context.metrics.clone())
                } else {
                    (None, Vec::new(), TaskMetrics::default())
                };

                Ok(TaskResponse {
                    task_id: task_id.clone(),
                    status: TaskStatus::Timeout,
                    result: None,
                    plan,
                    steps,
                    metrics,
                    error: Some(error),
                    created_at: Utc::now(),
                    started_at: Some(Utc::now()),
                    completed_at: Some(Utc::now()),
                })
            }
        }
    }

    /// Execute a batch of tasks
    pub async fn execute_batch(&self, request: BatchTaskRequest) -> ServiceResult<BatchTaskResponse> {
        let batch_id = Uuid::new_v4().to_string();
        info!("Executing batch: {} with {} tasks", batch_id, request.tasks.len());

        let start_time = Instant::now();
        let mut responses = Vec::new();

        match request.mode {
            BatchExecutionMode::Sequential => {
                for task_request in request.tasks {
                    let response = self.execute_task(task_request).await;
                    if !response.as_ref().map(|r| r.status == TaskStatus::Completed).unwrap_or(false)
                        && !request.continue_on_error {
                        return Err(ErrorBuilder::task_execution_failed("Batch execution stopped due to task failure"));
                    }
                    responses.push(response);
                }
            }
            BatchExecutionMode::Parallel => {
                let futures = request.tasks.into_iter()
                    .map(|task_request| self.execute_task(task_request))
                    .collect::<Vec<_>>();

                let results = futures::future::join_all(futures).await;
                responses = results;
            }
        }

        let _total_time = start_time.elapsed().as_secs();
        let mut completed_tasks = 0;
        let mut failed_tasks = 0;
        let mut total_execution_time = 0u64;

        for response in &responses {
            if let Ok(resp) = response {
                match resp.status {
                    TaskStatus::Completed => {
                        completed_tasks += 1;
                        total_execution_time += resp.metrics.total_execution_time.unwrap_or(resp.metrics.total_time_ms);
                    }
                    _ => failed_tasks += 1,
                }
            } else {
                failed_tasks += 1;
            }
        }

        let statistics = BatchStatistics {
            total_tasks: responses.len(),
            successful_tasks: completed_tasks,
            failed_tasks,
            total_time_ms: total_execution_time,
            average_time_ms: if completed_tasks > 0 {
                total_execution_time / completed_tasks as u64
            } else {
                0
            },
            // Legacy fields
            completed_tasks: Some(completed_tasks),
            total_execution_time: Some(total_execution_time),
            average_execution_time: Some(if completed_tasks > 0 {
                total_execution_time / completed_tasks as u64
            } else {
                0
            }),
        };

        let results = responses.into_iter().collect::<Result<Vec<_>, _>>()
            .map_err(|_| ErrorBuilder::internal_server_error("Failed to collect batch responses"))?;

        Ok(BatchTaskResponse {
            batch_id,
            results: results.clone(),
            statistics,
            responses: Some(results),  // Legacy field
        })
    }

    /// Get task status
    pub async fn get_task_status(&self, task_id: &str) -> ServiceResult<TaskResponse> {
        if let Some(task_context) = self.active_tasks.get(task_id) {
            Ok(TaskResponse {
                task_id: task_id.to_string(),
                status: task_context.status.clone(),
                result: None,
                plan: task_context.plan.clone(),
                steps: task_context.steps.clone(),
                metrics: task_context.metrics.clone(),
                error: None,
                created_at: Utc::now(),
                started_at: Some(Utc::now()),
                completed_at: None,
            })
        } else {
            Err(ErrorBuilder::task_not_found(task_id))
        }
    }

    /// Cancel a running task
    pub async fn cancel_task(&self, task_id: &str) -> ServiceResult<()> {
        if let Some(mut task_context) = self.active_tasks.get_mut(task_id) {
            task_context.status = TaskStatus::Cancelled;
            info!("Task {} cancelled", task_id);
            Ok(())
        } else {
            Err(ErrorBuilder::task_not_found(task_id))
        }
    }

    /// Get service status
    pub async fn get_service_status(&self) -> ServiceResult<ServiceStatus> {
        let metrics_snapshot = self.metrics.get_metrics_snapshot().await;
        let health = self.metrics.get_health_status().await;
        let health_str = format!("{:?}", health);

        Ok(ServiceStatus {
            name: "AI Agent Service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            health,
            uptime_seconds: metrics_snapshot.uptime_seconds,
            active_tasks: metrics_snapshot.active_tasks as usize,
            total_tasks_processed: metrics_snapshot.completed_tasks + metrics_snapshot.failed_tasks,
            system_metrics: metrics_snapshot.system_metrics,
            network_metrics: Default::default(),
            timestamp: Utc::now(),
            // Legacy fields
            status: Some(health_str),
            completed_tasks: Some(metrics_snapshot.completed_tasks),
            failed_tasks: Some(metrics_snapshot.failed_tasks),
            available_tools: self.available_tools.clone(),
            last_updated: Some(Utc::now()),
        })
    }

    /// Get metrics snapshot
    pub async fn get_metrics(&self) -> ServiceResult<MetricsSnapshot> {
        Ok(self.metrics.get_metrics_snapshot().await)
    }

    /// Internal task execution
    async fn execute_task_internal(&self, task_id: String) -> TaskResponse {
        info!("Starting internal execution for task: {}", task_id);

        // Get task request from active tasks
        let task_request = if let Some(context) = self.active_tasks.get(&task_id) {
            context.request.clone()
        } else {
            return TaskResponse {
                task_id: task_id.clone(),
                status: TaskStatus::Failed,
                result: None,
                plan: None,
                steps: Vec::new(),
                metrics: TaskMetrics::default(),
                error: Some(ErrorBuilder::task_not_found(&task_id)),
                created_at: Utc::now(),
                started_at: Some(Utc::now()),
                completed_at: Some(Utc::now()),
            };
        };

        let planning_start = Instant::now();
        let mut steps = Vec::new();

        // Create execution step for planning
        let now = Utc::now();
        steps.push(ExecutionStep {
            step_number: 1,
            step_type: StepType::Planning,
            description: "Understanding task and creating execution plan".to_string(),
            status: StepStatus::Running,
            output: None,
            error: None,
            started_at: Some(now),
            completed_at: None,
            duration_ms: None,
            // Legacy fields
            input: Some(serde_json::to_value(&task_request).unwrap_or_default()),
            execution_time_ms: Some(0),
            timestamp: Some(now),
        });

        // Execute task using the agent
        let agent_result = {
            let mut agent = self.agent.write().await;
            match agent.process_task(&task_request.task).await {
                Ok(result) => {
                    // Update planning step
                    let planning_duration = planning_start.elapsed().as_millis() as u64;
                    if let Some(step) = steps.last_mut() {
                        step.status = StepStatus::Completed;
                        step.duration_ms = Some(planning_duration);
                        step.execution_time_ms = Some(planning_duration);
                        step.completed_at = Some(Utc::now());
                        step.output = Some(serde_json::json!({
                            "task_plan": result.task_plan,
                            "success": result.success
                        }).to_string());
                    }

                    Some(result)
                }
                Err(e) => {
                    // Update planning step with error
                    let planning_duration = planning_start.elapsed().as_millis() as u64;
                    if let Some(step) = steps.last_mut() {
                        step.status = StepStatus::Failed;
                        step.duration_ms = Some(planning_duration);
                        step.execution_time_ms = Some(planning_duration);
                        step.completed_at = Some(Utc::now());
                        step.error = Some(e.to_string());
                    }

                    let service_error = ServiceErrorType::from(e).to_service_error();
                    self.metrics.record_error(&service_error.code).await;

                    // Get metrics from context
                    let metrics = if let Some(context) = self.active_tasks.get(&task_id) {
                        context.metrics.clone()
                    } else {
                        TaskMetrics::default()
                    };

                    return TaskResponse {
                        task_id: task_id.clone(),
                        status: TaskStatus::Failed,
                        result: None,
                        plan: None,
                        steps,
                        metrics,
                        error: Some(service_error),
                        created_at: Utc::now(),
                        started_at: Some(Utc::now()),
                        completed_at: Some(Utc::now()),
                    };
                }
            }
        };

        if let Some(agent_result) = agent_result {
            // Create execution step for task execution
            let execution_start = Instant::now();
            let exec_duration = execution_start.elapsed().as_millis() as u64;
            let exec_now = Utc::now();
            steps.push(ExecutionStep {
                step_number: 2,
                step_type: StepType::Execution,
                description: "Executing task plan".to_string(),
                status: if agent_result.success { StepStatus::Completed } else { StepStatus::Failed },
                output: Some(serde_json::json!({
                    "summary": agent_result.summary,
                    "success": agent_result.success
                }).to_string()),
                error: if !agent_result.success {
                    agent_result.details.clone()
                } else {
                    None
                },
                started_at: Some(exec_now),
                completed_at: Some(exec_now),
                duration_ms: Some(exec_duration),
                // Legacy fields
                input: Some(serde_json::json!({
                    "task": task_request.task
                })),
                execution_time_ms: Some(exec_duration),
                timestamp: Some(exec_now),
            });

            // Create completion step
            let comp_now = Utc::now();
            steps.push(ExecutionStep {
                step_number: 3,
                step_type: StepType::Completion,
                description: "Task execution completed".to_string(),
                status: StepStatus::Completed,
                output: Some(serde_json::json!({
                    "execution_time": agent_result.execution_time,
                    "success": agent_result.success
                }).to_string()),
                error: None,
                started_at: Some(comp_now),
                completed_at: Some(comp_now),
                duration_ms: Some(0),
                // Legacy fields
                input: None,
                execution_time_ms: Some(0),
                timestamp: Some(comp_now),
            });

            // Update task context
            if let Some(mut context) = self.active_tasks.get_mut(&task_id) {
                context.status = if agent_result.success {
                    TaskStatus::Completed
                } else {
                    TaskStatus::Failed
                };
                context.steps = steps.clone();
                context.plan = agent_result.task_plan.as_ref().map(|p| convert_task_plan(p.clone()));
                context.metrics.total_execution_time = Some(agent_result.execution_time.unwrap_or(0));
                context.metrics.planning_time_ms = Some(planning_start.elapsed().as_millis() as u64);
                context.metrics.execution_time_ms = Some(execution_start.elapsed().as_millis() as u64);
                context.metrics.steps_executed = steps.len() as u32;
            }

            // Get final metrics
            let metrics = if let Some(context) = self.active_tasks.get(&task_id) {
                context.metrics.clone()
            } else {
                TaskMetrics::default()
            };

            TaskResponse {
                task_id: task_id.clone(),
                status: if agent_result.success { TaskStatus::Completed } else { TaskStatus::Failed },
                result: Some(service_types::TaskResult {
                    success: agent_result.success,
                    summary: agent_result.summary,
                    details: agent_result.details,
                    artifacts: Vec::new(), // TODO: Extract artifacts from result
                    execution_time: agent_result.execution_time.unwrap_or(0),
                }),
                plan: agent_result.task_plan.map(convert_task_plan),
                steps,
                metrics,
                error: None,
                created_at: Utc::now(),
                started_at: Some(Utc::now()),
                completed_at: Some(Utc::now()),
            }
        } else {
            // Get metrics from context
            let metrics = if let Some(context) = self.active_tasks.get(&task_id) {
                context.metrics.clone()
            } else {
                TaskMetrics::default()
            };

            TaskResponse {
                task_id: task_id.clone(),
                status: TaskStatus::Failed,
                result: None,
                plan: None,
                steps,
                metrics,
                error: Some(ErrorBuilder::task_execution_failed("No result from agent")),
                created_at: Utc::now(),
                started_at: Some(Utc::now()),
                completed_at: Some(Utc::now()),
            }
        }
    }
}

/// Create model from configuration
fn create_model_from_config(config: &AgentConfig) -> Result<Box<dyn LanguageModel>, ServiceErrorType> {
    LlmModel::from_config(config.model.clone())
        .map(|m| Box::new(m) as Box<dyn LanguageModel>)
        .map_err(|e| ServiceErrorType::ConfigurationError(format!("Failed to create model: {}", e)))
}

/// Register basic tools with the agent
async fn register_basic_tools(agent: &TaskAgent) -> ServiceResult<()> {
    use crate::tools::{ReadFileTool, WriteFileTool, RunCommandTool, ListFilesTool};

    agent.register_tool(ReadFileTool).await;
    agent.register_tool(WriteFileTool).await;
    agent.register_tool(RunCommandTool).await;
    agent.register_tool(ListFilesTool).await;

    Ok(())
}

/// Get list of available tools
fn get_available_tools() -> Vec<String> {
    vec![
        "read_file".to_string(),
        "write_file".to_string(),
        "run_command".to_string(),
        "list_files".to_string(),
    ]
}

/// Convert types::TaskPlan to service::types::TaskPlan
fn convert_task_plan(plan: crate::types::TaskPlan) -> TaskPlan {
    TaskPlan {
        understanding: plan.understanding.clone(),
        approach: plan.approach.clone(),
        complexity: match plan.complexity {
            crate::types::TaskComplexity::Simple => TaskComplexity::Simple,
            crate::types::TaskComplexity::Moderate => TaskComplexity::Medium,
            crate::types::TaskComplexity::Complex => TaskComplexity::Complex,
        },
        steps: vec![plan.approach],  // Convert approach to steps
        required_tools: vec![],  // Not available in types::TaskPlan
        estimated_time: None,  // Not available in types::TaskPlan
        estimated_steps: plan.estimated_steps,
        requirements: plan.requirements,
        created_at: Some(Utc::now()),
    }
}