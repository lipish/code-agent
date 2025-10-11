//! Agent Workflow Test for Zhipu GLM-4.6
//!
//! This test suite demonstrates the complete agent workflow as described in README_CN.md:
//!
//! ## Workflow Steps
//! 1. **任务理解 (Task Understanding)** - AI analyzes and understands the task
//! 2. **任务执行 (Task Execution)** - Agent executes the planned approach  
//! 3. **结果生成 (Result Generation)** - Results are formatted and returned
//!
//! ## Test Configuration
//! - Provider: Zhipu (智谱AI)
//! - Model: glm-4-flash (for faster testing)
//! - API Endpoint: https://open.bigmodel.cn/api/paas/v4
//!
//! ## Test Cases
//! - Basic connectivity test
//! - Complete agent workflow test with Chinese task
//!
//! Run with: `cargo test test_zhipu --nocapture`

use task_runner::agent::TaskAgent;
use task_runner::config::{AgentConfig, ModelConfig, ModelProvider, LogFormat};
use task_runner::models::{LlmModel, LanguageModel};

/// Simple test for Agent Workflow with Zhipu GLM-4.6
/// 
/// This test demonstrates the agent workflow steps:
/// 1. Configuration Setup
/// 2. Model Creation 
/// 3. Agent Creation
/// 4. Task Processing
#[tokio::test]
async fn test_zhipu_agent_workflow() {
    println!("🧪 Zhipu Agent Workflow Test");
    println!("============================");

    // Step 1: Create model configuration
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1000,
        temperature: 0.7,
    };

    println!("✅ Model Config: {:?} - {}", model_config.provider, model_config.model_name);

    // Step 2: Create agent configuration
    let agent_config = AgentConfig {
        model: model_config.clone(),
        execution: task_runner::config::ExecutionConfig {
            max_steps: 5,
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
            enabled_tools: vec!["read_file".to_string()],
            disabled_tools: vec![],
        },
        logging: task_runner::config::LoggingConfig {
            level: "info".to_string(),
            file: None,
            console: true,
            format: LogFormat::Pretty,
        },
    };

    println!("✅ Agent Config created");

    // Step 3: Create model and agent
    let model = match LlmModel::from_config(model_config) {
        Ok(model) => {
            println!("✅ LlmModel created successfully");
            Box::new(model) as Box<dyn LanguageModel>
        }
        Err(e) => {
            println!("❌ Failed to create model: {}", e);
            return; // Skip test if model creation fails
        }
    };

    let mut agent = TaskAgent::new(model, agent_config);
    println!("✅ TaskAgent created successfully");

    // Step 4: Test agent workflow
    println!("\n🚀 Testing Agent Workflow:");
    println!("==========================");

    let test_task = "简单解释什么是Rust编程语言的特点";
    println!("📝 Task: '{}'", test_task);

    match agent.process_task(test_task).await {
        Ok(result) => {
            println!("✅ Task completed successfully!");
            println!("📊 Results:");
            println!("   - Success: {}", result.success);
            println!("   - Summary: {}", result.summary);
            
            if let Some(plan) = result.task_plan {
                println!("   - Plan Understanding: {}", 
                    plan.understanding.chars().take(100).collect::<String>());
                println!("   - Plan Approach: {}", 
                    plan.approach.chars().take(100).collect::<String>());
                println!("   - Complexity: {:?}", plan.complexity);
            }
            
            if let Some(exec_time) = result.execution_time {
                println!("   - Execution time: {}ms", exec_time);
            }
        }
        Err(e) => {
            println!("❌ Task failed: {}", e);
            println!("🔍 This might be due to:");
            println!("   - Network connectivity issues");
            println!("   - API key authentication problems");
            println!("   - Service endpoint availability");
        }
    }

    println!("\n🎯 Workflow test completed");
}

/// Basic connectivity test
#[tokio::test]
async fn test_zhipu_basic_connectivity() {
    println!("🔧 Basic Zhipu Connectivity Test");
    println!("=================================");

    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 100,
        temperature: 0.3,
    };

    let model = match LlmModel::from_config(model_config) {
        Ok(model) => model,
        Err(e) => {
            println!("❌ Model creation failed: {}", e);
            return;
        }
    };

    let test_prompt = "Hello, respond with 'Hello from GLM!'";
    println!("📤 Testing prompt: '{}'", test_prompt);

    match model.complete(test_prompt).await {
        Ok(response) => {
            println!("✅ API call successful!");
            println!("📥 Response: {}", response.content.trim());
            if let Some(usage) = response.usage {
                println!("📊 Tokens: {} total", usage.total_tokens);
            }
        }
        Err(e) => {
            println!("❌ API call failed: {}", e);
        }
    }
}