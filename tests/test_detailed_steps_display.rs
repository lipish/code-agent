//! 展示TaskPlan中具体步骤内容的测试
//! 专门用来打印step的详细内容，而不仅仅是步骤数量

use tokio;
use task_runner::agent::TaskAgent;
use task_runner::config::{AgentConfig, ModelConfig, ModelProvider, LogFormat};
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::planning::PlanningEngine;
use std::sync::Arc;

#[tokio::test]
async fn test_display_detailed_task_steps() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 TaskPlan详细步骤内容展示");
    println!("=============================");
    
    // 创建Agent
    let agent = setup_test_agent().await?;
    let mut agent = agent;
    
    // 测试任务 - 使用复杂一点的任务来获得更详细的步骤
    let task = "创建一个完整的Rust Web服务项目，包含API端点、数据库连接和测试用例";
    
    println!("🎯 任务: {}", task);
    println!("🔄 开始执行任务...");
    println!();
    
    match agent.process_task(task).await {
        Ok(result) => {
            println!("✅ 任务执行完成！");
            
            // 打印TaskPlan中的具体步骤
            if let Some(plan) = &result.task_plan {
                display_task_plan_with_steps(plan);
            } else {
                println!("⚠️ 未生成任务计划");
            }
            
            // 显示执行结果摘要
            println!("📄 执行结果摘要:");
            println!("================");
            println!("• 状态: {}", if result.success { "✅ 成功" } else { "❌ 失败" });
            println!("• 摘要: {}", result.summary);
            
            if let Some(details) = &result.details {
                println!("• 详情: {}", details);
            }
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 任务执行失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[tokio::test]
async fn test_planning_engine_detailed_steps() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 Planning Engine详细步骤分析");
    println!("===============================");
    
    // 创建Planning Engine
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 3000, // 增加token限制以获得更详细的步骤
        temperature: 0.6,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let planning_engine = PlanningEngine::new(Arc::new(model));
    
    // 测试不同复杂度的任务
    let tasks = vec![
        ("简单任务", "创建一个Rust Hello World项目"),
        ("中等任务", "实现一个REST API服务器，支持用户注册和登录"),
        ("复杂任务", "设计并实现一个微服务架构，包含用户服务、订单服务和支付服务"),
    ];
    
    for (category, task) in tasks {
        println!("📋 {} - {}", category, task);
        println!("{}", "=".repeat(60));
        
        match planning_engine.analyze_task(task).await {
            Ok(plan) => {
                display_task_plan_with_steps(&plan);
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

#[tokio::test]
async fn test_step_by_step_breakdown() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 逐步骤分解演示");
    println!("==================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),  
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 2000,
        temperature: 0.5,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    // 直接调用LLM获取详细步骤
    let task = "创建一个包含前端和后端的完整Web应用";
    let detailed_prompt = format!(
        "请详细分析以下任务并提供具体的执行步骤：

任务：{}

请按以下格式返回：
UNDERSTANDING: [你对任务的详细理解]
APPROACH: [解决方案的具体方法]
COMPLEXITY: [SIMPLE, MODERATE, 或 COMPLEX]
STEPS: [具体的执行步骤，每行一个步骤，格式为：步骤X. 具体操作]
REQUIREMENTS: [技术需求和依赖]

要求：
1. 步骤要具体可执行，包含具体的命令
2. 每个步骤都要说明预期结果
3. 用中文回答，格式清晰
4. 至少包含5个详细步骤", task);
    
    println!("🎯 任务: {}", task);
    println!("📤 发送详细分析请求给LLM...");
    
    match model.complete(&detailed_prompt).await {
        Ok(response) => {
            println!("📥 LLM详细响应:");
            println!("{}", "─".repeat(80));
            println!("{}", response.content);
            println!("{}", "─".repeat(80));
            
            // 解析步骤
            parse_and_display_steps(&response.content);
            
            Ok(())
        }
        Err(e) => {
            println!("❌ LLM调用失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

/// 显示TaskPlan的详细步骤内容
fn display_task_plan_with_steps(plan: &task_runner::types::TaskPlan) {
    println!("📊 TaskPlan详细内容:");
    println!("{}", "─".repeat(60));
    
    println!("🎯 任务理解:");
    println!("  {}", plan.understanding);
    println!();
    
    println!("🛠️ 解决方法:");
    println!("  {}", plan.approach);
    println!();
    
    println!("⚖️ 复杂度: {:?}", plan.complexity);
    
    if let Some(steps) = plan.estimated_steps {
        println!("📊 预估步骤数: {}", steps);
    }
    println!();
    
    // 关键：检查是否有具体步骤内容
    println!("📝 具体执行步骤:");
    if plan.requirements.is_empty() {
        println!("  ⚠️ 当前TaskPlan结构中没有详细步骤列表");
        println!("  💡 这可能是因为使用的是基础的TaskPlan结构");
        println!("  💡 步骤信息可能包含在understanding或approach字段中");
        
        // 尝试从understanding和approach中提取步骤信息
        extract_steps_from_text(&plan.understanding, "任务理解");
        extract_steps_from_text(&plan.approach, "解决方法");
    } else {
        for (i, req) in plan.requirements.iter().enumerate() {
            println!("  {}. {}", i + 1, req);
        }
    }
    println!();
}

/// 从文本中提取步骤信息
fn extract_steps_from_text(text: &str, field_name: &str) {
    let lines: Vec<&str> = text.lines().collect();
    let mut found_steps = false;
    
    for line in &lines {
        let line = line.trim();
        // 查找包含步骤信息的行
        if line.contains("步骤") || line.contains("Step") || 
           line.starts_with("1.") || line.starts_with("2.") ||
           line.contains("first") || line.contains("然后") || line.contains("接下来") {
            if !found_steps {
                println!("  🔍 从{}中发现的步骤信息:", field_name);
                found_steps = true;
            }
            println!("    - {}", line);
        }
    }
    
    if found_steps {
        println!();
    }
}

/// 解析LLM响应中的步骤信息
fn parse_and_display_steps(response: &str) {
    println!();
    println!("🔍 解析出的具体步骤:");
    println!("{}", "─".repeat(40));
    
    let lines: Vec<&str> = response.lines().collect();
    let mut in_steps_section = false;
    let mut step_count = 0;
    
    for line in &lines {
        let line = line.trim();
        
        // 检测步骤部分开始
        if line.to_uppercase().contains("STEPS:") || line.contains("步骤:") {
            in_steps_section = true;
            continue;
        }
        
        // 检测下一个部分开始（停止解析步骤）
        if in_steps_section && (line.to_uppercase().starts_with("REQUIREMENTS:") || 
                               line.contains("需求:") || line.contains("要求:")) {
            in_steps_section = false;
        }
        
        // 解析步骤内容
        if in_steps_section && !line.is_empty() {
            // 查找步骤行
            if line.contains("步骤") && (line.contains(".") || line.contains("：") || line.contains(":")) {
                step_count += 1;
                println!("📌 {}", line);
            } else if line.starts_with(char::is_numeric) || 
                     (line.len() > 2 && line.chars().nth(1) == Some('.')) {
                step_count += 1;
                println!("📌 {}", line);
            } else if !line.starts_with("STEPS") && line.len() > 3 {
                // 步骤的详细描述
                println!("   {}", line);
            }
        }
    }
    
    if step_count == 0 {
        println!("⚠️ 未找到明确的步骤格式，显示完整响应以供分析");
    } else {
        println!();
        println!("✅ 总共解析出 {} 个步骤", step_count);
    }
}

/// 设置测试Agent
async fn setup_test_agent() -> Result<TaskAgent, Box<dyn std::error::Error>> {
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 3000,
        temperature: 0.7,
    };

    let agent_config = AgentConfig {
        model: model_config.clone(),
        execution: task_runner::config::ExecutionConfig {
            max_steps: 15,
            timeout_seconds: 120,
            max_retries: 3,
            retry_delay_seconds: 2,
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

    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let model_box = Box::new(model) as Box<dyn LanguageModel>;
    let agent = TaskAgent::new(model_box, agent_config);
    
    Ok(agent)
}