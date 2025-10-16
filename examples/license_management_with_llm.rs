//! License Management System - Real LLM Integration Demo
//!
//! 这个示例演示如何使用真实的 LLM（智谱 GLM-4）来创建
//! 代理商和用户的 License 管理系统，并显示所有 prompt 和 response。

use agent_runner::execution::{
    SequentialExecutor,
    ExecutionConfig,
    ExecutionPhase,
};
use agent_runner::config::{ModelConfig, ModelProvider};
use agent_runner::models::LlmModel;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志 - 使用 DEBUG 级别以查看所有详细信息
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("🔐 License 管理系统 - 真实 LLM 集成演示");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("使用智谱 GLM-4 模型");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    // 创建 LLM 配置
    let api_key = "d2a0da2b02954b1f91a0a4ec16d4521b.GA2Tz9sF9kt4zVd3";
    
    println!("🔧 配置 LLM 连接器...");
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),  // 使用 flash 版本，速度更快
        api_key: Some(api_key.to_string()),
        endpoint: None,
        max_tokens: 2000,  // 降低 token 限制以加快响应
        temperature: 0.7,
    };

    println!("  提供商: {:?}", model_config.provider);
    println!("  模型: {}", model_config.model_name);
    println!("  Max Tokens: {}", model_config.max_tokens);
    println!("  Temperature: {}", model_config.temperature);
    println!();

    // 创建 LLM 模型
    let llm_model = LlmModel::from_config(model_config)?;
    let model: Arc<dyn agent_runner::models::LanguageModel> = Arc::new(llm_model);
    
    // 配置执行器 - 启用详细日志
    let config = ExecutionConfig {
        max_retries_per_phase: 2,
        require_confirmation: false,
        min_confidence_threshold: 0.7,
        enable_auto_rollback: true,
        verbose_logging: true,
    };

    println!("⚙️  创建 Sequential Executor...");
    println!("  最大重试次数: {}", config.max_retries_per_phase);
    println!("  置信度阈值: {}", config.min_confidence_threshold);
    println!("  详细日志: {}", config.verbose_logging);
    println!();

    let executor = SequentialExecutor::new(model, config);
    
    // 定义任务
    let task = r#"创建一个代理商和用户的 License 管理系统，包含以下功能：

1. 系统架构设计
   - 使用 Rust 语言
   - 采用模块化设计
   - 支持多种数据库（SQLite, PostgreSQL）

2. 核心功能模块
   - License 生成器（支持不同类型：Trial, Professional, Enterprise）
   - License 验证器（检查有效性、过期时间、使用次数）
   - 代理商管理（创建代理商、分配 License 配额）
   - 用户管理（用户注册、License 激活、使用统计）

3. 数据模型
   - Agent（代理商）: id, name, email, license_quota, created_at
   - User（用户）: id, name, email, agent_id, created_at
   - License（许可证）: id, key, type, user_id, agent_id, expires_at, activated_at, max_uses, current_uses

4. API 接口
   - POST /api/agents - 创建代理商
   - GET /api/agents/{id} - 查询代理商
   - POST /api/agents/{id}/licenses - 为代理商生成 License
   - POST /api/users - 创建用户
   - POST /api/licenses/activate - 激活 License
   - POST /api/licenses/validate - 验证 License
   - GET /api/licenses/{key}/status - 查询 License 状态

5. 实现步骤
   - 创建项目结构
   - 定义数据模型
   - 实现 License 生成算法
   - 实现 License 验证逻辑
   - 实现数据库操作
   - 实现 HTTP API
   - 添加测试
   - 创建配置文件

请详细规划并执行这个任务。"#;
    
    println!("📝 任务描述:");
    println!("{}", "=".repeat(70));
    println!("{}", task);
    println!("{}", "=".repeat(70));
    println!();
    
    println!("🚀 开始执行任务...");
    println!("{}", "=".repeat(70));
    println!();
    
    // 执行任务
    let plan = executor.execute_task(task).await?;
    
    println!();
    println!("{}", "=".repeat(70));
    println!("🎉 任务执行完成！");
    println!("{}", "=".repeat(70));
    println!();
    
    // 打印详细结果
    print_execution_summary(&plan);
    print_phase_details(&plan);
    print_execution_details(&plan);
    
    Ok(())
}

/// 打印执行摘要
fn print_execution_summary(plan: &agent_runner::execution::SequentialExecutionPlan) {
    println!();
    println!("📊 执行结果摘要");
    println!("═══════════════════════════════════════════════════════════════════");
    
    // 状态
    let (status_icon, status_text) = match &plan.current_phase {
        ExecutionPhase::Completed => ("✅", "成功完成"),
        ExecutionPhase::Failed { failed_at, reason } => {
            println!("❌ 失败阶段: {:?}", failed_at);
            println!("   失败原因: {}", reason);
            ("❌", "执行失败")
        },
        _ => ("⏸️ ", "执行中"),
    };
    println!("{} 状态: {}", status_icon, status_text);
    
    // 执行时间
    let total_duration = plan.updated_at.signed_duration_since(plan.started_at);
    println!("⏱️  总耗时: {:.2} 秒", total_duration.num_milliseconds() as f64 / 1000.0);
    
    // 统计信息
    let total_steps = plan.execution_history.len();
    let successful_steps = plan.execution_history.iter()
        .filter(|r| r.status == agent_runner::execution::PhaseStatus::Success)
        .count();
    
    println!("📈 步骤统计:");
    println!("   总步骤数: {}", total_steps);
    println!("   成功: {}", successful_steps);
    println!("   失败: {}", total_steps - successful_steps);
    println!();
}

/// 打印各阶段详细信息（包含 LLM 交互）
fn print_phase_details(plan: &agent_runner::execution::SequentialExecutionPlan) {
    println!();
    println!("🔍 各阶段详细信息");
    println!("═══════════════════════════════════════════════════════════════════");
    
    // Phase 1: Understanding
    if let Some(understanding) = &plan.understanding {
        println!();
        println!("🧠 阶段 1: 任务理解 (Understanding)");
        println!("─────────────────────────────────────────");
        println!("状态: {:?}", understanding.status);
        println!("置信度: {:.2}", understanding.validation.confidence);
        println!("耗时: {} ms", understanding.duration_ms);
        println!("重试次数: {}", understanding.retry_count);
        
        if let Some(output) = &understanding.output {
            println!();
            println!("📤 LLM 响应:");
            println!("─────────────────────────────────────────");
            println!("任务类型: {}", output.task_type);
            println!("复杂度: {:?}", output.complexity);
            println!();
            println!("理解内容:");
            println!("{}", output.understanding);
            
            if !output.key_requirements.is_empty() {
                println!();
                println!("关键需求 ({} 项):", output.key_requirements.len());
                for (idx, req) in output.key_requirements.iter().enumerate() {
                    println!("  {}. {}", idx + 1, req);
                }
            }
        }
        
        if !understanding.validation.messages.is_empty() {
            println!();
            println!("✅ 验证信息:");
            for msg in &understanding.validation.messages {
                println!("  • {}", msg);
            }
        }
        
        if !understanding.validation.warnings.is_empty() {
            println!();
            println!("⚠️  警告:");
            for warn in &understanding.validation.warnings {
                println!("  • {}", warn);
            }
        }
    }
    
    // Phase 2: Approach
    if let Some(approach) = &plan.approach {
        println!();
        println!("🎯 阶段 2: 方案设计 (Approach)");
        println!("─────────────────────────────────────────");
        println!("状态: {:?}", approach.status);
        println!("置信度: {:.2}", approach.validation.confidence);
        println!("耗时: {} ms", approach.duration_ms);
        println!("重试次数: {}", approach.retry_count);
        
        if let Some(output) = &approach.output {
            println!();
            println!("📤 LLM 响应:");
            println!("─────────────────────────────────────────");
            println!("方法描述:");
            println!("{}", output.approach);
            println!();
            println!("架构模式: {}", output.architecture_pattern);
            
            if !output.tech_stack.is_empty() {
                println!();
                println!("技术栈 ({} 项):", output.tech_stack.len());
                for (idx, tech) in output.tech_stack.iter().enumerate() {
                    println!("  {}. {}", idx + 1, tech);
                }
            }
            
            if !output.key_decisions.is_empty() {
                println!();
                println!("关键决策 ({} 项):", output.key_decisions.len());
                for (idx, decision) in output.key_decisions.iter().enumerate() {
                    println!();
                    println!("  决策 {}: {}", idx + 1, decision.decision);
                    println!("  理由: {}", decision.rationale);
                    if !decision.tradeoffs.is_empty() {
                        println!("  权衡考虑:");
                        for tradeoff in &decision.tradeoffs {
                            println!("    - {}", tradeoff);
                        }
                    }
                }
            }
            
            if !output.alternatives.is_empty() {
                println!();
                println!("备选方案 ({} 项):", output.alternatives.len());
                for (idx, alt) in output.alternatives.iter().enumerate() {
                    println!();
                    println!("  方案 {}: {}", idx + 1, alt.name);
                    println!("  描述: {}", alt.description);
                    println!("  优点: {}", alt.pros.join(", "));
                    println!("  缺点: {}", alt.cons.join(", "));
                }
            }
        }
    }
    
    // Phase 3: Planning
    if let Some(planning) = &plan.plan {
        println!();
        println!("📋 阶段 3: 详细计划 (Planning)");
        println!("─────────────────────────────────────────");
        println!("状态: {:?}", planning.status);
        println!("置信度: {:.2}", planning.validation.confidence);
        println!("耗时: {} ms", planning.duration_ms);
        println!("重试次数: {}", planning.retry_count);
        
        if let Some(output) = &planning.output {
            println!();
            println!("📤 LLM 响应:");
            println!("─────────────────────────────────────────");
            println!("总步骤数: {}", output.steps.len());
            println!("预计耗时: {} 分钟", output.estimated_duration);
            
            println!();
            println!("步骤详情:");
            for (idx, step) in output.steps.iter().enumerate() {
                println!();
                println!("  步骤 {}: {}", idx + 1, step.name);
                println!("  ├─ 类型: {:?}", step.step_type);
                println!("  ├─ 描述: {}", step.description);
                println!("  ├─ 预计耗时: {} 分钟", step.estimated_duration);
                
                if !step.preconditions.is_empty() {
                    println!("  ├─ 前置条件:");
                    for pre in &step.preconditions {
                        println!("  │  • {}", pre);
                    }
                }
                
                if !step.expected_outputs.is_empty() {
                    println!("  ├─ 期望输出:");
                    for output in &step.expected_outputs {
                        println!("  │  • {}", output);
                    }
                }
                
                if !step.validation_criteria.is_empty() {
                    println!("  └─ 验证标准:");
                    for criteria in &step.validation_criteria {
                        println!("     • {}", criteria);
                    }
                } else {
                    println!("  └─ (无验证标准)");
                }
            }
            
            if !output.dependencies.is_empty() {
                println!();
                println!("步骤依赖 ({} 项):", output.dependencies.len());
                for dep in &output.dependencies {
                    println!("  {} -> {} ({:?})", dep.step_id, dep.depends_on, dep.dependency_type);
                }
            }
        }
    }
    
    println!();
}

/// 打印执行详情
fn print_execution_details(plan: &agent_runner::execution::SequentialExecutionPlan) {
    if plan.execution_history.is_empty() {
        return;
    }
    
    println!();
    println!("⚙️  阶段 4: 执行详情 (Execution)");
    println!("═══════════════════════════════════════════════════════════════════");
    
    for (idx, step_result) in plan.execution_history.iter().enumerate() {
        println!();
        println!("步骤 {} - {:?}", idx + 1, step_result.status);
        println!("─────────────────────────────────────────");
        println!("耗时: {} ms", step_result.duration_ms);
        println!("置信度: {:.2}", step_result.validation.confidence);
        
        if let Some(output) = &step_result.output {
            if !output.generated_files.is_empty() {
                println!();
                println!("生成的文件 ({} 个):", output.generated_files.len());
                for file in &output.generated_files {
                    println!("  📄 {}", file);
                }
            }
            
            if !output.modified_files.is_empty() {
                println!();
                println!("修改的文件 ({} 个):", output.modified_files.len());
                for file in &output.modified_files {
                    println!("  ✏️  {}", file);
                }
            }
            
            if !output.executed_commands.is_empty() {
                println!();
                println!("执行的命令 ({} 条):", output.executed_commands.len());
                for cmd in &output.executed_commands {
                    println!("  ⚙️  {}", cmd);
                }
            }
            
            if !output.logs.is_empty() {
                println!();
                println!("执行日志:");
                for log in &output.logs {
                    println!("  {}", log);
                }
            }
        }
        
        if !step_result.validation.messages.is_empty() {
            println!();
            println!("✅ 验证结果:");
            for msg in &step_result.validation.messages {
                println!("  • {}", msg);
            }
        }
        
        if !step_result.validation.warnings.is_empty() {
            println!();
            println!("⚠️  警告:");
            for warn in &step_result.validation.warnings {
                println!("  • {}", warn);
            }
        }
        
        if !step_result.validation.suggestions.is_empty() {
            println!();
            println!("💡 改进建议:");
            for suggestion in &step_result.validation.suggestions {
                println!("  • {}", suggestion);
            }
        }
    }
    
    println!();
}
