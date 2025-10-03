//! Core service implementation

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use tokio::time::timeout;
use chrono::Utc;
use uuid::Uuid;
use tracing::info;

use crate::agent::CodeAgent;
use crate::config::AgentConfig;
use crate::models::{LanguageModel, ZhipuModel};
use crate::{ServiceConfig, TaskRequest, TaskResponse, TaskStatus, ServiceStatus, MetricsSnapshot, TaskResult};
use crate::service_types::{TaskPlan, TaskMetrics, BatchTaskRequest, BatchTaskResponse, BatchExecutionMode, BatchStatistics, StepType, StepStatus, ExecutionStep};
use crate::service::error::{ServiceResult, ServiceErrorType, ErrorBuilder};
use crate::service::metrics_simple::MetricsCollector;

/// Code Agent Service
#[derive(Debug)]
pub struct CodeAgentService {
    /// Service configuration
    config: ServiceConfig,
    /// Metrics collector
    metrics: Arc<MetricsCollector>,
    /// Agent instance
    agent: Arc<RwLock<CodeAgent>>,
    /// Active tasks
    active_tasks: Arc<RwLock<HashMap<String, Arc<RwLock<TaskContext>>>>>,
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

impl CodeAgentService {
    /// Create a new service instance
    pub async fn new(config: ServiceConfig, agent_config: AgentConfig) -> ServiceResult<Self> {
        info!("Creating AI Agent Service with config: {:?}", config);

        // Create the agent
        let model = create_model_from_config(&agent_config)?;
        let mut agent = CodeAgent::new(model, agent_config.clone());

        // Register basic tools
        register_basic_tools(&mut agent).await?;

        let service = Self {
            available_tools: get_available_tools(),
            task_semaphore: Arc::new(Semaphore::new(config.max_concurrent_tasks as usize)),
            metrics: Arc::new(MetricsCollector::new()),
            agent: Arc::new(RwLock::new(agent)),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
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
        let task_context = Arc::new(RwLock::new(TaskContext {
            task_id: task_id.clone(),
            request: request.clone(),
            status: TaskStatus::Running,
            steps: Vec::new(),
            start_time: Instant::now(),
            plan: None,
            metrics: TaskMetrics {
                total_execution_time: 0,
                planning_time_ms: 0,
                execution_time_ms: 0,
                steps_executed: 0,
                tools_used: 0,
                memory_usage_mb: None,
                cpu_usage_percent: None,
                custom_metrics: HashMap::new(),
            },
            current_step: 0,
        }));

        // Register active task
        {
            let mut active_tasks = self.active_tasks.write().await;
            active_tasks.insert(task_id.clone(), task_context.clone());
        }

        // Execute the task with timeout
        let task_timeout = Duration::from_secs(
            request.context
                .as_ref()
                .and_then(|c| c.constraints.as_ref())
                .and_then(|c| c.max_execution_time)
                .unwrap_or(self.config.default_task_timeout)
        );

        let result = timeout(task_timeout, self.execute_task_internal(task_context.clone())).await;

        // Remove from active tasks
        {
            let mut active_tasks = self.active_tasks.write().await;
            active_tasks.remove(&task_id);
        }

        match result {
            Ok(task_result) => {
                self.metrics.record_task_completion(
                    task_result.metrics.total_execution_time as f64,
                    task_result.status == TaskStatus::Completed,
                ).await;
                Ok(task_result)
            }
            Err(_) => {
                let error = ErrorBuilder::task_timeout(&task_id);
                self.metrics.record_task_completion(0.0, false).await;
                self.metrics.record_error("timeout").await;

                let context = task_context.read().await;
                Ok(TaskResponse {
                    task_id: task_id.clone(),
                    status: TaskStatus::Timeout,
                    result: None,
                    plan: context.plan.clone(),
                    steps: context.steps.clone(),
                    metrics: context.metrics.clone(),
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
                        total_execution_time += resp.metrics.total_execution_time;
                    }
                    _ => failed_tasks += 1,
                }
            } else {
                failed_tasks += 1;
            }
        }

        let statistics = BatchStatistics {
            total_tasks: responses.len() as u32,
            completed_tasks,
            failed_tasks,
            total_execution_time,
            average_execution_time: if completed_tasks > 0 {
                total_execution_time as f64 / completed_tasks as f64
            } else {
                0.0
            },
        };

        Ok(BatchTaskResponse {
            batch_id,
            responses: responses.into_iter().collect::<Result<Vec<_>, _>>()
                .map_err(|_| ErrorBuilder::internal_server_error("Failed to collect batch responses"))?,
            statistics,
        })
    }

    /// Get task status
    pub async fn get_task_status(&self, task_id: &str) -> ServiceResult<TaskResponse> {
        let active_tasks = self.active_tasks.read().await;

        if let Some(task_context) = active_tasks.get(task_id) {
            let context = task_context.read().await;
            Ok(TaskResponse {
                task_id: task_id.to_string(),
                status: context.status.clone(),
                result: None,
                plan: context.plan.clone(),
                steps: context.steps.clone(),
                metrics: context.metrics.clone(),
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
        let active_tasks = self.active_tasks.write().await;

        if let Some(task_context) = active_tasks.get(task_id) {
            let mut context = task_context.write().await;
            context.status = TaskStatus::Cancelled;
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

        Ok(ServiceStatus {
            name: "AI Agent Service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            status: health,
            uptime_seconds: metrics_snapshot.uptime_seconds,
            active_tasks: metrics_snapshot.active_tasks as u32,
            completed_tasks: metrics_snapshot.completed_tasks,
            failed_tasks: metrics_snapshot.failed_tasks,
            available_tools: self.available_tools.clone(),
            system_metrics: metrics_snapshot.system_metrics,
            last_updated: Utc::now(),
        })
    }

    /// Get metrics snapshot
    pub async fn get_metrics(&self) -> ServiceResult<MetricsSnapshot> {
        Ok(self.metrics.get_metrics_snapshot().await)
    }

    /// Internal task execution
    async fn execute_task_internal(&self, task_context: Arc<RwLock<TaskContext>>) -> TaskResponse {
        let task_id = {
            let context = task_context.read().await;
            context.task_id.clone()
        };

        info!("Starting internal execution for task: {}", task_id);

        // Extract task request
        let (task_request, task_id_clone) = {
            let context = task_context.read().await;
            (context.request.clone(), context.task_id.clone())
        };

        let planning_start = Instant::now();
        let mut steps = Vec::new();

        // Create execution step for planning
        steps.push(ExecutionStep {
            step_number: 1,
            step_type: StepType::Planning,
            description: "Understanding task and creating execution plan".to_string(),
            input: Some(serde_json::to_value(&task_request).unwrap_or_default()),
            output: None,
            status: StepStatus::Running,
            error: None,
            execution_time_ms: 0,
            timestamp: Utc::now(),
        });

        // Execute task using the agent
        let agent_result = {
            let mut agent = self.agent.write().await;
            match agent.process_task(&task_request.task).await {
                Ok(result) => {
                    // Update planning step
                    if let Some(step) = steps.last_mut() {
                        step.status = StepStatus::Completed;
                        step.execution_time_ms = planning_start.elapsed().as_millis() as u64;
                        step.output = Some(serde_json::json!({
                            "task_plan": result.task_plan,
                            "success": result.success
                        }));
                    }

                    Some(result)
                }
                Err(e) => {
                    // Update planning step with error
                    if let Some(step) = steps.last_mut() {
                        step.status = StepStatus::Failed;
                        step.execution_time_ms = planning_start.elapsed().as_millis() as u64;
                        step.error = Some(e.to_string());
                    }

                    let service_error = ServiceErrorType::from(e).to_service_error();
                    self.metrics.record_error(&service_error.code).await;

                    return {
                        let context = task_context.read().await;
                        TaskResponse {
                            task_id: task_id_clone,
                            status: TaskStatus::Failed,
                            result: None,
                            plan: None,
                            steps,
                            metrics: context.metrics.clone(),
                            error: Some(service_error),
                            created_at: Utc::now(),
                            started_at: Some(Utc::now()),
                            completed_at: Some(Utc::now()),
                        }
                    };
                }
            }
        };

        if let Some(agent_result) = agent_result {
            // Create execution step for task execution
            let execution_start = Instant::now();
            steps.push(ExecutionStep {
                step_number: 2,
                step_type: StepType::Execution,
                description: "Executing task plan".to_string(),
                input: Some(serde_json::json!({
                    "task": task_request.task
                })),
                output: Some(serde_json::json!({
                    "summary": agent_result.summary,
                    "success": agent_result.success
                })),
                status: if agent_result.success { StepStatus::Completed } else { StepStatus::Failed },
                error: if !agent_result.success {
                    agent_result.details.clone()
                } else {
                    None
                },
                execution_time_ms: execution_start.elapsed().as_millis() as u64,
                timestamp: Utc::now(),
            });

            // Create completion step
            steps.push(ExecutionStep {
                step_number: 3,
                step_type: StepType::Completion,
                description: "Task execution completed".to_string(),
                input: None,
                output: Some(serde_json::json!({
                    "execution_time": agent_result.execution_time,
                    "success": agent_result.success
                })),
                status: StepStatus::Completed,
                error: None,
                execution_time_ms: 0,
                timestamp: Utc::now(),
            });

            // Update task context
            {
                let mut context = task_context.write().await;
                context.status = if agent_result.success {
                    TaskStatus::Completed
                } else {
                    TaskStatus::Failed
                };
                context.steps = steps.clone();
                context.plan = agent_result.task_plan.as_ref().map(|p| convert_task_plan(p.clone()));
                context.metrics.total_execution_time = agent_result.execution_time.unwrap_or(0);
                context.metrics.planning_time_ms = planning_start.elapsed().as_millis() as u64;
                context.metrics.execution_time_ms = execution_start.elapsed().as_millis() as u64;
                context.metrics.steps_executed = steps.len() as u32;
            }

            let context = task_context.read().await;
            TaskResponse {
                task_id: task_id_clone,
                status: if agent_result.success { TaskStatus::Completed } else { TaskStatus::Failed },
                result: Some(TaskResult {
                    success: agent_result.success,
                    summary: agent_result.summary,
                    details: agent_result.details,
                    artifacts: Vec::new(), // TODO: Extract artifacts from result
                    execution_time: agent_result.execution_time.unwrap_or(0),
                }),
                plan: agent_result.task_plan.map(convert_task_plan),
                steps: steps,
                metrics: context.metrics.clone(),
                error: None,
                created_at: Utc::now(),
                started_at: Some(Utc::now()),
                completed_at: Some(Utc::now()),
            }
        } else {
            let context = task_context.read().await;
            TaskResponse {
                task_id: task_id_clone,
                status: TaskStatus::Failed,
                result: None,
                plan: None,
                steps,
                metrics: context.metrics.clone(),
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
    match &config.model.provider {
        crate::config::ModelProvider::Zhipu => {
            let api_key = config.model.api_key.clone()
                .ok_or_else(|| ServiceErrorType::ConfigurationError("Zhipu API key not found".to_string()))?;
            Ok(Box::new(ZhipuModel::new(
                api_key,
                config.model.model_name.clone(),
                config.model.endpoint.clone(),
            )))
        }
        // TODO: Implement other model providers
        _ => Err(ServiceErrorType::ConfigurationError("Unsupported model provider".to_string())),
    }
}

/// Register basic tools with the agent
async fn register_basic_tools(agent: &mut CodeAgent) -> ServiceResult<()> {
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

/// Convert types::TaskPlan to service_types::TaskPlan
fn convert_task_plan(plan: crate::types::TaskPlan) -> TaskPlan {
    TaskPlan {
        understanding: plan.understanding,
        approach: plan.approach,
        complexity: match plan.complexity {
            crate::types::TaskComplexity::Simple => crate::service_types::TaskComplexity::Simple,
            crate::types::TaskComplexity::Moderate => crate::service_types::TaskComplexity::Moderate,
            crate::types::TaskComplexity::Complex => crate::service_types::TaskComplexity::Complex,
        },
        estimated_steps: plan.estimated_steps.unwrap_or(1),
        requirements: plan.requirements,
        created_at: Utc::now(),
    }
}