//! Agent任务计划生成演示测试
//! 展示Agent如何将LLM返回的内容转换为结构化的任务计划

use tokio;
use task_runner::agent::TaskAgent;
use task_runner::config::{AgentConfig, ModelConfig, ModelProvider, LogFormat};
use task_runner::models::{LlmModel, LanguageModel};

#[tokio::test]
async fn test_agent_task_planning_process() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Agent任务计划生成演示");
    println!("========================");
    
    // 创建Agent
    let agent = setup_demo_agent().await?;
    let mut agent = agent;
    
    // 测试任务
    let task_request = "创建一个名为hello-rust的Rust项目，包含基本的Hello World程序";
    
    println!("📝 用户任务请求:");
    println!("  {}", task_request);
    println!();
    
    println!("🔄 Agent处理流程:");
    println!("=================");
    
    println!("步骤1: 📋 任务理解 - Agent调用Planning Engine分析任务");
    println!("步骤2: 🤖 LLM交互 - 发送结构化提示词给智谱GLM");
    println!("步骤3: 📊 解析响应 - 将LLM返回解析为TaskPlan结构");
    println!("步骤4: ⚙️ 执行任务 - 基于TaskPlan执行具体操作");
    println!("步骤5: 📋 生成结果 - 返回包含TaskPlan的完整结果");
    println!();
    
    println!("🚀 开始执行任务...");
    let start_time = std::time::Instant::now();
    
    // 执行任务（这里会完整展示整个流程）
    match agent.process_task(task_request).await {
        Ok(result) => {
            let duration = start_time.elapsed();
            
            println!("✅ 任务执行完成！耗时: {:?}", duration);
            println!();
            
            // 展示生成的任务计划
            println!("📋 生成的任务计划 (TaskPlan):");
            println!("==============================");
            
            if let Some(plan) = &result.task_plan {
                println!("🎯 任务理解:");
                println!("  {}", plan.understanding);
                println!();
                
                println!("🛠️ 解决方法:");
                println!("  {}", plan.approach);
                println!();
                
                println!("⚖️ 复杂度评估: {:?}", plan.complexity);
                
                if let Some(steps) = plan.estimated_steps {
                    println!("📊 预估步骤数: {}", steps);
                }
                
                if !plan.requirements.is_empty() {
                    println!("📝 识别的需求:");
                    for (i, req) in plan.requirements.iter().enumerate() {
                        println!("  {}. {}", i + 1, req);
                    }
                } else {
                    println!("📝 识别的需求: 无具体需求列表");
                }
            } else {
                println!("⚠️ 未生成任务计划");
            }
            
            println!();
            println!("🎯 执行结果:");
            println!("=============");
            println!("✅ 执行状态: {}", if result.success { "成功" } else { "失败" });
            println!("📄 执行摘要: {}", result.summary);
            
            if let Some(details) = &result.details {
                println!("📋 详细信息: {}", details);
            }
            
            if let Some(exec_time) = result.execution_time {
                println!("⏱️ 执行时间: {}秒", exec_time);
            }
            
            println!();
            println!("🔍 关键观察:");
            println!("============");
            println!("1. LLM原始输出 → 被PlanningEngine解析为结构化的TaskPlan");
            println!("2. TaskPlan包含: understanding, approach, complexity, requirements");
            println!("3. Agent基于TaskPlan决定具体的执行策略");
            println!("4. 最终结果包含完整的计划信息，便于追踪和调试");
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 任务执行失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[tokio::test]
async fn test_task_plan_structure_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 TaskPlan数据结构演示");
    println!("=======================");
    
    // 直接测试Planning Engine
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
    let planning_engine = task_runner::planning::PlanningEngine::new(
        std::sync::Arc::new(model)
    );
    
    let task = "编写一个Python脚本来分析CSV文件中的数据";
    
    println!("🎯 测试任务: {}", task);
    println!();
    
    println!("🤖 调用Planning Engine...");
    match planning_engine.analyze_task(task).await {
        Ok(plan) => {
            println!("✅ 任务分析完成");
            println!();
            
            println!("📋 TaskPlan结构详解:");
            println!("====================");
            
            println!("```rust");
            println!("TaskPlan {{");
            println!("    understanding: \"{}\",", plan.understanding);
            println!("    approach: \"{}\",", plan.approach);
            println!("    complexity: {:?},", plan.complexity);
            println!("    estimated_steps: {:?},", plan.estimated_steps);
            println!("    requirements: {:?},", plan.requirements);
            println!("}}");
            println!("```");
            
            println!();
            println!("🔍 字段说明:");
            println!("=============");
            println!("• understanding: Agent对任务的理解和分析");
            println!("• approach: 解决任务的方法和策略");
            println!("• complexity: 任务复杂度（Simple/Moderate/Complex）");
            println!("• estimated_steps: 预估需要的执行步骤数");
            println!("• requirements: 任务依赖和具体需求列表");
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 任务分析失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

/// 设置演示用的Agent
async fn setup_demo_agent() -> Result<TaskAgent, Box<dyn std::error::Error>> {
    // 模型配置
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 2000,
        temperature: 0.7,
    };

    // Agent配置
    let agent_config = AgentConfig {
        model: model_config.clone(),
        execution: task_runner::config::ExecutionConfig {
            max_steps: 10,
            timeout_seconds: 60,
            max_retries: 2,
            retry_delay_seconds: 1,
        },
        safety: task_runner::config::SafetyConfig {
            enable_safety_checks: true,
            allowed_directories: vec![".".to_string()],
            blocked_commands: vec!["rm -rf".to_string()],
        },
        tools: task_runner::config::ToolConfig {
            auto_discovery: true,
            custom_tools_path: None,
            enabled_tools: vec![
                "read_file".to_string(),
                "write_file".to_string(),
                "run_command".to_string(),
                "list_files".to_string(),
            ],
            disabled_tools: vec![],
        },
        logging: task_runner::config::LoggingConfig {
            level: "info".to_string(),
            file: None,
            console: true,
            format: LogFormat::Pretty,
        },
    };

    // 创建模型和Agent
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let model_box = Box::new(model) as Box<dyn LanguageModel>;
    let agent = TaskAgent::new(model_box, agent_config);
    
    Ok(agent)
}