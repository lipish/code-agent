//! 完整展示TaskPlan结构的所有字段

use tokio;
use task_runner::planning::PlanningEngine;
use task_runner::models::LlmModel;
use task_runner::config::{ModelConfig, ModelProvider};
use task_runner::types::{TaskPlan, TaskComplexity};
use std::sync::Arc;
use serde_json;

#[tokio::test]
async fn test_complete_taskplan_display() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 TaskPlan完整结构展示");
    println!("========================");
    
    // 创建模型配置
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 2000,
        temperature: 0.7,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let planning_engine = PlanningEngine::new(Arc::new(model));
    
    // 测试任务
    let task = "创建一个完整的Web应用程序，包含前端、后端和数据库";
    
    println!("🎯 测试任务: {}", task);
    println!();
    
    match planning_engine.analyze_task(task).await {
        Ok(plan) => {
            println!("✅ 任务分析完成");
            println!();
            
            // 完整打印TaskPlan结构
            print_complete_taskplan(&plan);
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 任务分析失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[tokio::test]
async fn test_multiple_taskplan_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 多个TaskPlan对比展示");
    println!("========================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1500,
        temperature: 0.6,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let planning_engine = PlanningEngine::new(Arc::new(model));
    
    let tasks = vec![
        ("简单任务", "读取一个文件的内容"),
        ("中等任务", "创建一个Rust库项目"),
        ("复杂任务", "设计分布式系统架构"),
    ];
    
    for (i, (category, task)) in tasks.iter().enumerate() {
        println!("📋 {} - {}: {}", i + 1, category, task);
        println!("{}", "=".repeat(60));
        
        match planning_engine.analyze_task(task).await {
            Ok(plan) => {
                print_complete_taskplan(&plan);
                println!();
            }
            Err(e) => {
                println!("❌ 分析失败: {}", e);
                println!();
            }
        }
    }
    
    Ok(())
}

/// 完整打印TaskPlan结构的所有字段
fn print_complete_taskplan(plan: &TaskPlan) {
    println!("📊 TaskPlan完整结构:");
    println!("{}", "─".repeat(80));
    
    // 1. 基本信息
    println!("🎯 UNDERSTANDING (任务理解):");
    println!("   类型: String");
    println!("   长度: {} 字符", plan.understanding.len());
    println!("   内容: \"{}\"", plan.understanding);
    println!();
    
    println!("🛠️ APPROACH (解决方法):");
    println!("   类型: String");
    println!("   长度: {} 字符", plan.approach.len());
    println!("   内容: \"{}\"", plan.approach);
    println!();
    
    println!("⚖️ COMPLEXITY (复杂度):");
    println!("   类型: TaskComplexity (枚举)");
    println!("   值: {:?}", plan.complexity);
    println!("   说明: {}", get_complexity_description(&plan.complexity));
    println!();
    
    println!("📊 ESTIMATED_STEPS (预估步骤):");
    println!("   类型: Option<u32>");
    match plan.estimated_steps {
        Some(steps) => {
            println!("   值: Some({})", steps);
            println!("   说明: 预估需要 {} 个执行步骤", steps);
        }
        None => {
            println!("   值: None");
            println!("   说明: 未估算步骤数");
        }
    }
    println!();
    
    println!("📝 REQUIREMENTS (需求列表):");
    println!("   类型: Vec<String>");
    println!("   长度: {} 项", plan.requirements.len());
    if plan.requirements.is_empty() {
        println!("   内容: [] (空列表)");
        println!("   说明: 未识别出具体需求");
    } else {
        println!("   内容: [");
        for (i, req) in plan.requirements.iter().enumerate() {
            println!("     [{}] \"{}\"", i, req);
        }
        println!("   ]");
        println!("   说明: 识别出 {} 个具体需求", plan.requirements.len());
    }
    println!();
    
    // 2. Rust Debug格式
    println!("🔍 Rust Debug 格式:");
    println!("{}", "─".repeat(50));
    println!("{:#?}", plan);
    println!();
    
    // 3. JSON格式 (如果实现了Serialize)
    println!("🌐 JSON 格式 (如果支持):");
    println!("{}", "─".repeat(50));
    match serde_json::to_string_pretty(plan) {
        Ok(json) => println!("{}", json),
        Err(_) => println!("TaskPlan未实现Serialize trait"),
    }
    println!();
    
    // 4. 字段统计
    println!("📈 字段统计:");
    println!("{}", "─".repeat(30));
    println!("• understanding 字符数: {}", plan.understanding.len());
    println!("• approach 字符数: {}", plan.approach.len());
    println!("• complexity 复杂度: {:?}", plan.complexity);
    println!("• estimated_steps: {}", 
             plan.estimated_steps.map_or("未设置".to_string(), |s| s.to_string()));
    println!("• requirements 数量: {}", plan.requirements.len());
    
    let total_chars = plan.understanding.len() + plan.approach.len() + 
                     plan.requirements.iter().map(|r| r.len()).sum::<usize>();
    println!("• 总文本字符数: {}", total_chars);
    println!();
    
    // 5. 内存占用估算
    println!("💾 内存占用估算:");
    println!("{}", "─".repeat(30));
    let memory_estimate = estimate_taskplan_memory_usage(plan);
    println!("• 估算内存占用: {} 字节", memory_estimate);
    println!("{}", "─".repeat(80));
}

/// 获取复杂度描述
fn get_complexity_description(complexity: &TaskComplexity) -> &'static str {
    match complexity {
        TaskComplexity::Simple => "简单任务，通常需要1-3个步骤",
        TaskComplexity::Moderate => "中等任务，通常需要3-7个步骤", 
        TaskComplexity::Complex => "复杂任务，通常需要7+个步骤",
    }
}

/// 估算TaskPlan的内存占用
fn estimate_taskplan_memory_usage(plan: &TaskPlan) -> usize {
    let base_size = std::mem::size_of::<TaskPlan>();
    let understanding_size = plan.understanding.len();
    let approach_size = plan.approach.len();
    let requirements_size = plan.requirements.iter().map(|r| r.len()).sum::<usize>();
    let requirements_overhead = plan.requirements.len() * std::mem::size_of::<String>();
    
    base_size + understanding_size + approach_size + requirements_size + requirements_overhead
}

#[tokio::test]
async fn test_taskplan_field_details() {
    println!("🔍 TaskPlan字段类型详解");
    println!("========================");
    
    // 创建一个示例TaskPlan来展示结构
    let example_plan = TaskPlan {
        understanding: "这是一个示例任务理解".to_string(),
        approach: "这是解决方法".to_string(),
        complexity: TaskComplexity::Moderate,
        estimated_steps: Some(5),
        requirements: vec![
            "需求1".to_string(),
            "需求2".to_string(),
            "需求3".to_string(),
        ],
    };
    
    println!("📋 TaskPlan结构定义:");
    println!("{}", "─".repeat(40));
    println!("```rust");
    println!("pub struct TaskPlan {{");
    println!("    pub understanding: String,      // 任务理解");
    println!("    pub approach: String,           // 解决方法");
    println!("    pub complexity: TaskComplexity, // 复杂度枚举");
    println!("    pub estimated_steps: Option<u32>, // 可选的步骤数");
    println!("    pub requirements: Vec<String>,  // 需求列表");
    println!("}}");
    println!("```");
    println!();
    
    println!("🎯 TaskComplexity枚举定义:");
    println!("{}", "─".repeat(40));
    println!("```rust");
    println!("pub enum TaskComplexity {{");
    println!("    Simple,   // 简单");
    println!("    Moderate, // 中等");
    println!("    Complex,  // 复杂");
    println!("}}");
    println!("```");
    println!();
    
    println!("📊 示例TaskPlan实例:");
    print_complete_taskplan(&example_plan);
}