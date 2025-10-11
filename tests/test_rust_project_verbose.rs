//! Rust项目创建测试 - 详细过程观察
//!
//! 这个测试专门用来观察agent处理"创建一个Rust工程"任务的完整过程
//! 包括：输入分析、模型交互、输出生成等各个环节

use task_runner::agent::TaskAgent;
use task_runner::config::{AgentConfig, ModelConfig, ModelProvider, LogFormat};
use task_runner::models::{LlmModel, LanguageModel};

#[tokio::test]
async fn test_rust_project_creation_verbose() {
    println!("🦀 Rust项目创建过程观察测试");
    println!("============================");
    println!();

    // 设置agent
    let agent = setup_verbose_agent().await;
    if agent.is_none() {
        println!("❌ Agent设置失败，跳过测试");
        return;
    }
    let mut agent = agent.unwrap();

    // 测试任务
    let task = "创建一个Rust工程，名字叫hello-world";
    
    println!("📝 测试任务输入:");
    println!("================");
    println!("任务描述: {}", task);
    println!("任务类型: 项目创建");
    println!("期望输出: 详细的步骤计划");
    println!();

    println!("🚀 开始执行任务...");
    println!("=================");
    
    let start_time = std::time::Instant::now();
    
    match agent.process_task(task).await {
        Ok(result) => {
            let duration = start_time.elapsed();
            
            println!("✅ 任务执行完成！耗时: {:?}", duration);
            println!();
            
            // 显示详细的执行结果
            display_detailed_results(&result);
            
        }
        Err(e) => {
            println!("❌ 任务执行失败: {}", e);
            println!("失败原因分析:");
            
            let error_str = e.to_string();
            if error_str.contains("timeout") {
                println!("  - 超时错误：任务处理时间过长");
            } else if error_str.contains("API") {
                println!("  - API错误：网络连接或认证问题");
            } else {
                println!("  - 其他错误: {}", error_str);
            }
        }
    }
}

async fn setup_verbose_agent() -> Option<TaskAgent> {
    println!("🔧 配置Agent...");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 2000,
        temperature: 0.7,
    };

    println!("  ✅ 模型配置:");
    println!("     - 提供商: {:?}", model_config.provider);
    println!("     - 模型: {}", model_config.model_name);
    println!("     - 最大令牌: {}", model_config.max_tokens);
    println!("     - 温度: {}", model_config.temperature);

    let agent_config = AgentConfig {
        model: model_config.clone(),
        execution: task_runner::config::ExecutionConfig {
            max_steps: 15,
            timeout_seconds: 60,
            max_retries: 2,
            retry_delay_seconds: 1,
        },
        safety: task_runner::config::SafetyConfig {
            enable_safety_checks: true,
            allowed_directories: vec![".".to_string(), "/tmp".to_string()],
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

    println!("  ✅ 执行配置:");
    println!("     - 最大步骤: {}", agent_config.execution.max_steps);
    println!("     - 超时时间: {}秒", agent_config.execution.timeout_seconds);
    println!("     - 启用工具: {:?}", agent_config.tools.enabled_tools);

    match LlmModel::from_config(model_config) {
        Ok(model) => {
            println!("  ✅ 语言模型创建成功");
            let model_box = Box::new(model) as Box<dyn LanguageModel>;
            let agent = TaskAgent::new(model_box, agent_config);
            println!("  ✅ TaskAgent创建成功");
            println!();
            Some(agent)
        }
        Err(e) => {
            println!("  ❌ 模型创建失败: {}", e);
            None
        }
    }
}

fn display_detailed_results(result: &task_runner::types::TaskResult) {
    println!("📊 详细执行结果分析");
    println!("===================");
    
    // 基本信息
    println!("🔍 基本信息:");
    println!("  - 执行状态: {}", if result.success { "✅ 成功" } else { "❌ 失败" });
    
    if let Some(exec_time) = result.execution_time {
        println!("  - 执行时间: {}毫秒", exec_time);
    }
    
    println!("  - 总结长度: {}个字符", result.summary.len());
    
    if let Some(details) = &result.details {
        println!("  - 详情长度: {}个字符", details.len());
    }
    println!();

    // 任务计划分析
    if let Some(plan) = &result.task_plan {
        println!("🧠 任务计划详细分析");
        println!("==================");
        
        println!("📋 任务理解:");
        println!("-------------");
        println!("{}", plan.understanding);
        println!();
        
        println!("🎯 执行方法:");
        println!("-------------");
        println!("{}", plan.approach);
        println!();
        
        println!("⚖️ 复杂度评估: {:?}", plan.complexity);
        
        if let Some(steps) = plan.estimated_steps {
            println!("📊 预估步骤数: {}", steps);
        } else {
            println!("📊 预估步骤数: 未提供");
        }
        
        println!();
        println!("📝 识别的需求清单:");
        println!("------------------");
        if plan.requirements.is_empty() {
            println!("  ❌ 没有识别出具体需求");
        } else {
            for (i, req) in plan.requirements.iter().enumerate() {
                println!("  {}. {}", i + 1, req);
            }
        }
        println!();
        
        // 步骤分解质量分析
        println!("🔍 步骤分解质量分析");
        println!("===================");
        
        // 检查是否包含步骤相关的关键词
        let understanding_has_steps = plan.understanding.contains("步骤") || 
                                    plan.understanding.contains("阶段") ||
                                    plan.understanding.contains("过程");
                                    
        let approach_has_steps = plan.approach.contains("步骤") || 
                               plan.approach.contains("首先") ||
                               plan.approach.contains("然后") ||
                               plan.approach.contains("接下来") ||
                               plan.approach.contains("最后");
        
        if understanding_has_steps {
            println!("  ✅ 任务理解中包含步骤分析");
        } else {
            println!("  ⚠️ 任务理解中缺少步骤分析");
        }
        
        if approach_has_steps {
            println!("  ✅ 执行方法中体现了步骤思维");
        } else {
            println!("  ⚠️ 执行方法缺少清晰的步骤规划");
        }
        
        // 检查Rust相关知识
        let has_rust_knowledge = plan.understanding.contains("Rust") ||
                                plan.understanding.contains("cargo") ||
                                plan.approach.contains("Rust") ||
                                plan.approach.contains("cargo") ||
                                plan.approach.contains("Cargo.toml");
        
        if has_rust_knowledge {
            println!("  ✅ 显示了Rust相关的专业知识");
        } else {
            println!("  ⚠️ 缺少Rust专业知识的体现");
        }
        
        // 检查是否提到了具体的命令或操作
        let has_concrete_actions = plan.approach.contains("cargo new") ||
                                 plan.approach.contains("cargo build") ||
                                 plan.approach.contains("cargo run") ||
                                 plan.approach.contains("创建") ||
                                 plan.approach.contains("编写");
        
        if has_concrete_actions {
            println!("  ✅ 包含了具体的操作指令");
        } else {
            println!("  ⚠️ 缺少具体可执行的操作");
        }
        println!();
        
    } else {
        println!("❌ 严重问题：没有生成任务计划！");
        println!("   这表明agent在任务理解或计划生成方面存在根本性问题");
        println!();
    }

    // 执行摘要
    println!("📄 执行摘要");
    println!("============");
    println!("{}", result.summary);
    
    if let Some(details) = &result.details {
        println!();
        println!("📋 详细信息");
        println!("============");
        println!("{}", details);
    }
    
    println!();
    println!("🎯 总体评价");
    println!("============");
    
    if result.success {
        if result.task_plan.is_some() {
            println!("✅ Agent成功完成了任务处理，生成了计划");
        } else {
            println!("⚠️ Agent完成了任务但没有生成详细计划");
        }
    } else {
        println!("❌ Agent未能成功完成任务");
    }
}

/// 简单的连接测试，观察基础的模型交互
#[tokio::test]
async fn test_basic_model_interaction() {
    println!("🔬 基础模型交互测试");
    println!("===================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 500,
        temperature: 0.5,
    };

    let model = match LlmModel::from_config(model_config) {
        Ok(model) => model,
        Err(e) => {
            println!("❌ 模型创建失败: {}", e);
            return;
        }
    };

    let test_prompt = "请详细说明创建一个Rust项目的步骤，要求分步骤列出。";
    
    println!("📤 发送给模型的提示:");
    println!("===================");
    println!("{}", test_prompt);
    println!();
    
    println!("🔄 正在调用模型API...");
    
    match model.complete(test_prompt).await {
        Ok(response) => {
            println!("✅ 模型响应成功！");
            println!();
            
            println!("📥 模型完整响应:");
            println!("================");
            println!("{}", response.content);
            println!();
            
            if let Some(usage) = response.usage {
                println!("📊 令牌使用统计:");
                println!("================");
                println!("  - 提示令牌: {}", usage.prompt_tokens);
                println!("  - 完成令牌: {}", usage.completion_tokens);
                println!("  - 总令牌: {}", usage.total_tokens);
            }
            
            // 分析响应质量
            println!();
            println!("🔍 响应质量分析:");
            println!("================");
            
            let content = &response.content;
            
            if content.contains("步骤") || content.contains("第一") || content.contains("首先") {
                println!("  ✅ 响应包含步骤化思维");
            } else {
                println!("  ⚠️ 响应缺少明确的步骤结构");
            }
            
            if content.contains("cargo") || content.contains("Rust") {
                println!("  ✅ 响应包含Rust专业知识");
            } else {
                println!("  ⚠️ 响应缺少Rust相关内容");
            }
            
            if content.len() > 200 {
                println!("  ✅ 响应内容充实 ({}字符)", content.len());
            } else {
                println!("  ⚠️ 响应内容较简单 ({}字符)", content.len());
            }
            
        }
        Err(e) => {
            println!("❌ 模型API调用失败: {}", e);
        }
    }
    
    println!();
    println!("🎯 基础交互测试完成");
}