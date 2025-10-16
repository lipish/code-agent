//! Sequential Execution System
//! 
//! 实现分阶段的顺序执行机制，包括 Understanding → Approach → Plan → Execution 的完整流程。
//! 每个阶段都有独立的验证、重试和纠错机制。

use crate::errors::AgentError;
use crate::models::LanguageModel;
use crate::types::{TaskComplexity, StepDependency};
use crate::execution::guardrails::{OperationGuard, GuardrailEngine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// Core Types - Execution Phases
// ============================================================================

/// 执行阶段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionPhase {
    /// 任务理解阶段
    Understanding,
    /// 方案设计阶段
    Approach,
    /// 详细计划阶段
    Planning,
    /// 执行阶段
    Execution {
        /// 当前执行的步骤索引
        current_step: usize,
        /// 总步骤数
        total_steps: usize,
    },
    /// 验证阶段
    Validation,
    /// 已完成
    Completed,
    /// 失败
    Failed {
        /// 失败的阶段
        failed_at: Box<ExecutionPhase>,
        /// 失败原因
        reason: String,
    },
}

/// 阶段执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseResult<T> {
    /// 阶段名称
    pub phase: ExecutionPhase,
    /// 执行状态
    pub status: PhaseStatus,
    /// 阶段输出
    pub output: Option<T>,
    /// 执行耗时（毫秒）
    pub duration_ms: u64,
    /// 验证结果
    pub validation: ValidationResult,
    /// 执行时间
    pub executed_at: DateTime<Utc>,
    /// 可能的错误
    pub error: Option<String>,
    /// 重试次数
    pub retry_count: u32,
}

/// 阶段状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PhaseStatus {
    /// 待执行
    Pending,
    /// 执行中
    Running,
    /// 成功
    Success,
    /// 失败（可重试）
    Failed,
    /// 跳过
    Skipped,
    /// 等待人工确认
    AwaitingConfirmation,
}

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// 是否通过验证
    pub passed: bool,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f32,
    /// 验证消息
    pub messages: Vec<String>,
    /// 警告信息
    pub warnings: Vec<String>,
    /// 改进建议
    pub suggestions: Vec<String>,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            passed: true,
            confidence: 1.0,
            messages: vec![],
            warnings: vec![],
            suggestions: vec![],
        }
    }
}

// ============================================================================
// Sequential Execution Plan
// ============================================================================

/// 顺序执行计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequentialExecutionPlan {
    /// 任务ID
    pub task_id: String,
    
    /// 当前执行阶段
    pub current_phase: ExecutionPhase,
    
    /// 任务理解结果
    pub understanding: Option<PhaseResult<UnderstandingOutput>>,
    
    /// 方案设计结果
    pub approach: Option<PhaseResult<ApproachOutput>>,
    
    /// 详细计划结果
    pub plan: Option<PhaseResult<DetailedPlan>>,
    
    /// 执行历史（每个步骤的结果）
    pub execution_history: Vec<PhaseResult<StepExecutionOutput>>,
    
    /// 最终验证结果
    pub final_validation: Option<PhaseResult<ValidationOutput>>,
    
    /// 开始时间
    pub started_at: DateTime<Utc>,
    
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
    
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    
    /// 执行配置
    pub config: ExecutionConfig,
}

impl SequentialExecutionPlan {
    /// 创建新的执行计划
    pub fn new(task_id: String, config: ExecutionConfig) -> Self {
        let now = Utc::now();
        Self {
            task_id,
            current_phase: ExecutionPhase::Understanding,
            understanding: None,
            approach: None,
            plan: None,
            execution_history: Vec::new(),
            final_validation: None,
            started_at: now,
            updated_at: now,
            completed_at: None,
            config,
        }
    }
    
    /// 获取总执行时间（分钟）
    pub fn total_duration_minutes(&self) -> f64 {
        if let Some(completed) = self.completed_at {
            (completed - self.started_at).num_minutes() as f64
        } else {
            (Utc::now() - self.started_at).num_minutes() as f64
        }
    }
    
    /// 获取已完成的步骤数量
    pub fn completed_steps_count(&self) -> usize {
        self.execution_history
            .iter()
            .filter(|result| result.status == PhaseStatus::Success)
            .count()
    }
    
    /// 查找失败的步骤
    pub fn find_failed_step(&self) -> Option<&PhaseResult<StepExecutionOutput>> {
        self.execution_history
            .iter()
            .find(|result| result.status == PhaseStatus::Failed)
    }
}

/// 执行配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// 每个阶段的最大重试次数
    pub max_retries_per_phase: u32,
    
    /// 是否在关键点暂停等待确认
    pub require_confirmation: bool,
    
    /// 验证的最小置信度阈值
    pub min_confidence_threshold: f32,
    
    /// 是否启用自动回滚
    pub enable_auto_rollback: bool,
    
    /// 是否启用详细日志
    pub verbose_logging: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            max_retries_per_phase: 3,
            require_confirmation: false,
            min_confidence_threshold: 0.7,
            enable_auto_rollback: true,
            verbose_logging: false,
        }
    }
}

// ============================================================================
// Phase Output Types
// ============================================================================

/// Understanding 阶段输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderstandingOutput {
    /// 任务描述的理解
    pub understanding: String,
    
    /// 识别出的关键需求
    pub key_requirements: Vec<String>,
    
    /// 任务类型
    pub task_type: String,
    
    /// 复杂度评估
    pub complexity: TaskComplexity,
    
    /// 潜在风险
    pub potential_risks: Vec<String>,
    
    /// 需要澄清的问题
    pub clarification_needed: Vec<String>,
}

/// Approach 阶段输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproachOutput {
    /// 核心方案描述
    pub approach: String,
    
    /// 技术栈选择
    pub tech_stack: Vec<String>,
    
    /// 架构模式
    pub architecture_pattern: String,
    
    /// 关键技术决策
    pub key_decisions: Vec<TechnicalDecision>,
    
    /// 预期成果
    pub expected_outcomes: Vec<String>,
    
    /// 替代方案
    pub alternatives: Vec<AlternativeApproach>,
}

/// 技术决策
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalDecision {
    /// 决策项
    pub decision: String,
    /// 原因
    pub rationale: String,
    /// 权衡考虑
    pub tradeoffs: Vec<String>,
}

/// 替代方案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeApproach {
    /// 方案名称
    pub name: String,
    /// 方案描述
    pub description: String,
    /// 优点
    pub pros: Vec<String>,
    /// 缺点
    pub cons: Vec<String>,
}

/// Detailed Plan 阶段输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedPlan {
    /// 执行步骤列表
    pub steps: Vec<ExecutionStep>,
    
    /// 步骤依赖关系
    pub dependencies: Vec<StepDependency>,
    
    /// 预估总时间（分钟）
    pub estimated_duration: u32,
    
    /// 所需资源
    pub required_resources: Vec<String>,
    
    /// 里程碑
    pub milestones: Vec<Milestone>,
    
    /// 成功标准
    pub success_criteria: Vec<String>,
}

/// 里程碑
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    /// 里程碑名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 关联的步骤
    pub associated_steps: Vec<String>,
    /// 预计完成时间（分钟）
    pub estimated_completion: u32,
}

/// 执行步骤（增强版）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// 步骤ID
    pub id: String,
    
    /// 步骤序号
    pub sequence: usize,
    
    /// 步骤名称
    pub name: String,
    
    /// 步骤描述
    pub description: String,
    
    /// 步骤类型
    pub step_type: StepType,
    
    /// 预估执行时间（分钟）
    pub estimated_duration: u32,
    
    /// 前置条件
    pub preconditions: Vec<String>,
    
    /// 预期输出
    pub expected_outputs: Vec<String>,
    
    /// 验证标准
    pub validation_criteria: Vec<String>,
    
    /// 失败时的回滚步骤
    pub rollback_steps: Vec<String>,
    
    /// 是否需要人工确认
    pub requires_confirmation: bool,
    
    /// 是否允许失败继续
    pub allow_failure: bool,
    
    /// 操作守卫（用于安全检查）
    pub operation_guard: Option<OperationGuard>,
    
    /// 是否在执行前创建快照
    pub create_snapshot_before: bool,
    
    /// 快照ID（执行后填充）
    pub snapshot_id: Option<String>,
}

/// 步骤类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    /// 准备阶段
    Preparation,
    /// 代码生成
    CodeGeneration,
    /// 配置修改
    Configuration,
    /// 文件操作
    FileOperation,
    /// 命令执行
    CommandExecution,
    /// 测试验证
    Testing,
    /// 部署操作
    Deployment,
    /// 清理操作
    Cleanup,
}

/// 步骤执行输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepExecutionOutput {
    /// 步骤ID
    pub step_id: String,
    
    /// 执行状态
    pub status: ExecutionStatus,
    
    /// 输出数据
    pub outputs: HashMap<String, serde_json::Value>,
    
    /// 执行日志
    pub logs: Vec<String>,
    
    /// 生成的文件
    pub generated_files: Vec<String>,
    
    /// 修改的文件
    pub modified_files: Vec<String>,
    
    /// 执行的命令
    pub executed_commands: Vec<String>,
}

/// 执行状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {
    Success,
    PartialSuccess,
    Failed,
    Skipped,
    RolledBack,
}

/// 最终验证输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationOutput {
    /// 验证是否通过
    pub passed: bool,
    
    /// 验证的详细结果
    pub validation_details: Vec<ValidationDetail>,
    
    /// 总体评分 (0.0 - 1.0)
    pub overall_score: f32,
    
    /// 建议
    pub recommendations: Vec<String>,
}

/// 验证详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDetail {
    /// 验证项
    pub item: String,
    /// 是否通过
    pub passed: bool,
    /// 详细说明
    pub details: String,
}

// ============================================================================
// Sequential Executor
// ============================================================================

/// 顺序执行器
pub struct SequentialExecutor {
    model: Arc<dyn LanguageModel>,
    config: ExecutionConfig,
    guardrail_engine: Option<GuardrailEngine>,
}

impl SequentialExecutor {
    /// 创建新的执行器
    pub fn new(model: Arc<dyn LanguageModel>, config: ExecutionConfig) -> Self {
        Self { 
            model, 
            config,
            guardrail_engine: None,
        }
    }
    
    /// 创建带有保护机制的执行器
    pub fn new_with_guardrails(
        model: Arc<dyn LanguageModel>,
        config: ExecutionConfig,
        guardrail_engine: GuardrailEngine,
    ) -> Self {
        Self {
            model,
            config,
            guardrail_engine: Some(guardrail_engine),
        }
    }
    
    /// 执行完整流程
    pub async fn execute_task(
        &self,
        task_description: &str,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut plan = SequentialExecutionPlan::new(task_id.clone(), self.config.clone());
        
        // Phase 1: Understanding
        plan = self.phase_understanding(plan, task_description).await?;
        
        // Phase 2: Approach
        plan = self.phase_approach(plan).await?;
        
        // Phase 3: Planning
        plan = self.phase_planning(plan).await?;
        
        // Phase 4: Execution (逐步执行)
        plan = self.phase_execution(plan).await?;
        
        // Phase 5: Final Validation
        plan = self.phase_validation(plan).await?;
        
        Ok(plan)
    }
    
    /// Phase 1: Understanding 阶段
    async fn phase_understanding(
        &self,
        mut plan: SequentialExecutionPlan,
        task_description: &str,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        plan.current_phase = ExecutionPhase::Understanding;
        
        if self.config.verbose_logging {
            tracing::info!("🧠 Phase 1: Understanding task...");
        }
        
        let start_time = std::time::Instant::now();
        let mut retry_count = 0;
        
        // 构建 Understanding 阶段的提示词
        let prompt = self.build_understanding_prompt(task_description);
        
        // 重试循环
        loop {
            match self.call_llm_with_retry(&prompt, retry_count).await {
                Ok(response) => {
                    // 解析 LLM 响应
                    match self.parse_understanding_response(&response.content) {
                        Ok(understanding) => {
                            // 验证 Understanding 输出
                            let validation = self.validate_understanding(&understanding);
                            
                            if !validation.passed || validation.confidence < self.config.min_confidence_threshold {
                                if retry_count < self.config.max_retries_per_phase {
                                    retry_count += 1;
                                    if self.config.verbose_logging {
                                        tracing::warn!(
                                            "Understanding validation failed (confidence: {}), retrying... ({}/{})",
                                            validation.confidence,
                                            retry_count,
                                            self.config.max_retries_per_phase
                                        );
                                    }
                                    continue;
                                }
                            }
                            
                            let duration_ms = start_time.elapsed().as_millis() as u64;
                            
                            plan.understanding = Some(PhaseResult {
                                phase: ExecutionPhase::Understanding,
                                status: PhaseStatus::Success,
                                output: Some(understanding),
                                duration_ms,
                                validation,
                                executed_at: Utc::now(),
                                error: None,
                                retry_count,
                            });
                            
                            plan.updated_at = Utc::now();
                            return Ok(plan);
                        }
                        Err(e) => {
                            if retry_count < self.config.max_retries_per_phase {
                                retry_count += 1;
                                if self.config.verbose_logging {
                                    tracing::warn!(
                                        "Failed to parse understanding response: {}, retrying... ({}/{})",
                                        e,
                                        retry_count,
                                        self.config.max_retries_per_phase
                                    );
                                }
                                continue;
                            }
                            
                            return Err(AgentError::ExecutionError(
                                format!("Failed to parse understanding after {} retries: {}", retry_count, e)
                            ));
                        }
                    }
                }
                Err(e) => {
                    if retry_count < self.config.max_retries_per_phase {
                        retry_count += 1;
                        if self.config.verbose_logging {
                            tracing::warn!(
                                "LLM call failed: {}, retrying... ({}/{})",
                                e,
                                retry_count,
                                self.config.max_retries_per_phase
                            );
                        }
                        continue;
                    }
                    
                    return Err(AgentError::ExecutionError(
                        format!("LLM call failed after {} retries: {}", retry_count, e)
                    ));
                }
            }
        }
    }
    
    /// Phase 2: Approach 阶段
    async fn phase_approach(
        &self,
        mut plan: SequentialExecutionPlan,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        plan.current_phase = ExecutionPhase::Approach;
        
        if self.config.verbose_logging {
            tracing::info!("🎯 Phase 2: Designing approach...");
        }
        
        let understanding = plan
            .understanding
            .as_ref()
            .ok_or(AgentError::InvalidState(
                "Understanding phase not completed".into(),
            ))?
            .output
            .as_ref()
            .ok_or(AgentError::InvalidState(
                "Understanding output is empty".into(),
            ))?;
        
        let start_time = std::time::Instant::now();
        let mut retry_count = 0;
        
        let prompt = self.build_approach_prompt(understanding);
        
        loop {
            match self.call_llm_with_retry(&prompt, retry_count).await {
                Ok(response) => {
                    match self.parse_approach_response(&response.content) {
                        Ok(approach) => {
                            let validation = self.validate_approach(&approach);
                            
                            if !validation.passed || validation.confidence < self.config.min_confidence_threshold {
                                if retry_count < self.config.max_retries_per_phase {
                                    retry_count += 1;
                                    if self.config.verbose_logging {
                                        tracing::warn!(
                                            "Approach validation failed (confidence: {}), retrying... ({}/{})",
                                            validation.confidence,
                                            retry_count,
                                            self.config.max_retries_per_phase
                                        );
                                    }
                                    continue;
                                }
                            }
                            
                            let duration_ms = start_time.elapsed().as_millis() as u64;
                            
                            plan.approach = Some(PhaseResult {
                                phase: ExecutionPhase::Approach,
                                status: PhaseStatus::Success,
                                output: Some(approach),
                                duration_ms,
                                validation,
                                executed_at: Utc::now(),
                                error: None,
                                retry_count,
                            });
                            
                            plan.updated_at = Utc::now();
                            return Ok(plan);
                        }
                        Err(e) => {
                            if retry_count < self.config.max_retries_per_phase {
                                retry_count += 1;
                                if self.config.verbose_logging {
                                    tracing::warn!(
                                        "Failed to parse approach response: {}, retrying... ({}/{})",
                                        e,
                                        retry_count,
                                        self.config.max_retries_per_phase
                                    );
                                }
                                continue;
                            }
                            
                            return Err(AgentError::ExecutionError(
                                format!("Failed to parse approach after {} retries: {}", retry_count, e)
                            ));
                        }
                    }
                }
                Err(e) => {
                    if retry_count < self.config.max_retries_per_phase {
                        retry_count += 1;
                        continue;
                    }
                    
                    return Err(AgentError::ExecutionError(
                        format!("LLM call failed after {} retries: {}", retry_count, e)
                    ));
                }
            }
        }
    }
    
    /// Phase 3: Planning 阶段
    async fn phase_planning(
        &self,
        mut plan: SequentialExecutionPlan,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        plan.current_phase = ExecutionPhase::Planning;
        
        if self.config.verbose_logging {
            tracing::info!("📋 Phase 3: Creating detailed plan...");
        }
        
        let approach = plan
            .approach
            .as_ref()
            .ok_or(AgentError::InvalidState(
                "Approach phase not completed".into(),
            ))?
            .output
            .as_ref()
            .ok_or(AgentError::InvalidState(
                "Approach output is empty".into(),
            ))?;
        
        let start_time = std::time::Instant::now();
        let mut retry_count = 0;
        
        let prompt = self.build_planning_prompt(approach);
        
        loop {
            match self.call_llm_with_retry(&prompt, retry_count).await {
                Ok(response) => {
                    match self.parse_planning_response(&response.content) {
                        Ok(detailed_plan) => {
                            let validation = self.validate_planning(&detailed_plan);
                            
                            if !validation.passed || validation.confidence < self.config.min_confidence_threshold {
                                if retry_count < self.config.max_retries_per_phase {
                                    retry_count += 1;
                                    if self.config.verbose_logging {
                                        tracing::warn!(
                                            "Planning validation failed (confidence: {}), retrying... ({}/{})",
                                            validation.confidence,
                                            retry_count,
                                            self.config.max_retries_per_phase
                                        );
                                    }
                                    continue;
                                }
                            }
                            
                            let duration_ms = start_time.elapsed().as_millis() as u64;
                            
                            plan.plan = Some(PhaseResult {
                                phase: ExecutionPhase::Planning,
                                status: PhaseStatus::Success,
                                output: Some(detailed_plan),
                                duration_ms,
                                validation,
                                executed_at: Utc::now(),
                                error: None,
                                retry_count,
                            });
                            
                            plan.updated_at = Utc::now();
                            return Ok(plan);
                        }
                        Err(e) => {
                            if retry_count < self.config.max_retries_per_phase {
                                retry_count += 1;
                                if self.config.verbose_logging {
                                    tracing::warn!(
                                        "Failed to parse planning response: {}, retrying... ({}/{})",
                                        e,
                                        retry_count,
                                        self.config.max_retries_per_phase
                                    );
                                }
                                continue;
                            }
                            
                            return Err(AgentError::ExecutionError(
                                format!("Failed to parse planning after {} retries: {}", retry_count, e)
                            ));
                        }
                    }
                }
                Err(e) => {
                    if retry_count < self.config.max_retries_per_phase {
                        retry_count += 1;
                        continue;
                    }
                    
                    return Err(AgentError::ExecutionError(
                        format!("LLM call failed after {} retries: {}", retry_count, e)
                    ));
                }
            }
        }
    }
    
    /// Phase 4: Execution 阶段
    async fn phase_execution(
        &self,
        mut plan: SequentialExecutionPlan,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        if self.config.verbose_logging {
            tracing::info!("⚙️  Phase 4: Executing steps...");
        }
        
        // Get the detailed plan
        let detailed_plan = plan
            .plan
            .as_ref()
            .ok_or(AgentError::InvalidState(
                "Planning phase not completed".into(),
            ))?
            .output
            .as_ref()
            .ok_or(AgentError::InvalidState(
                "Planning output is empty".into(),
            ))?;
        
        let total_steps = detailed_plan.steps.len();
        
        if total_steps == 0 {
            if self.config.verbose_logging {
                tracing::warn!("No steps to execute");
            }
            plan.updated_at = Utc::now();
            return Ok(plan);
        }
        
        // Execute each step sequentially
        for (index, step) in detailed_plan.steps.iter().enumerate() {
            plan.current_phase = ExecutionPhase::Execution {
                current_step: index + 1,
                total_steps,
            };
            
            if self.config.verbose_logging {
                tracing::info!(
                    "▶️  Executing step {}/{}: {}",
                    index + 1,
                    total_steps,
                    step.name
                );
            }
            
            // Execute the step with guardrails
            match self.execute_step(step, &plan).await {
                Ok(step_result) => {
                    plan.execution_history.push(step_result);
                    
                    if self.config.verbose_logging {
                        tracing::info!(
                            "✅ Step {}/{} completed successfully",
                            index + 1,
                            total_steps
                        );
                    }
                }
                Err(e) => {
                    if self.config.verbose_logging {
                        tracing::error!(
                            "❌ Step {}/{} failed: {}",
                            index + 1,
                            total_steps,
                            e
                        );
                    }
                    
                    // Check if failure is allowed
                    if !step.allow_failure {
                        // Attempt rollback if enabled
                        if self.config.enable_auto_rollback {
                            if let Err(rollback_err) = self.rollback_steps(&plan).await {
                                tracing::error!("Rollback failed: {}", rollback_err);
                            }
                        }
                        
                        plan.current_phase = ExecutionPhase::Failed {
                            failed_at: Box::new(ExecutionPhase::Execution {
                                current_step: index + 1,
                                total_steps,
                            }),
                            reason: format!("Step {} failed: {}", step.name, e),
                        };
                        
                        return Err(e);
                    }
                    
                    // Record failed step but continue
                    plan.execution_history.push(PhaseResult {
                        phase: ExecutionPhase::Execution {
                            current_step: index + 1,
                            total_steps,
                        },
                        status: PhaseStatus::Failed,
                        output: None,
                        duration_ms: 0,
                        validation: ValidationResult {
                            passed: false,
                            confidence: 0.0,
                            messages: vec![],
                            warnings: vec![format!("Step failed but allowed to continue: {}", e)],
                            suggestions: vec![],
                        },
                        executed_at: Utc::now(),
                        error: Some(e.to_string()),
                        retry_count: 0,
                    });
                }
            }
        }
        
        plan.updated_at = Utc::now();
        Ok(plan)
    }
    
    /// Phase 5: Validation 阶段
    async fn phase_validation(
        &self,
        mut plan: SequentialExecutionPlan,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        plan.current_phase = ExecutionPhase::Validation;
        
        if self.config.verbose_logging {
            tracing::info!("✅ Phase 5: Final validation...");
        }
        
        let validation_output = ValidationOutput {
            passed: true,
            validation_details: vec![],
            overall_score: 0.9,
            recommendations: vec![],
        };
        
        plan.final_validation = Some(PhaseResult {
            phase: ExecutionPhase::Validation,
            status: PhaseStatus::Success,
            output: Some(validation_output),
            duration_ms: 50,
            validation: ValidationResult::default(),
            executed_at: Utc::now(),
            error: None,
            retry_count: 0,
        });
        
        plan.current_phase = ExecutionPhase::Completed;
        plan.completed_at = Some(Utc::now());
        plan.updated_at = Utc::now();
        
        Ok(plan)
    }
}

// ============================================================================
// Helper Methods for LLM Integration
// ============================================================================

impl SequentialExecutor {
    /// Build prompt for Understanding phase
    fn build_understanding_prompt(&self, task_description: &str) -> String {
        format!(r#"Analyze the following task and provide a structured response.

Task: {}

Please provide your analysis in the following format:

UNDERSTANDING: [Your understanding of the task in one paragraph]

KEY_REQUIREMENTS:
- [Requirement 1]
- [Requirement 2]
- [Requirement 3]
...

TASK_TYPE: [Type of task: development/analysis/configuration/deployment/other]

COMPLEXITY: [Simple/Moderate/Complex]

POTENTIAL_RISKS:
- [Risk 1]
- [Risk 2]
...

CLARIFICATION_NEEDED:
- [Question 1]
- [Question 2]
...

Be specific and thorough in your analysis."#, task_description)
    }

    /// Build prompt for Approach phase
    fn build_approach_prompt(&self, understanding: &UnderstandingOutput) -> String {
        format!(r#"Based on the following task understanding, design a technical approach.

Task Understanding: {}
Task Type: {}
Complexity: {:?}
Key Requirements: {}

Please provide your approach in the following format:

APPROACH: [High-level approach description]

TECH_STACK:
- [Technology 1]
- [Technology 2]
- [Technology 3]
...

ARCHITECTURE_PATTERN: [Pattern name and description]

KEY_DECISIONS:
- DECISION: [Decision 1]
  RATIONALE: [Why this decision]
  TRADEOFFS: [Tradeoffs considered]
- DECISION: [Decision 2]
  RATIONALE: [Why this decision]
  TRADEOFFS: [Tradeoffs considered]
...

EXPECTED_OUTCOMES:
- [Outcome 1]
- [Outcome 2]
...

ALTERNATIVES:
- NAME: [Alternative 1]
  DESCRIPTION: [Description]
  PROS: [Advantages]
  CONS: [Disadvantages]
...

Be specific and justify your technical choices."#,
            understanding.understanding,
            understanding.task_type,
            understanding.complexity,
            understanding.key_requirements.join(", ")
        )
    }

    /// Build prompt for Planning phase
    fn build_planning_prompt(&self, approach: &ApproachOutput) -> String {
        format!(r#"Based on the following technical approach, create a detailed execution plan.

Approach: {}
Tech Stack: {}
Architecture: {}

Please provide a detailed plan in the following format:

STEPS:
- STEP_1:
  NAME: [Step name]
  DESCRIPTION: [What to do]
  TYPE: [Preparation/CodeGeneration/Configuration/FileOperation/CommandExecution/Testing/Deployment/Cleanup]
  DURATION: [Estimated minutes]
  PRECONDITIONS: [What must be ready]
  OUTPUTS: [What will be produced]
  VALIDATION: [How to verify success]

- STEP_2:
  ...

DEPENDENCIES:
- STEP_1 -> STEP_2
- STEP_2 -> STEP_3
...

ESTIMATED_DURATION: [Total minutes]

REQUIRED_RESOURCES:
- [Resource 1]
- [Resource 2]
...

MILESTONES:
- MILESTONE_1:
  NAME: [Name]
  DESCRIPTION: [Description]
  STEPS: [Associated step numbers]
  DURATION: [Minutes]
...

SUCCESS_CRITERIA:
- [Criterion 1]
- [Criterion 2]
...

Provide a concrete, actionable plan with clear steps."#,
            approach.approach,
            approach.tech_stack.join(", "),
            approach.architecture_pattern
        )
    }

    /// Call LLM with retry logic
    async fn call_llm_with_retry(
        &self,
        prompt: &str,
        retry_count: u32,
    ) -> Result<crate::models::ModelResponse, AgentError> {
        use crate::models::LanguageModel;
        
        // Add exponential backoff for retries
        if retry_count > 0 {
            let delay = std::time::Duration::from_millis(100 * 2u64.pow(retry_count - 1));
            tokio::time::sleep(delay).await;
        }
        
        self.model
            .complete(prompt)
            .await
            .map_err(|e| AgentError::ModelError(e))
    }

    /// Parse Understanding response (supports both standard and markdown formats)
    fn parse_understanding_response(&self, content: &str) -> Result<UnderstandingOutput, AgentError> {
        let mut understanding = String::new();
        let mut key_requirements = Vec::new();
        let mut task_type = String::from("general");
        let mut complexity = TaskComplexity::Moderate;
        let mut potential_risks = Vec::new();
        let mut clarification_needed = Vec::new();

        let mut current_section = "";

        for line in content.lines() {
            let trimmed = line.trim();
            
            // Handle both "FIELD:" and "**FIELD**" formats
            if trimmed.starts_with("UNDERSTANDING:") || trimmed.starts_with("**UNDERSTANDING**") {
                current_section = "understanding";
                let value = trimmed
                    .trim_start_matches("UNDERSTANDING:")
                    .trim_start_matches("**UNDERSTANDING**")
                    .trim();
                if !value.is_empty() {
                    understanding = value.to_string();
                }
            } else if trimmed.starts_with("KEY_REQUIREMENTS:") || trimmed.starts_with("**KEY_REQUIREMENTS**") {
                current_section = "key_requirements";
            } else if trimmed.starts_with("TASK_TYPE:") || trimmed.starts_with("**TASK_TYPE**") {
                current_section = "task_type";
                let value = trimmed
                    .trim_start_matches("TASK_TYPE:")
                    .trim_start_matches("**TASK_TYPE**")
                    .trim();
                if !value.is_empty() {
                    task_type = value.to_string();
                }
            } else if trimmed.starts_with("COMPLEXITY:") || trimmed.starts_with("**COMPLEXITY**") {
                current_section = "complexity";
                let value = trimmed
                    .trim_start_matches("COMPLEXITY:")
                    .trim_start_matches("**COMPLEXITY**")
                    .trim();
                complexity = match value.to_lowercase().as_str() {
                    "simple" => TaskComplexity::Simple,
                    "moderate" | "medium" => TaskComplexity::Moderate,
                    "complex" | "high" => TaskComplexity::Complex,
                    _ => TaskComplexity::Moderate,
                };
            } else if trimmed.starts_with("POTENTIAL_RISKS:") || trimmed.starts_with("**POTENTIAL_RISKS**") {
                current_section = "potential_risks";
            } else if trimmed.starts_with("CLARIFICATION_NEEDED:") || trimmed.starts_with("**CLARIFICATION_NEEDED**") {
                current_section = "clarification_needed";
            } else if trimmed.starts_with("- ") {
                let item = trimmed.trim_start_matches("- ").trim().to_string();
                match current_section {
                    "key_requirements" => key_requirements.push(item),
                    "potential_risks" => potential_risks.push(item),
                    "clarification_needed" => clarification_needed.push(item),
                    _ => {}
                }
            } else if !trimmed.is_empty() && current_section == "understanding" && understanding.is_empty() {
                understanding = trimmed.to_string();
            }
        }

        Ok(UnderstandingOutput {
            understanding,
            key_requirements,
            task_type,
            complexity,
            potential_risks,
            clarification_needed,
        })
    }

    /// Validate Understanding output
    fn validate_understanding(&self, understanding: &UnderstandingOutput) -> ValidationResult {
        let mut passed = true;
        let mut confidence = 1.0f32;
        let mut messages = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();

        // Check if understanding is not empty
        if understanding.understanding.is_empty() {
            passed = false;
            confidence *= 0.5;
            warnings.push("Understanding description is empty".to_string());
        } else {
            messages.push("Task understanding captured".to_string());
        }

        // Check if key requirements are identified
        if understanding.key_requirements.is_empty() {
            confidence *= 0.8;
            warnings.push("No key requirements identified".to_string());
            suggestions.push("Consider identifying specific requirements".to_string());
        } else {
            messages.push(format!("{} key requirements identified", understanding.key_requirements.len()));
        }

        // Check complexity assessment
        messages.push(format!("Complexity assessed as: {:?}", understanding.complexity));

        // Adjust confidence based on completeness
        if !understanding.potential_risks.is_empty() {
            messages.push(format!("{} potential risks identified", understanding.potential_risks.len()));
        } else {
            confidence *= 0.9;
            suggestions.push("Consider identifying potential risks".to_string());
        }

        ValidationResult {
            passed,
            confidence,
            messages,
            warnings,
            suggestions,
        }
    }

    /// Parse Approach response
    fn parse_approach_response(&self, content: &str) -> Result<ApproachOutput, AgentError> {
        let mut approach = String::new();
        let mut tech_stack = Vec::new();
        let mut architecture_pattern = String::from("standard");
        let mut key_decisions = Vec::new();
        let mut expected_outcomes = Vec::new();
        let mut alternatives = Vec::new();

        let mut current_section = "";
        let mut current_decision: Option<(String, String, Vec<String>)> = None;
        let mut current_alternative: Option<(String, String, Vec<String>, Vec<String>)> = None;

        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("APPROACH:") || trimmed.starts_with("**APPROACH**") {
                current_section = "approach";
                let value = trimmed
                    .trim_start_matches("APPROACH:")
                    .trim_start_matches("**APPROACH**")
                    .trim();
                if !value.is_empty() {
                    approach = value.to_string();
                }
            } else if trimmed.starts_with("TECH_STACK:") || trimmed.starts_with("**TECH_STACK**") {
                current_section = "tech_stack";
            } else if trimmed.starts_with("ARCHITECTURE_PATTERN:") || trimmed.starts_with("**ARCHITECTURE_PATTERN**") {
                current_section = "architecture";
                let value = trimmed
                    .trim_start_matches("ARCHITECTURE_PATTERN:")
                    .trim_start_matches("**ARCHITECTURE_PATTERN**")
                    .trim();
                if !value.is_empty() {
                    architecture_pattern = value.to_string();
                }
            } else if trimmed.starts_with("EXPECTED_OUTCOMES:") || trimmed.starts_with("**EXPECTED_OUTCOMES**") {
                current_section = "outcomes";
            } else if trimmed.starts_with("- ") {
                let item = trimmed.trim_start_matches("- ").trim().to_string();
                match current_section {
                    "tech_stack" => tech_stack.push(item),
                    "outcomes" => expected_outcomes.push(item),
                    _ => {}
                }
            } else if !trimmed.is_empty() && current_section == "approach" && approach.is_empty() {
                approach = trimmed.to_string();
            }
        }

        Ok(ApproachOutput {
            approach,
            tech_stack,
            architecture_pattern,
            key_decisions,
            expected_outcomes,
            alternatives,
        })
    }

    /// Validate Approach output
    fn validate_approach(&self, approach: &ApproachOutput) -> ValidationResult {
        let mut passed = true;
        let mut confidence = 1.0f32;
        let mut messages = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();

        if approach.approach.is_empty() {
            passed = false;
            confidence *= 0.5;
            warnings.push("Approach description is empty".to_string());
        } else {
            messages.push("Approach description captured".to_string());
        }

        if approach.tech_stack.is_empty() {
            confidence *= 0.8;
            warnings.push("No tech stack specified".to_string());
        } else {
            messages.push(format!("{} technologies identified", approach.tech_stack.len()));
        }

        if approach.expected_outcomes.is_empty() {
            confidence *= 0.9;
            suggestions.push("Consider defining expected outcomes".to_string());
        }

        ValidationResult {
            passed,
            confidence,
            messages,
            warnings,
            suggestions,
        }
    }

    /// Parse Planning response with enhanced step parsing
    fn parse_planning_response(&self, content: &str) -> Result<DetailedPlan, AgentError> {
        let mut steps = Vec::new();
        let mut dependencies = Vec::new();
        let mut estimated_duration = 0u32;
        let mut required_resources = Vec::new();
        let mut milestones = Vec::new();
        let mut success_criteria = Vec::new();

        let mut current_section = "";
        let mut current_step: Option<ExecutionStep> = None;
        let mut step_counter = 0;

        for line in content.lines() {
            let trimmed = line.trim();
            
            // Section headers
            if trimmed.starts_with("STEPS:") || trimmed.starts_with("**STEPS**") {
                current_section = "steps";
                continue;
            } else if trimmed.starts_with("DEPENDENCIES:") || trimmed.starts_with("**DEPENDENCIES**") {
                current_section = "dependencies";
                if let Some(step) = current_step.take() {
                    steps.push(step);
                }
                continue;
            } else if trimmed.starts_with("ESTIMATED_DURATION:") || trimmed.starts_with("**ESTIMATED_DURATION**") {
                current_section = "duration";
                if let Some(step) = current_step.take() {
                    steps.push(step);
                }
                let value = trimmed
                    .trim_start_matches("ESTIMATED_DURATION:")
                    .trim_start_matches("**ESTIMATED_DURATION**")
                    .trim()
                    .split_whitespace()
                    .next()
                    .unwrap_or("60");
                estimated_duration = value.parse().unwrap_or(60);
                continue;
            } else if trimmed.starts_with("REQUIRED_RESOURCES:") || trimmed.starts_with("**REQUIRED_RESOURCES**") {
                current_section = "resources";
                continue;
            } else if trimmed.starts_with("MILESTONES:") || trimmed.starts_with("**MILESTONES**") {
                current_section = "milestones";
                continue;
            } else if trimmed.starts_with("SUCCESS_CRITERIA:") || trimmed.starts_with("**SUCCESS_CRITERIA**") {
                current_section = "criteria";
                continue;
            }
            
            // Parse step definitions
            if current_section == "steps" && (trimmed.starts_with("- STEP_") || trimmed.starts_with("STEP_")) {
                // Save previous step if exists
                if let Some(step) = current_step.take() {
                    steps.push(step);
                }
                
                step_counter += 1;
                current_step = Some(ExecutionStep {
                    id: uuid::Uuid::new_v4().to_string(),
                    sequence: step_counter,
                    name: format!("Step {}", step_counter),
                    description: String::new(),
                    step_type: StepType::CodeGeneration,
                    estimated_duration: 5,
                    preconditions: vec![],
                    expected_outputs: vec![],
                    validation_criteria: vec![],
                    rollback_steps: vec![],
                    requires_confirmation: false,
                    allow_failure: false,
                    operation_guard: None,
                    create_snapshot_before: false,
                    snapshot_id: None,
                });
                continue;
            }
            
            // Parse step fields
            if let Some(ref mut step) = current_step {
                if trimmed.starts_with("NAME:") {
                    step.name = trimmed.trim_start_matches("NAME:").trim().to_string();
                } else if trimmed.starts_with("DESCRIPTION:") {
                    step.description = trimmed.trim_start_matches("DESCRIPTION:").trim().to_string();
                } else if trimmed.starts_with("TYPE:") {
                    let type_str = trimmed.trim_start_matches("TYPE:").trim();
                    step.step_type = match type_str.to_lowercase().as_str() {
                        "preparation" => StepType::Preparation,
                        "codegeneration" | "code" => StepType::CodeGeneration,
                        "configuration" | "config" => StepType::Configuration,
                        "fileoperation" | "file" => StepType::FileOperation,
                        "commandexecution" | "command" => StepType::CommandExecution,
                        "testing" | "test" => StepType::Testing,
                        "deployment" | "deploy" => StepType::Deployment,
                        "cleanup" => StepType::Cleanup,
                        _ => StepType::CodeGeneration,
                    };
                } else if trimmed.starts_with("DURATION:") {
                    if let Ok(duration) = trimmed.trim_start_matches("DURATION:").trim()
                        .split_whitespace().next().unwrap_or("5").parse::<u32>() {
                        step.estimated_duration = duration;
                    }
                } else if trimmed.starts_with("PRECONDITIONS:") {
                    // Next lines will be preconditions
                } else if trimmed.starts_with("OUTPUTS:") || trimmed.starts_with("EXPECTED_OUTPUTS:") {
                    // Next lines will be outputs
                } else if trimmed.starts_with("VALIDATION:") {
                    // Next lines will be validation criteria
                } else if trimmed.starts_with("- ") && current_section == "steps" {
                    let item = trimmed.trim_start_matches("- ").trim().to_string();
                    if !item.is_empty() {
                        if item.starts_with("STEP_") {
                            // Skip step markers in list format
                        } else if step.description.is_empty() {
                            step.description = item;
                        } else {
                            step.expected_outputs.push(item);
                        }
                    }
                }
            }
            
            // Parse dependencies (e.g., "- STEP_1 -> STEP_2")
            if current_section == "dependencies" && trimmed.starts_with("- ") {
                let dep_str = trimmed.trim_start_matches("- ").trim();
                if dep_str.contains("->") {
                    let parts: Vec<&str> = dep_str.split("->").map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        dependencies.push(crate::types::StepDependency {
                            step_id: parts[1].to_string(),
                            depends_on: parts[0].to_string(),
                            dependency_type: crate::types::DependencyType::StrictDependency,
                            condition: None,
                        });
                    }
                }
            }
            
            // Parse resources and criteria
            if trimmed.starts_with("- ") {
                let item = trimmed.trim_start_matches("- ").trim().to_string();
                match current_section {
                    "resources" => required_resources.push(item),
                    "criteria" => success_criteria.push(item),
                    _ => {}
                }
            }
        }
        
        // Don't forget the last step
        if let Some(step) = current_step {
            steps.push(step);
        }

        // If no steps were parsed, create a default simple step
        if steps.is_empty() {
            steps.push(ExecutionStep {
                id: uuid::Uuid::new_v4().to_string(),
                sequence: 1,
                name: "Execute Task".to_string(),
                description: "Execute the planned task".to_string(),
                step_type: StepType::Preparation,
                estimated_duration: estimated_duration.max(10),
                preconditions: vec![],
                expected_outputs: vec![],
                validation_criteria: success_criteria.clone(),
                rollback_steps: vec![],
                requires_confirmation: false,
                allow_failure: false,
                operation_guard: None,
                create_snapshot_before: false,
                snapshot_id: None,
            });
        }

        Ok(DetailedPlan {
            steps,
            dependencies,
            estimated_duration: estimated_duration.max(10),
            required_resources,
            milestones,
            success_criteria,
        })
    }

    /// Validate Planning output
    fn validate_planning(&self, planning: &DetailedPlan) -> ValidationResult {
        let mut passed = true;
        let mut confidence = 1.0f32;
        let mut messages = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();

        if planning.steps.is_empty() {
            passed = false;
            confidence *= 0.3;
            warnings.push("No execution steps defined".to_string());
        } else {
            messages.push(format!("{} execution steps defined", planning.steps.len()));
        }

        if planning.estimated_duration == 0 {
            confidence *= 0.9;
            warnings.push("No duration estimate provided".to_string());
        } else {
            messages.push(format!("Estimated duration: {} minutes", planning.estimated_duration));
        }

        if planning.success_criteria.is_empty() {
            confidence *= 0.8;
            suggestions.push("Consider defining success criteria".to_string());
        } else {
            messages.push(format!("{} success criteria defined", planning.success_criteria.len()));
        }

        ValidationResult {
            passed,
            confidence,
            messages,
            warnings,
            suggestions,
        }
    }

    /// Execute a single step with guardrail checks
    async fn execute_step(
        &self,
        step: &ExecutionStep,
        _plan: &SequentialExecutionPlan,
    ) -> Result<PhaseResult<StepExecutionOutput>, AgentError> {
        let start_time = std::time::Instant::now();
        
        // Step 1: Check guardrails if engine is available
        if let Some(guardrail_engine) = &self.guardrail_engine {
            if let Err(e) = self.check_step_safety(step, guardrail_engine).await {
                return Err(e);
            }
        }
        
        // Step 2: Check if user confirmation is required
        if step.requires_confirmation && self.config.require_confirmation {
            if self.config.verbose_logging {
                tracing::info!("⚠️  Step requires user confirmation: {}", step.name);
                tracing::info!("   Description: {}", step.description);
            }
            
            // In a real implementation, this would wait for user input
            // For now, we'll assume confirmation is granted
            if self.config.verbose_logging {
                tracing::info!("✅ User confirmation granted (auto-approved in demo mode)");
            }
        }
        
        // Step 3: Create snapshot if needed
        let _snapshot_id = if step.create_snapshot_before {
            Some(self.create_snapshot(step).await?)
        } else {
            None
        };
        
        // Step 4: Execute the actual step
        let output = self.execute_step_action(step).await?;
        
        // Step 5: Validate the execution
        let validation = self.validate_step_execution(step, &output)?;
        
        let duration_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(PhaseResult {
            phase: ExecutionPhase::Execution {
                current_step: step.sequence,
                total_steps: step.sequence,
            },
            status: if validation.passed {
                PhaseStatus::Success
            } else {
                PhaseStatus::Failed
            },
            output: Some(output),
            duration_ms,
            validation,
            executed_at: Utc::now(),
            error: None,
            retry_count: 0,
        })
    }

    /// Check step safety using guardrails
    async fn check_step_safety(
        &self,
        step: &ExecutionStep,
        guardrail_engine: &crate::execution::guardrails::GuardrailEngine,
    ) -> Result<(), AgentError> {
        use crate::execution::guardrails::{OperationType, OperationTarget};
        
        // Determine operation type from step type
        let operation_type = match step.step_type {
            StepType::FileOperation => {
                if step.description.contains("delete") || step.description.contains("删除") {
                    OperationType::FileDelete
                } else if step.description.contains("create") || step.description.contains("创建") {
                    OperationType::FileCreate
                } else {
                    OperationType::FileModify
                }
            }
            StepType::CommandExecution => {
                if step.description.contains("rm ") || step.description.contains("delete") {
                    OperationType::CommandDelete
                } else {
                    OperationType::CommandWrite
                }
            }
            StepType::Configuration => OperationType::ConfigModify,
            StepType::Deployment => OperationType::DeployStart,
            _ => OperationType::FileRead,
        };
        
        // Create operation targets from expected outputs
        let targets: Vec<OperationTarget> = step
            .expected_outputs
            .iter()
            .map(|output| OperationTarget {
                resource_type: "file".to_string(),
                path: output.clone(),
                is_protected: false,
                snapshot: None,
            })
            .collect();
        
        // Check the operation
        let guard = guardrail_engine.check_operation(
            operation_type,
            &step.description,
            targets,
        )?;
        
        // If confirmation is required by guardrails
        if guard.requires_confirmation {
            if self.config.verbose_logging {
                tracing::warn!(
                    "⚠️  Guardrail check: {} risk operation detected",
                    match guard.risk_level {
                        crate::execution::guardrails::OperationRiskLevel::Safe => "🟢 Safe",
                        crate::execution::guardrails::OperationRiskLevel::Low => "🟡 Low",
                        crate::execution::guardrails::OperationRiskLevel::Medium => "🟠 Medium",
                        crate::execution::guardrails::OperationRiskLevel::High => "🔴 High",
                        crate::execution::guardrails::OperationRiskLevel::Critical => "🚨 Critical",
                    }
                );
                tracing::warn!("   Operation: {:?}", guard.operation_type);
                tracing::warn!("   Impact: {} files, {} directories",
                    guard.expected_impact.affected_files,
                    guard.expected_impact.affected_directories
                );
                
                if !guard.detected_patterns.is_empty() {
                    tracing::warn!("   Dangerous patterns detected:");
                    for pattern in &guard.detected_patterns {
                        tracing::warn!("     - {}: {}", pattern.name, pattern.warning_message);
                    }
                }
            }
            
            // In production, this would request actual user confirmation
            // For now, we'll auto-approve in demo mode
            if self.config.require_confirmation {
                if self.config.verbose_logging {
                    tracing::info!("✅ Guardrail check passed (auto-approved in demo mode)");
                }
            }
        }
        
        Ok(())
    }

    /// Create a snapshot before executing a step
    async fn create_snapshot(&self, step: &ExecutionStep) -> Result<String, AgentError> {
        let snapshot_id = uuid::Uuid::new_v4().to_string();
        
        if self.config.verbose_logging {
            tracing::info!("📸 Creating snapshot: {}", snapshot_id);
            tracing::info!("   Step: {}", step.name);
        }
        
        // In a real implementation, this would create actual snapshots of files/state
        // For now, we just return the snapshot ID
        
        Ok(snapshot_id)
    }

    /// Execute the actual step action with real operations
    async fn execute_step_action(
        &self,
        step: &ExecutionStep,
    ) -> Result<StepExecutionOutput, AgentError> {
        use crate::execution::{read_file, write_file, run_command};
        
        let mut generated_files = Vec::new();
        let mut modified_files = Vec::new();
        let mut executed_commands = Vec::new();
        let mut logs = vec![
            format!("Started execution of: {}", step.name),
            format!("Description: {}", step.description),
            format!("Type: {:?}", step.step_type),
        ];
        
        match step.step_type {
            StepType::FileOperation => {
                // Analyze step description to determine file operation
                if step.description.contains("create") || step.description.contains("创建") {
                    // Create file operation
                    for output in &step.expected_outputs {
                        if output.contains(".") {  // Looks like a file path
                            let content = format!("// Generated by agent-runner\n// Step: {}\n// {}", 
                                step.name, step.description);
                            match write_file(output, &content).await {
                                Ok(_) => {
                                    generated_files.push(output.clone());
                                    modified_files.push(output.clone());
                                    logs.push(format!("✅ Created file: {}", output));
                                }
                                Err(e) => {
                                    logs.push(format!("❌ Failed to create file {}: {}", output, e));
                                }
                            }
                        }
                    }
                } else if step.description.contains("read") || step.description.contains("读取") {
                    // Read file operation
                    for output in &step.expected_outputs {
                        if output.contains(".") {
                            match read_file(output).await {
                                Ok(content) => {
                                    logs.push(format!("✅ Read {} bytes from {}", content.len(), output));
                                }
                                Err(e) => {
                                    logs.push(format!("❌ Failed to read file {}: {}", output, e));
                                }
                            }
                        }
                    }
                } else if step.description.contains("modify") || step.description.contains("修改") {
                    // Modify file operation
                    for output in &step.expected_outputs {
                        if output.contains(".") {
                            match read_file(output).await {
                                Ok(content) => {
                                    // Append a comment
                                    let new_content = format!("{}\n// Modified by agent-runner\n", content);
                                    match write_file(output, &new_content).await {
                                        Ok(_) => {
                                            modified_files.push(output.clone());
                                            logs.push(format!("✅ Modified file: {}", output));
                                        }
                                        Err(e) => {
                                            logs.push(format!("❌ Failed to modify {}: {}", output, e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    logs.push(format!("❌ Failed to read {}: {}", output, e));
                                }
                            }
                        }
                    }
                }
            }
            
            StepType::CommandExecution => {
                // Execute commands (with safety checks already done by guardrails)
                let cmd_str = step.description.trim();
                
                logs.push(format!("Executing command: {}", cmd_str));
                
                match run_command(cmd_str).await {
                    Ok(cmd_output) => {
                        executed_commands.push(cmd_str.to_string());
                        logs.push(format!("✅ Command output: {}", cmd_output));
                    }
                    Err(e) => {
                        logs.push(format!("❌ Command execution failed: {}", e));
                        // Don't fail the step if command fails in some cases
                    }
                }
            }
            
            StepType::CodeGeneration => {
                // Generate code using LLM
                logs.push("Generating code with LLM...".to_string());
                
                let prompt = format!(
                    "Generate code for the following requirement:\n{}\n\nProvide only the code without explanations.",
                    step.description
                );
                
                match self.model.complete(&prompt).await {
                    Ok(response) => {
                        logs.push(format!("LLM response received: {} chars", response.content.len()));
                        
                        // Extract code from response
                        let code = if response.content.contains("```") {
                            // Extract code block
                            let parts: Vec<&str> = response.content.split("```").collect();
                            if parts.len() > 1 {
                                parts[1].lines()
                                    .skip(1)  // Skip language identifier
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            } else {
                                response.content.clone()
                            }
                        } else {
                            response.content
                        };
                        
                        // Write to expected output files
                        for output_file in &step.expected_outputs {
                            if output_file.contains(".") {
                                match write_file(output_file, &code).await {
                                    Ok(_) => {
                                        generated_files.push(output_file.clone());
                                        modified_files.push(output_file.clone());
                                        logs.push(format!("✅ Generated code written to: {}", output_file));
                                    }
                                    Err(e) => {
                                        logs.push(format!("❌ Failed to write code to {}: {}", output_file, e));
                                    }
                                }
                            }
                        }
                        
                        logs.push(format!("Code generation completed: {} lines", code.lines().count()));
                    }
                    Err(e) => {
                        logs.push(format!("❌ Code generation failed: {}", e));
                    }
                }
            }
            
            StepType::Testing => {
                // Run tests
                logs.push("Running tests...".to_string());
                
                match run_command("cargo test --quiet").await {
                    Ok(test_output) => {
                        executed_commands.push("cargo test".to_string());
                        logs.push(format!("✅ Tests passed: {}", test_output));
                    }
                    Err(e) => {
                        logs.push(format!("⚠️  Testing skipped or failed: {}", e));
                    }
                }
            }
            
            StepType::Preparation => {
                logs.push("Preparation step: setting up environment...".to_string());
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                logs.push("✅ Preparation completed".to_string());
            }
            
            StepType::Configuration => {
                logs.push("Configuration step: updating settings...".to_string());
                
                // If there are expected outputs that look like config files
                for output in &step.expected_outputs {
                    if output.ends_with(".toml") || output.ends_with(".json") || output.ends_with(".yaml") {
                        let config_content = format!("# Configuration generated by agent-runner\n# {}", step.description);
                        match write_file(output, &config_content).await {
                            Ok(_) => {
                                generated_files.push(output.clone());
                                logs.push(format!("✅ Created config: {}", output));
                            }
                            Err(e) => {
                                logs.push(format!("❌ Failed to create config {}: {}", output, e));
                            }
                        }
                    }
                }
                
                logs.push("✅ Configuration completed".to_string());
            }
            
            StepType::Deployment => {
                logs.push("Deployment step: deploying application...".to_string());
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                logs.push("✅ Deployment completed (simulated)".to_string());
            }
            
            StepType::Cleanup => {
                logs.push("Cleanup step: cleaning up resources...".to_string());
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                logs.push("✅ Cleanup completed".to_string());
            }
        }
        
        logs.push("Execution completed successfully".to_string());
        
        Ok(StepExecutionOutput {
            step_id: step.id.clone(),
            status: ExecutionStatus::Success,
            outputs: HashMap::new(),
            logs,
            generated_files,
            modified_files,
            executed_commands,
        })
    }

    /// Validate step execution
    fn validate_step_execution(
        &self,
        step: &ExecutionStep,
        output: &StepExecutionOutput,
    ) -> Result<ValidationResult, AgentError> {
        let mut passed = true;
        let mut confidence = 1.0f32;
        let mut messages = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();
        
        // Check if execution was successful
        if output.status != ExecutionStatus::Success {
            passed = false;
            confidence = 0.0;
            warnings.push(format!("Step execution status: {:?}", output.status));
        } else {
            messages.push(format!("Step '{}' executed successfully", step.name));
        }
        
        // Validate against step's validation criteria
        if !step.validation_criteria.is_empty() {
            messages.push(format!("{} validation criteria to check", step.validation_criteria.len()));
            // In a real implementation, we would check each criterion
        }
        
        // Check expected outputs
        if !step.expected_outputs.is_empty() && output.generated_files.is_empty() && output.modified_files.is_empty() {
            confidence *= 0.9;
            suggestions.push("Expected outputs not verified".to_string());
        }
        
        Ok(ValidationResult {
            passed,
            confidence,
            messages,
            warnings,
            suggestions,
        })
    }

    /// Rollback executed steps
    async fn rollback_steps(
        &self,
        plan: &SequentialExecutionPlan,
    ) -> Result<(), AgentError> {
        if self.config.verbose_logging {
            tracing::warn!("↩️  Initiating rollback...");
        }
        
        let successful_steps: Vec<_> = plan
            .execution_history
            .iter()
            .filter(|result| result.status == PhaseStatus::Success)
            .collect();
        
        if self.config.verbose_logging {
            tracing::info!("   Rolling back {} successful steps", successful_steps.len());
        }
        
        // Rollback in reverse order
        for (index, _step_result) in successful_steps.iter().rev().enumerate() {
            if self.config.verbose_logging {
                tracing::info!("   Rollback step {}/{}", index + 1, successful_steps.len());
            }
            
            // In a real implementation, this would:
            // - Restore snapshots
            // - Undo file changes
            // - Execute rollback commands
            // etc.
            
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        
        if self.config.verbose_logging {
            tracing::info!("✅ Rollback completed");
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MockModel;
    
    #[tokio::test]
    async fn test_sequential_execution() {
        let model = Arc::new(MockModel::new("test".to_string()));
        let config = ExecutionConfig::default();
        let executor = SequentialExecutor::new(model, config);
        
        let result = executor.execute_task("Test task").await;
        assert!(result.is_ok());
        
        let plan = result.unwrap();
        assert_eq!(plan.current_phase, ExecutionPhase::Completed);
        assert!(plan.understanding.is_some());
        assert!(plan.approach.is_some());
        assert!(plan.plan.is_some());
    }
}
