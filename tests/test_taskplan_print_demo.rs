//! TaskPlan结构化计划内容打印演示
//! 专门用来展示LLM响应如何被解析为结构化的TaskPlan

use tokio;
use task_runner::planning::PlanningEngine;
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};
use std::sync::Arc;

#[tokio::test]
async fn test_print_taskplan_structure() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 TaskPlan结构化计划内容演示");
    println!("===============================");
    
    // 创建模型配置
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1500,
        temperature: 0.6,
    };
    
    // 创建LLM模型
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    // 创建Planning Engine
    let planning_engine = PlanningEngine::new(Arc::new(model));
    
    // 测试任务
    let task = "创建一个Rust工程，名字叫hello-world，包含基本的Hello World程序";
    
    println!("🎯 输入任务:");
    println!("  {}", task);
    println!();
    
    println!("🤖 正在调用Planning Engine分析任务...");
    let start_time = std::time::Instant::now();
    
    match planning_engine.analyze_task(task).await {
        Ok(plan) => {
            let duration = start_time.elapsed();
            println!("✅ 任务分析完成！耗时: {:?}", duration);
            println!();
            
            // 打印完整的TaskPlan结构
            println!("📊 解析后的TaskPlan结构化内容:");
            println!("=====================================");
            
            println!("🎯 任务理解 (understanding):");
            println!("─────────────────────────────");
            println!("{}", plan.understanding);
            println!();
            
            println!("🛠️ 解决方法 (approach):");
            println!("─────────────────────────");
            println!("{}", plan.approach);
            println!();
            
            println!("⚖️ 复杂度评估 (complexity):");
            println!("──────────────────────────");
            println!("{:?}", plan.complexity);
            println!();
            
            println!("📊 预估步骤数 (estimated_steps):");
            println!("────────────────────────────────");
            match plan.estimated_steps {
                Some(steps) => println!("{} 步", steps),
                None => println!("未估算"),
            }
            println!();
            
            println!("📝 识别的需求 (requirements):");
            println!("───────────────────────────────");
            if plan.requirements.is_empty() {
                println!("  (无具体需求列表)");
            } else {
                for (i, req) in plan.requirements.iter().enumerate() {
                    println!("  {}. {}", i + 1, req);
                }
            }
            println!();
            
            // 打印Rust Debug格式
            println!("🔍 完整TaskPlan结构 (Debug格式):");
            println!("=====================================");
            println!("{:#?}", plan);
            println!();
            
            // 统计信息
            println!("📈 统计信息:");
            println!("=============");
            println!("• 理解内容长度: {} 字符", plan.understanding.len());
            println!("• 方法描述长度: {} 字符", plan.approach.len());
            println!("• 需求数量: {}", plan.requirements.len());
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 任务分析失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[tokio::test]
async fn test_multiple_tasks_taskplan() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 多任务TaskPlan对比演示");
    println!("==========================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1000,
        temperature: 0.5,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let planning_engine = PlanningEngine::new(Arc::new(model));
    
    let tasks = vec![
        "读取一个文件",
        "创建一个Python脚本分析数据",
        "设计一个完整的Web应用程序架构",
    ];
    
    for (i, task) in tasks.iter().enumerate() {
        println!("📋 任务 {}: {}", i + 1, task);
        println!("{}", "─".repeat(50));
        
        match planning_engine.analyze_task(task).await {
            Ok(plan) => {
                println!("✅ 解析成功");
                println!("  理解: {}", truncate_string(&plan.understanding, 80));
                println!("  方法: {}", truncate_string(&plan.approach, 80));
                println!("  复杂度: {:?}", plan.complexity);
                println!("  步骤数: {:?}", plan.estimated_steps);
                println!("  需求数: {}", plan.requirements.len());
            }
            Err(e) => {
                println!("❌ 解析失败: {}", e);
            }
        }
        println!();
    }
    
    Ok(())
}

/// 截断字符串用于显示
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

#[tokio::test]
async fn test_show_llm_to_taskplan_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 LLM原始响应 → TaskPlan转换过程演示");
    println!("==========================================");
    
    // 我们先直接调用LLM获取原始响应
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1200,
        temperature: 0.4,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    // 构造Planning Engine使用的提示词
    let task = "编写一个简单的文件备份脚本";
    let prompt = format!(
        "分析以下任务并提供详细的执行计划：

任务：{}

请按以下格式回答：
UNDERSTANDING: [你对任务的理解]
APPROACH: [解决这个任务的方法]
COMPLEXITY: [SIMPLE, MODERATE, 或 COMPLEX]
REQUIREMENTS: [列出具体需求，每行一个]

请用中文回答，并提供技术细节。", task
    );
    
    println!("📤 发送给LLM的提示词:");
    println!("{}", "─".repeat(60));
    println!("{}", prompt);
    println!("{}", "─".repeat(60));
    println!();
    
    // 获取LLM原始响应
    match model.complete(&prompt).await {
        Ok(response) => {
            println!("📥 LLM原始响应:");
            println!("{}", "─".repeat(60));
            println!("{}", response.content);
            println!("{}", "─".repeat(60));
            println!();
            
            // 现在使用Planning Engine解析这个响应
            let planning_engine = PlanningEngine::new(Arc::new(
                LlmModel::from_config(ModelConfig {
                    provider: ModelProvider::Zhipu,
                    model_name: "glm-4-flash".to_string(),
                    api_key: Some("your-api-key-here".to_string()),
                    endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
                    max_tokens: 1200,
                    temperature: 0.4,
                }).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
            ));
            
            match planning_engine.analyze_task(task).await {
                Ok(plan) => {
                    println!("🔄 解析后的TaskPlan结构:");
                    println!("{}", "─".repeat(60));
                    println!("TaskPlan {{");
                    println!("    understanding: \"{}\",", plan.understanding);
                    println!("    approach: \"{}\",", plan.approach);
                    println!("    complexity: {:?},", plan.complexity);
                    println!("    estimated_steps: {:?},", plan.estimated_steps);
                    println!("    requirements: {:?},", plan.requirements);
                    println!("}}");
                    println!("{}", "─".repeat(60));
                    
                    println!();
                    println!("✅ 转换完成！原始文本已成功解析为结构化数据");
                }
                Err(e) => {
                    println!("❌ TaskPlan解析失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ LLM调用失败: {}", e);
        }
    }
    
    Ok(())
}