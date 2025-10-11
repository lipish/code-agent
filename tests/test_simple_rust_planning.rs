//! Simple Rust Project Planning Test
//!
//! Tests agent's ability to break down "创建一个 Rust 工程" into step-by-step plans

use task_runner::agent::TaskAgent;
use task_runner::config::{AgentConfig, ModelConfig, ModelProvider, LogFormat};
use task_runner::models::{LlmModel, LanguageModel};

#[tokio::test]
async fn test_simple_rust_project_decomposition() {
    println!("🦀 Testing Rust Project Step Decomposition");
    println!("===========================================");

    // Quick connectivity test
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1500,
        temperature: 0.5,
    };

    let model = match LlmModel::from_config(model_config.clone()) {
        Ok(model) => Box::new(model) as Box<dyn LanguageModel>,
        Err(e) => {
            println!("❌ Model creation failed: {}", e);
            return;
        }
    };

    let agent_config = AgentConfig {
        model: model_config,
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
            enabled_tools: vec!["read_file".to_string(), "write_file".to_string()],
            disabled_tools: vec![],
        },
        logging: task_runner::config::LoggingConfig {
            level: "info".to_string(),
            file: None,
            console: true,
            format: LogFormat::Pretty,
        },
    };

    let mut agent = TaskAgent::new(model, agent_config);
    println!("✅ Agent created successfully");

    // Test task decomposition
    let task = "创建一个 Rust 工程。请分析这个任务并制定详细的步骤计划。";
    println!("📝 Task: {}", task);
    println!("🚀 Starting task planning analysis...");

    let start_time = std::time::Instant::now();
    
    match agent.process_task(task).await {
        Ok(result) => {
            let duration = start_time.elapsed();
            println!("✅ Task completed in {:?}", duration);
            println!();
            
            println!("📊 PLANNING RESULTS:");
            println!("===================");
            println!("Success: {}", result.success);
            println!("Summary: {}", result.summary);
            
            if let Some(plan) = result.task_plan {
                println!();
                println!("🧠 TASK PLAN ANALYSIS:");
                println!("======================");
                println!("Understanding: {}", plan.understanding);
                println!("Approach: {}", plan.approach);
                println!("Complexity: {:?}", plan.complexity);
                
                if let Some(steps) = plan.estimated_steps {
                    println!("Estimated steps: {}", steps);
                    
                    if steps >= 3 {
                        println!("✅ Agent provided reasonable step estimation");
                    } else {
                        println!("⚠️  Step estimation seems low for project creation");
                    }
                } else {
                    println!("❌ No step estimation provided");
                }
                
                println!("Requirements identified: {}", plan.requirements.len());
                for (i, req) in plan.requirements.iter().enumerate() {
                    println!("  {}. {}", i + 1, req);
                }
                
                // Analysis of planning quality
                println!();
                println!("🔍 STEP DECOMPOSITION ANALYSIS:");
                println!("===============================");
                
                let understanding_has_steps = plan.understanding.contains("步骤") || 
                                            plan.understanding.contains("step") ||
                                            plan.understanding.contains("阶段");
                                            
                let approach_has_steps = plan.approach.contains("步骤") || 
                                       plan.approach.contains("step") ||
                                       plan.approach.contains("首先") ||
                                       plan.approach.contains("然后") ||
                                       plan.approach.contains("接下来");
                
                if understanding_has_steps || approach_has_steps {
                    println!("✅ Agent shows step-by-step thinking");
                } else {
                    println!("❌ No clear step decomposition identified");
                }
                
                let has_rust_knowledge = plan.understanding.contains("Rust") ||
                                       plan.understanding.contains("cargo") ||
                                       plan.approach.contains("Rust") ||
                                       plan.approach.contains("cargo");
                
                if has_rust_knowledge {
                    println!("✅ Shows Rust-specific knowledge");
                } else {
                    println!("⚠️  Limited Rust-specific context");
                }
                
            } else {
                println!("❌ No task plan generated - this is a critical issue");
            }
            
        }
        Err(e) => {
            println!("❌ Task failed: {}", e);
        }
    }
    
    println!();
    println!("🎯 Test completed - Check results above for step decomposition quality");
}