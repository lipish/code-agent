use agent_runner::execution::{SequentialExecutor, ExecutionConfig, ExecutionPhase};
use agent_runner::models::MockModel;
use std::sync::Arc;

/// 演示顺序执行系统的使用
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    
    println!("\n🚀 顺序执行系统演示");
    println!("{}", "=".repeat(80));
    
    // 创建模型
    let model = Arc::new(MockModel::new("sequential-demo".to_string()));
    
    // 配置执行器
    let config = ExecutionConfig {
        max_retries_per_phase: 3,
        require_confirmation: false,
        min_confidence_threshold: 0.7,
        enable_auto_rollback: true,
        verbose_logging: true,
    };
    
    println!("\n📋 执行配置:");
    println!("  • 最大重试次数: {}", config.max_retries_per_phase);
    println!("  • 最小置信度阈值: {}", config.min_confidence_threshold);
    println!("  • 自动回滚: {}", config.enable_auto_rollback);
    println!("  • 详细日志: {}", config.verbose_logging);
    
    // 创建执行器
    let executor = SequentialExecutor::new(model, config);
    
    // 测试任务
    let task_description = r#"
创建一个简单的用户认证系统，包括以下功能：
1. 用户注册
2. 用户登录
3. 密码重置
4. JWT token 生成和验证
    "#;
    
    println!("\n📝 任务描述:");
    println!("{}", task_description.trim());
    println!("\n{}", "=".repeat(80));
    
    // 执行任务
    println!("\n⏳ 开始执行任务...\n");
    
    let start_time = std::time::Instant::now();
    let plan = executor.execute_task(task_description).await?;
    let elapsed = start_time.elapsed();
    
    println!("\n{}", "=".repeat(80));
    println!("✅ 任务执行完成！");
    println!("{}", "=".repeat(80));
    
    // 打印执行结果摘要
    println!("\n📊 执行摘要:");
    println!("  • 任务 ID: {}", plan.task_id);
    println!("  • 最终状态: {:?}", plan.current_phase);
    println!("  • 总耗时: {:.2} 秒", elapsed.as_secs_f64());
    println!("  • 开始时间: {}", plan.started_at.format("%Y-%m-%d %H:%M:%S"));
    if let Some(completed) = plan.completed_at {
        println!("  • 完成时间: {}", completed.format("%Y-%m-%d %H:%M:%S"));
    }
    
    // Phase 1: Understanding
    if let Some(understanding) = &plan.understanding {
        println!("\n🧠 Phase 1: Understanding");
        println!("  • 状态: {:?}", understanding.status);
        println!("  • 耗时: {} ms", understanding.duration_ms);
        println!("  • 置信度: {:.2}", understanding.validation.confidence);
        println!("  • 重试次数: {}", understanding.retry_count);
        
        if let Some(output) = &understanding.output {
            println!("  • 任务理解: {}", output.understanding);
            println!("  • 任务类型: {}", output.task_type);
            println!("  • 复杂度: {:?}", output.complexity);
        }
    }
    
    // Phase 2: Approach
    if let Some(approach) = &plan.approach {
        println!("\n🎯 Phase 2: Approach");
        println!("  • 状态: {:?}", approach.status);
        println!("  • 耗时: {} ms", approach.duration_ms);
        println!("  • 置信度: {:.2}", approach.validation.confidence);
        println!("  • 重试次数: {}", approach.retry_count);
        
        if let Some(output) = &approach.output {
            println!("  • 方案描述: {}", output.approach);
            println!("  • 架构模式: {}", output.architecture_pattern);
        }
    }
    
    // Phase 3: Planning
    if let Some(planning) = &plan.plan {
        println!("\n📋 Phase 3: Planning");
        println!("  • 状态: {:?}", planning.status);
        println!("  • 耗时: {} ms", planning.duration_ms);
        println!("  • 置信度: {:.2}", planning.validation.confidence);
        println!("  • 重试次数: {}", planning.retry_count);
        
        if let Some(output) = &planning.output {
            println!("  • 执行步骤数: {}", output.steps.len());
            println!("  • 预估总时间: {} 分钟", output.estimated_duration);
            println!("  • 里程碑数: {}", output.milestones.len());
        }
    }
    
    // Phase 4: Execution
    if !plan.execution_history.is_empty() {
        println!("\n⚙️  Phase 4: Execution");
        println!("  • 已执行步骤数: {}", plan.execution_history.len());
        println!("  • 成功步骤数: {}", plan.completed_steps_count());
        
        if let Some(failed) = plan.find_failed_step() {
            println!("  • ❌ 发现失败步骤:");
            if let Some(output) = &failed.output {
                println!("      步骤 ID: {}", output.step_id);
                println!("      状态: {:?}", output.status);
            }
            if let Some(error) = &failed.error {
                println!("      错误: {}", error);
            }
        }
    }
    
    // Phase 5: Final Validation
    if let Some(validation) = &plan.final_validation {
        println!("\n✅ Phase 5: Final Validation");
        println!("  • 状态: {:?}", validation.status);
        println!("  • 耗时: {} ms", validation.duration_ms);
        
        if let Some(output) = &validation.output {
            println!("  • 验证通过: {}", output.passed);
            println!("  • 总体评分: {:.2}", output.overall_score);
            println!("  • 验证项数: {}", output.validation_details.len());
            
            if !output.recommendations.is_empty() {
                println!("  • 建议:");
                for rec in &output.recommendations {
                    println!("      - {}", rec);
                }
            }
        }
    }
    
    println!("\n{}", "=".repeat(80));
    
    // 检查最终状态
    match plan.current_phase {
        ExecutionPhase::Completed => {
            println!("🎉 任务成功完成！");
        }
        ExecutionPhase::Failed { failed_at, reason } => {
            println!("❌ 任务执行失败！");
            println!("  失败阶段: {:?}", failed_at);
            println!("  失败原因: {}", reason);
        }
        _ => {
            println!("⏸️  任务未完成，当前阶段: {:?}", plan.current_phase);
        }
    }
    
    println!("{}", "=".repeat(80));
    
    Ok(())
}
