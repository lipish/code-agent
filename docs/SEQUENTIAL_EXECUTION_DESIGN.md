# Sequential Execution with Error Correction - Design Document

## 概述

基于 OpenAI Codex 的设计理念，本文档提出了一个更加稳定、可靠的顺序执行机制，实现 Understanding → Approach → Plan → Execution 的渐进式任务处理流程。

## 核心设计理念

### 1. **分阶段执行（Phased Execution）**

```
Phase 1: Understanding (任务理解)
    ↓ 验证 & 确认
Phase 2: Approach (方案设计) 
    ↓ 验证 & 细化
Phase 3: Plan (详细计划)
    ↓ 验证 & 分解
Phase 4: Execution (逐步执行)
    ↓ 每个步骤后验证
Phase 5: Validation (整体验证)
```

### 2. **每个阶段都有验证和纠错机制**

- **验证点（Validation Points）**：每个阶段完成后必须通过验证
- **重试机制（Retry Mechanism）**：失败时可以重试
- **回滚能力（Rollback Capability）**：严重错误时可以回退
- **人工介入（Human Intervention）**：关键决策点可以暂停等待确认

## 新的类型系统设计

### 1. ExecutionPhase - 执行阶段枚举

```rust
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
```

### 2. PhaseResult - 阶段执行结果

```rust
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
```

### 3. SequentialExecutionPlan - 顺序执行计划

```rust
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
```

### 4. 各阶段的输出类型

```rust
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
```

## 核心执行流程

### SequentialExecutor - 顺序执行器

```rust
/// 顺序执行器
pub struct SequentialExecutor {
    model: Arc<dyn LanguageModel>,
    config: ExecutionConfig,
    state_store: Arc<dyn StateStore>,
}

impl SequentialExecutor {
    /// 创建新的执行器
    pub fn new(
        model: Arc<dyn LanguageModel>,
        config: ExecutionConfig,
    ) -> Self {
        Self {
            model,
            config,
            state_store: Arc::new(InMemoryStateStore::new()),
        }
    }
    
    /// 执行完整流程
    pub async fn execute_task(
        &self,
        task_description: &str,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        let task_id = generate_task_id();
        let mut plan = SequentialExecutionPlan::new(task_id.clone(), self.config.clone());
        
        // Phase 1: Understanding
        plan = self.phase_understanding(plan, task_description).await?;
        self.save_state(&plan).await?;
        
        // 验证是否需要人工确认
        if self.config.require_confirmation {
            self.wait_for_confirmation(&plan, ExecutionPhase::Understanding).await?;
        }
        
        // Phase 2: Approach
        plan = self.phase_approach(plan).await?;
        self.save_state(&plan).await?;
        
        if self.config.require_confirmation {
            self.wait_for_confirmation(&plan, ExecutionPhase::Approach).await?;
        }
        
        // Phase 3: Planning
        plan = self.phase_planning(plan).await?;
        self.save_state(&plan).await?;
        
        if self.config.require_confirmation {
            self.wait_for_confirmation(&plan, ExecutionPhase::Planning).await?;
        }
        
        // Phase 4: Execution (逐步执行)
        plan = self.phase_execution(plan).await?;
        self.save_state(&plan).await?;
        
        // Phase 5: Final Validation
        plan = self.phase_validation(plan).await?;
        self.save_state(&plan).await?;
        
        Ok(plan)
    }
    
    /// Phase 1: Understanding 阶段
    async fn phase_understanding(
        &self,
        mut plan: SequentialExecutionPlan,
        task_description: &str,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        plan.current_phase = ExecutionPhase::Understanding;
        
        let result = self.execute_phase_with_retry(
            ExecutionPhase::Understanding,
            || async {
                // 调用 LLM 进行任务理解
                let understanding = self.llm_understand_task(task_description).await?;
                
                // 验证理解的质量
                let validation = self.validate_understanding(&understanding)?;
                
                Ok((understanding, validation))
            }
        ).await?;
        
        plan.understanding = Some(result);
        plan.updated_at = Utc::now();
        
        Ok(plan)
    }
    
    /// Phase 2: Approach 阶段
    async fn phase_approach(
        &self,
        mut plan: SequentialExecutionPlan,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        plan.current_phase = ExecutionPhase::Approach;
        
        let understanding = plan.understanding.as_ref()
            .ok_or(AgentError::InvalidState("Understanding phase not completed".into()))?;
        
        let result = self.execute_phase_with_retry(
            ExecutionPhase::Approach,
            || async {
                // 基于理解生成方案
                let approach = self.llm_generate_approach(&understanding.output).await?;
                
                // 验证方案的可行性
                let validation = self.validate_approach(&approach)?;
                
                Ok((approach, validation))
            }
        ).await?;
        
        plan.approach = Some(result);
        plan.updated_at = Utc::now();
        
        Ok(plan)
    }
    
    /// Phase 3: Planning 阶段
    async fn phase_planning(
        &self,
        mut plan: SequentialExecutionPlan,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        plan.current_phase = ExecutionPhase::Planning;
        
        let approach = plan.approach.as_ref()
            .ok_or(AgentError::InvalidState("Approach phase not completed".into()))?;
        
        let result = self.execute_phase_with_retry(
            ExecutionPhase::Planning,
            || async {
                // 生成详细的执行计划
                let detailed_plan = self.llm_generate_detailed_plan(&approach.output).await?;
                
                // 验证计划的完整性
                let validation = self.validate_plan(&detailed_plan)?;
                
                Ok((detailed_plan, validation))
            }
        ).await?;
        
        plan.plan = Some(result);
        plan.updated_at = Utc::now();
        
        Ok(plan)
    }
    
    /// Phase 4: Execution 阶段（逐步执行）
    async fn phase_execution(
        &self,
        mut plan: SequentialExecutionPlan,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        let detailed_plan = plan.plan.as_ref()
            .ok_or(AgentError::InvalidState("Planning phase not completed".into()))?
            .output.as_ref()
            .ok_or(AgentError::InvalidState("Plan output is missing".into()))?;
        
        let total_steps = detailed_plan.steps.len();
        
        // 逐步执行每个步骤
        for (index, step) in detailed_plan.steps.iter().enumerate() {
            plan.current_phase = ExecutionPhase::Execution {
                current_step: index,
                total_steps,
            };
            
            // 检查依赖关系
            if !self.check_dependencies(&step.id, &detailed_plan.dependencies, &plan.execution_history)? {
                return Err(AgentError::ExecutionError(
                    format!("Dependencies not satisfied for step: {}", step.name)
                ));
            }
            
            // 执行单个步骤
            let step_result = self.execute_single_step(step, &plan).await;
            
            // 保存执行历史
            plan.execution_history.push(step_result.clone());
            plan.updated_at = Utc::now();
            self.save_state(&plan).await?;
            
            // 处理执行失败
            match &step_result.status {
                PhaseStatus::Failed if !step.allow_failure => {
                    // 尝试回滚
                    if self.config.enable_auto_rollback {
                        self.rollback_step(step, &mut plan).await?;
                    }
                    
                    return Err(AgentError::ExecutionError(
                        format!("Step failed: {}", step.name)
                    ));
                }
                PhaseStatus::AwaitingConfirmation => {
                    self.wait_for_step_confirmation(&plan, &step.id).await?;
                }
                _ => {}
            }
        }
        
        Ok(plan)
    }
    
    /// Phase 5: Validation 阶段
    async fn phase_validation(
        &self,
        mut plan: SequentialExecutionPlan,
    ) -> Result<SequentialExecutionPlan, AgentError> {
        plan.current_phase = ExecutionPhase::Validation;
        
        let result = self.execute_phase_with_retry(
            ExecutionPhase::Validation,
            || async {
                // 执行最终验证
                let validation = self.validate_execution_results(&plan).await?;
                
                Ok((validation, ValidationResult {
                    passed: true,
                    confidence: 1.0,
                    messages: vec!["Final validation completed".to_string()],
                    warnings: vec![],
                    suggestions: vec![],
                }))
            }
        ).await?;
        
        plan.final_validation = Some(result);
        plan.current_phase = ExecutionPhase::Completed;
        plan.completed_at = Some(Utc::now());
        plan.updated_at = Utc::now();
        
        Ok(plan)
    }
    
    /// 执行单个步骤
    async fn execute_single_step(
        &self,
        step: &ExecutionStep,
        plan: &SequentialExecutionPlan,
    ) -> PhaseResult<StepExecutionOutput> {
        let start_time = Utc::now();
        
        // 执行步骤的具体逻辑
        let (status, output, error) = match &step.step_type {
            StepType::CodeGeneration => {
                self.execute_code_generation(step, plan).await
            }
            StepType::FileOperation => {
                self.execute_file_operation(step, plan).await
            }
            StepType::CommandExecution => {
                self.execute_command(step, plan).await
            }
            StepType::Testing => {
                self.execute_testing(step, plan).await
            }
            // ... 其他步骤类型
            _ => {
                (PhaseStatus::Success, None, None)
            }
        };
        
        let duration = (Utc::now() - start_time).num_milliseconds() as u64;
        
        PhaseResult {
            phase: ExecutionPhase::Execution {
                current_step: step.sequence,
                total_steps: 0,
            },
            status,
            output,
            duration_ms: duration,
            validation: self.validate_step_output(step, &output).unwrap_or_default(),
            executed_at: start_time,
            error,
            retry_count: 0,
        }
    }
    
    /// 带重试的阶段执行
    async fn execute_phase_with_retry<F, Fut, T>(
        &self,
        phase: ExecutionPhase,
        mut execute_fn: F,
    ) -> Result<PhaseResult<T>, AgentError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<(Option<T>, ValidationResult), AgentError>>,
    {
        let mut retry_count = 0;
        
        loop {
            let start_time = Utc::now();
            
            match execute_fn().await {
                Ok((output, validation)) => {
                    let duration = (Utc::now() - start_time).num_milliseconds() as u64;
                    
                    // 检查验证结果
                    if validation.passed && validation.confidence >= self.config.min_confidence_threshold {
                        return Ok(PhaseResult {
                            phase: phase.clone(),
                            status: PhaseStatus::Success,
                            output,
                            duration_ms: duration,
                            validation,
                            executed_at: start_time,
                            error: None,
                            retry_count,
                        });
                    } else if retry_count < self.config.max_retries_per_phase {
                        retry_count += 1;
                        tracing::warn!(
                            "Phase {:?} validation failed (confidence: {}), retrying ({}/{})",
                            phase, validation.confidence, retry_count, self.config.max_retries_per_phase
                        );
                        continue;
                    } else {
                        return Ok(PhaseResult {
                            phase: phase.clone(),
                            status: PhaseStatus::Failed,
                            output,
                            duration_ms: duration,
                            validation,
                            executed_at: start_time,
                            error: Some("Validation confidence too low".to_string()),
                            retry_count,
                        });
                    }
                }
                Err(e) if retry_count < self.config.max_retries_per_phase => {
                    retry_count += 1;
                    tracing::warn!(
                        "Phase {:?} failed: {}, retrying ({}/{})",
                        phase, e, retry_count, self.config.max_retries_per_phase
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
    
    /// 检查步骤依赖是否满足
    fn check_dependencies(
        &self,
        step_id: &str,
        dependencies: &[StepDependency],
        history: &[PhaseResult<StepExecutionOutput>],
    ) -> Result<bool, AgentError> {
        for dep in dependencies {
            if dep.step_id != step_id {
                continue;
            }
            
            // 检查依赖的步骤是否已成功执行
            let dep_executed = history.iter().any(|result| {
                if let Some(output) = &result.output {
                    output.step_id == dep.depends_on && result.status == PhaseStatus::Success
                } else {
                    false
                }
            });
            
            if !dep_executed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

## 实现优势

### 1. 稳定性保证

- ✅ **每个阶段独立验证**：确保质量门控
- ✅ **重试机制**：自动处理临时失败
- ✅ **状态持久化**：支持断点续传
- ✅ **回滚能力**：失败时可以撤销更改

### 2. 可观测性

- ✅ **详细的执行历史**：每个阶段的完整记录
- ✅ **置信度评分**：量化每个决策的可靠性
- ✅ **执行时间追踪**：性能分析和优化
- ✅ **完整的日志链**：便于调试和审计

### 3. 灵活性

- ✅ **可配置的行为**：根据需求调整重试、确认等
- ✅ **人工介入点**：关键决策可以暂停等待
- ✅ **部分执行**：支持从特定阶段开始
- ✅ **条件分支**：基于结果动态调整

### 4. 纠错机制

- ✅ **自动重试**：临时性失败自动恢复
- ✅ **降级策略**：主方案失败时使用备选方案
- ✅ **智能回滚**：最小化失败影响
- ✅ **错误诊断**：提供详细的失败原因和建议

## 对比传统方式

### 传统方式（一次性执行）

```
Request → [LLM生成Plan] → [一次性执行所有步骤] → Result
                ↓
         容易出错，难以恢复
```

### 新方式（分阶段执行）

```
Request
  ↓ validate
Understanding  (可重试，可确认)
  ↓ validate
Approach       (可重试，可确认)
  ↓ validate
Detailed Plan  (可重试，可确认)
  ↓ validate
Step 1         (可重试，可回滚)
  ↓ validate
Step 2         (可重试，可回滚)
  ↓ validate
...
  ↓ validate
Final Validation
  ↓
Result         (完整的执行历史和诊断信息)
```

## 使用示例

```rust
use agent_runner::execution::{SequentialExecutor, ExecutionConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置执行器
    let config = ExecutionConfig {
        max_retries_per_phase: 3,
        require_confirmation: true,  // 关键点需要人工确认
        min_confidence_threshold: 0.8,
        enable_auto_rollback: true,
        verbose_logging: true,
    };
    
    let model = create_llm_model()?;
    let executor = SequentialExecutor::new(model, config);
    
    // 执行任务
    let plan = executor.execute_task(
        "创建一个用户认证系统，包括注册、登录、密码重置功能"
    ).await?;
    
    // 检查执行结果
    match plan.current_phase {
        ExecutionPhase::Completed => {
            println!("✅ 任务成功完成！");
            println!("总耗时: {} 分钟", plan.total_duration_minutes());
            println!("执行了 {} 个步骤", plan.execution_history.len());
        }
        ExecutionPhase::Failed { reason, .. } => {
            println!("❌ 任务执行失败: {}", reason);
            println!("已完成 {} 个步骤", plan.completed_steps_count());
            
            // 查看失败详情
            if let Some(failed_step) = plan.find_failed_step() {
                println!("失败的步骤: {}", failed_step.name);
                println!("失败原因: {:?}", failed_step.error);
            }
        }
        _ => {
            println!("⏸️  任务暂停，等待继续执行");
        }
    }
    
    Ok(())
}
```

## 下一步实施计划

### Phase 1: 核心类型定义（1-2天）
- [ ] 定义 `ExecutionPhase` 和相关枚举
- [ ] 定义 `PhaseResult` 和验证类型
- [ ] 定义 `SequentialExecutionPlan` 结构

### Phase 2: 基础执行器（3-5天）
- [ ] 实现 `SequentialExecutor` 框架
- [ ] 实现 Understanding 阶段
- [ ] 实现 Approach 阶段
- [ ] 实现 Planning 阶段

### Phase 3: 步骤执行引擎（5-7天）
- [ ] 实现步骤依赖检查
- [ ] 实现逐步执行逻辑
- [ ] 实现回滚机制
- [ ] 实现状态持久化

### Phase 4: 验证和纠错（3-5天）
- [ ] 实现各阶段的验证逻辑
- [ ] 实现重试机制
- [ ] 实现置信度评估
- [ ] 实现错误诊断

### Phase 5: 测试和优化（5-7天）
- [ ] 单元测试
- [ ] 集成测试
- [ ] 性能优化
- [ ] 文档完善

## 总结

这个设计参考了 Codex 的渐进式执行和验证理念，将复杂的任务执行分解为可管理、可验证、可恢复的小阶段。每个阶段都有明确的输入、输出和验证标准，确保整个流程的稳定性和可靠性。

关键创新点：
1. **分阶段执行**：Understanding → Approach → Plan → Execution
2. **每步验证**：每个阶段都有质量门控
3. **纠错机制**：重试、回滚、降级策略
4. **可观测性**：完整的执行历史和诊断信息
5. **人机协作**：关键决策点支持人工介入
