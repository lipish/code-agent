//! Rust Project Creation Planning Test
//!
//! This test validates the agent's ability to decompose the task of creating
//! a Rust project into detailed, actionable step-by-step plans.
//!
//! Based on memory knowledge about agent workflow test patterns, this implements:
//! 1. Basic connectivity test to verify API access
//! 2. Comprehensive workflow test covering task understanding, planning, and execution
//!
//! Test Focus: Task decomposition and step-by-step planning capabilities
//! Example Task: "创建一个 Rust 工程" (Create a Rust project)

use task_runner::agent::TaskAgent;
use task_runner::config::{AgentConfig, ModelConfig, ModelProvider, LogFormat};
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::types::TaskComplexity;

/// Test agent's ability to create step-by-step plans for Rust project creation
#[tokio::test]
async fn test_rust_project_step_planning() {
    println!("🦀 Rust Project Planning Test - Step Decomposition");
    println!("==================================================");
    println!();

    // Setup test agent following memory patterns
    let agent = setup_planning_test_agent().await;
    if agent.is_none() {
        println!("⚠️  Skipping test - agent setup failed");
        return;
    }
    let mut agent = agent.unwrap();

    // Test Case 1: Basic Rust project creation
    test_basic_rust_project_planning(&mut agent).await;
    
    // Test Case 2: Rust project with specific features
    test_advanced_rust_project_planning(&mut agent).await;
    
    // Test Case 3: Rust library project
    test_rust_library_planning(&mut agent).await;

    println!("🎉 Rust Project Planning Test Completed!");
    println!("=========================================");
}

/// Setup test agent optimized for planning analysis
async fn setup_planning_test_agent() -> Option<TaskAgent> {
    println!("🔧 Setting up planning test agent...");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 2000, // Optimized for planning tasks
        temperature: 0.5, // Lower temperature for more structured planning
    };

    let agent_config = AgentConfig {
        model: model_config.clone(),
        execution: task_runner::config::ExecutionConfig {
            max_steps: 15, // Allow for detailed step planning
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

    match LlmModel::from_config(model_config) {
        Ok(model) => {
            let model_box = Box::new(model) as Box<dyn LanguageModel>;
            let agent = TaskAgent::new(model_box, agent_config);
            println!("✅ Planning test agent created successfully");
            Some(agent)
        }
        Err(e) => {
            println!("❌ Failed to create planning test agent: {}", e);
            None
        }
    }
}

/// Test Case 1: Basic Rust project creation planning
async fn test_basic_rust_project_planning(agent: &mut TaskAgent) {
    println!("📋 Test Case 1: Basic Rust Project Creation");
    println!("==========================================");
    
    let task = "创建一个 Rust 工程。请详细分析这个任务，并制定具体的执行步骤计划。";
    
    println!("📝 Task (Chinese): {}", task);
    println!("📝 Task (English): Create a Rust project. Please analyze this task in detail and create specific execution step plans.");
    println!();

    analyze_rust_project_planning(agent, "Basic Rust Project", task).await;
}

/// Test Case 2: Advanced Rust project with features
async fn test_advanced_rust_project_planning(agent: &mut TaskAgent) {
    println!("📋 Test Case 2: Advanced Rust Project with Features");
    println!("=================================================");
    
    let task = r#"
创建一个完整的 Rust 工程，包含以下要求：
1. 创建一个名为 "task-processor" 的二进制项目
2. 添加必要的依赖项：tokio、serde、clap
3. 设置项目结构，包含 src/main.rs、src/lib.rs、tests/ 目录
4. 创建基本的 CLI 接口
5. 添加单元测试
6. 配置 Cargo.toml 文件
7. 创建 README.md 文档

请详细分析每个步骤，并制定完整的执行计划。
"#;
    
    println!("📝 Advanced Task:");
    println!("{}", task.trim());
    println!();

    analyze_rust_project_planning(agent, "Advanced Rust Project", task).await;
}

/// Test Case 3: Rust library project planning
async fn test_rust_library_planning(agent: &mut TaskAgent) {
    println!("📋 Test Case 3: Rust Library Project");
    println!("===================================");
    
    let task = r#"
创建一个 Rust 库项目，用于处理配置文件解析。要求：
- 项目名称：config-parser
- 支持 JSON、YAML、TOML 格式
- 提供统一的 API 接口
- 包含完整的文档和示例
- 设置 CI/CD 配置
- 发布到 crates.io 的准备工作

请分析这个任务的复杂性，并制定详细的分步执行计划。每个步骤都要具体可执行。
"#;
    
    println!("📝 Library Project Task:");
    println!("{}", task.trim());
    println!();

    analyze_rust_project_planning(agent, "Rust Library", task).await;
}

/// Analyze and evaluate planning quality for Rust project tasks
async fn analyze_rust_project_planning(agent: &mut TaskAgent, project_type: &str, task_description: &str) {
    println!("🚀 Analyzing {} planning capabilities...", project_type);
    
    let start_time = std::time::Instant::now();
    
    match agent.process_task(task_description).await {
        Ok(result) => {
            let duration = start_time.elapsed();
            
            println!("✅ Planning analysis completed!");
            println!();
            println!("📊 PLANNING DECOMPOSITION ANALYSIS");
            println!("=================================");
            
            // Basic metrics
            println!("⏱️  Performance Metrics:");
            println!("   - Analysis time: {:?}", duration);
            println!("   - Task success: {}", result.success);
            println!("   - Summary length: {} chars", result.summary.len());
            
            if let Some(execution_time) = result.execution_time {
                println!("   - Internal processing: {}ms", execution_time);
            }
            println!();
            
            // Task planning analysis
            if let Some(plan) = &result.task_plan {
                println!("🧠 TASK DECOMPOSITION QUALITY");
                println!("=============================");
                
                // Understanding analysis
                println!("📋 Task Understanding:");
                let understanding = &plan.understanding;
                println!("   - Understanding depth: {} characters", understanding.len());
                
                if understanding.len() > 100 {
                    let preview = understanding.chars().take(150).collect::<String>();
                    println!("   - Understanding preview: \"{}...\"", preview);
                } else {
                    println!("   - Understanding content: \"{}\"", understanding);
                }
                
                // Check for Chinese task comprehension
                if understanding.contains("Rust") || understanding.contains("项目") || understanding.contains("工程") {
                    println!("   ✅ Shows understanding of Rust project requirements");
                } else {
                    println!("   ⚠️  May not fully understand Rust project context");
                }
                println!();
                
                // Approach analysis
                println!("🎯 Execution Approach:");
                let approach = &plan.approach;
                println!("   - Approach detail: {} characters", approach.len());
                
                if approach.len() > 100 {
                    let preview = approach.chars().take(150).collect::<String>();
                    println!("   - Approach preview: \"{}...\"", preview);
                } else {
                    println!("   - Approach content: \"{}\"", approach);
                }
                
                // Check for step-by-step planning indicators
                let has_steps = approach.contains("步骤") || approach.contains("step") || 
                               approach.contains("阶段") || approach.contains("phase") ||
                               approach.contains("首先") || approach.contains("然后") ||
                               approach.contains("最后") || approach.contains("接下来");
                
                if has_steps {
                    println!("   ✅ Shows step-by-step planning approach");
                } else {
                    println!("   ⚠️  Approach lacks clear step indicators");
                }
                println!();
                
                // Complexity assessment
                println!("⚖️  Complexity Assessment:");
                println!("   - Assessed complexity: {:?}", plan.complexity);
                
                let appropriate_complexity = match project_type {
                    "Basic Rust Project" => matches!(plan.complexity, TaskComplexity::Simple | TaskComplexity::Moderate),
                    "Advanced Rust Project" => matches!(plan.complexity, TaskComplexity::Moderate | TaskComplexity::Complex),
                    "Rust Library" => matches!(plan.complexity, TaskComplexity::Complex),
                    _ => true,
                };
                
                if appropriate_complexity {
                    println!("   ✅ Appropriate complexity assessment for {}", project_type);
                } else {
                    println!("   ⚠️  Complexity may be misassessed for {}", project_type);
                }
                
                // Step estimation analysis
                if let Some(estimated_steps) = plan.estimated_steps {
                    println!("   - Estimated steps: {}", estimated_steps);
                    
                    let reasonable_steps = match project_type {
                        "Basic Rust Project" => estimated_steps >= 3 && estimated_steps <= 8,
                        "Advanced Rust Project" => estimated_steps >= 7 && estimated_steps <= 15,
                        "Rust Library" => estimated_steps >= 10 && estimated_steps <= 20,
                        _ => true,
                    };
                    
                    if reasonable_steps {
                        println!("   ✅ Reasonable step estimation for project type");
                    } else {
                        println!("   ⚠️  Step estimation may be inappropriate");
                    }
                } else {
                    println!("   ❌ No step estimation provided");
                }
                println!();
                
                // Requirements analysis
                println!("📋 Requirements Identification:");
                println!("   - Identified requirements: {}", plan.requirements.len());
                
                if plan.requirements.is_empty() {
                    println!("   ❌ No requirements identified - major planning gap");
                } else {
                    println!("   📝 Requirements found:");
                    for (i, req) in plan.requirements.iter().enumerate().take(5) {
                        println!("      {}. {}", i + 1, req);
                    }
                    if plan.requirements.len() > 5 {
                        println!("      ... and {} more requirements", plan.requirements.len() - 5);
                    }
                    
                    // Check for Rust-specific requirements
                    let rust_specific = plan.requirements.iter().any(|req| 
                        req.contains("Cargo") || req.contains("cargo") ||
                        req.contains("crate") || req.contains("依赖") ||
                        req.contains("Rust") || req.contains("src/")
                    );
                    
                    if rust_specific {
                        println!("   ✅ Includes Rust-specific requirements");
                    } else {
                        println!("   ⚠️  Missing Rust-specific technical requirements");
                    }
                }
                println!();
                
                // Planning quality score
                println!("🏆 PLANNING QUALITY SCORE");
                println!("========================");
                
                let mut score = 0;
                let mut max_score = 0;
                
                // Understanding depth (0-25 points)
                max_score += 25;
                if understanding.len() > 200 {
                    score += 25;
                    println!("   ✅ Understanding depth: Excellent (25/25)");
                } else if understanding.len() > 100 {
                    score += 15;
                    println!("   ⚠️  Understanding depth: Good (15/25)");
                } else if understanding.len() > 50 {
                    score += 10;
                    println!("   ⚠️  Understanding depth: Fair (10/25)");
                } else {
                    score += 5;
                    println!("   ❌ Understanding depth: Poor (5/25)");
                }
                
                // Approach detail (0-25 points)
                max_score += 25;
                if approach.len() > 200 && has_steps {
                    score += 25;
                    println!("   ✅ Approach planning: Excellent (25/25)");
                } else if approach.len() > 100 {
                    score += 15;
                    println!("   ⚠️  Approach planning: Good (15/25)");
                } else if approach.len() > 50 {
                    score += 10;
                    println!("   ⚠️  Approach planning: Fair (10/25)");
                } else {
                    score += 5;
                    println!("   ❌ Approach planning: Poor (5/25)");
                }
                
                // Complexity assessment (0-20 points)
                max_score += 20;
                if appropriate_complexity {
                    score += 20;
                    println!("   ✅ Complexity assessment: Correct (20/20)");
                } else {
                    score += 10;
                    println!("   ⚠️  Complexity assessment: Questionable (10/20)");
                }
                
                // Requirements identification (0-20 points)
                max_score += 20;
                if plan.requirements.len() >= 5 {
                    score += 20;
                    println!("   ✅ Requirements identification: Excellent (20/20)");
                } else if plan.requirements.len() >= 3 {
                    score += 15;
                    println!("   ⚠️  Requirements identification: Good (15/20)");
                } else if plan.requirements.len() >= 1 {
                    score += 10;
                    println!("   ⚠️  Requirements identification: Fair (10/20)");
                } else {
                    score += 0;
                    println!("   ❌ Requirements identification: None (0/20)");
                }
                
                // Step estimation (0-10 points)
                max_score += 10;
                if let Some(_) = plan.estimated_steps {
                    score += 10;
                    println!("   ✅ Step estimation: Provided (10/10)");
                } else {
                    score += 0;
                    println!("   ❌ Step estimation: Missing (0/10)");
                }
                
                let quality_percentage = (score as f64 / max_score as f64) * 100.0;
                println!();
                println!("🎯 OVERALL PLANNING QUALITY: {:.1}% ({}/{})", 
                         quality_percentage, score, max_score);
                
                // Quality assessment
                if quality_percentage >= 85.0 {
                    println!("   🏆 EXCELLENT - Agent shows strong step-by-step planning capability");
                } else if quality_percentage >= 70.0 {
                    println!("   ✅ GOOD - Agent demonstrates solid planning skills");
                } else if quality_percentage >= 55.0 {
                    println!("   ⚠️  ADEQUATE - Agent shows basic planning ability");
                } else {
                    println!("   ❌ POOR - Agent lacks effective step decomposition skills");
                }
                
                // Specific feedback for Rust project planning
                println!();
                println!("🦀 RUST PROJECT PLANNING ASSESSMENT:");
                if understanding.contains("Rust") || approach.contains("cargo") {
                    println!("   ✅ Shows awareness of Rust ecosystem");
                } else {
                    println!("   ❌ Lacks Rust-specific technical understanding");
                }
                
                if has_steps {
                    println!("   ✅ Demonstrates step-by-step planning approach");
                } else {
                    println!("   ❌ Missing clear step decomposition");
                }
                
            } else {
                println!("❌ CRITICAL: No task plan generated!");
                println!("   This indicates a fundamental failure in planning capability.");
            }
            
            println!();
            println!("📄 EXECUTION SUMMARY:");
            println!("   {}", result.summary);
            
        }
        Err(e) => {
            println!("❌ Planning analysis failed: {}", e);
            println!("🔍 Error Analysis:");
            let error_str = e.to_string();
            if error_str.contains("timeout") {
                println!("   - Planning task exceeded timeout limit");
            } else if error_str.contains("API") {
                println!("   - API connectivity or authentication issue");
            } else {
                println!("   - Unexpected error during planning: {}", error_str);
            }
        }
    }
    
    println!();
    println!("{}", "=".repeat(60));
    println!();
}

/// Basic connectivity test following memory pattern
#[tokio::test]
async fn test_basic_connectivity_for_planning() {
    println!("🔧 Basic Connectivity Test for Planning");
    println!("=======================================");

    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 500,
        temperature: 0.3,
    };

    let model = match LlmModel::from_config(model_config) {
        Ok(model) => model,
        Err(e) => {
            println!("❌ Model creation failed: {}", e);
            return;
        }
    };

    let test_prompt = "请用中文回答：创建Rust项目需要哪些基本步骤？";
    println!("📤 Testing prompt: '{}'", test_prompt);

    match model.complete(test_prompt).await {
        Ok(response) => {
            println!("✅ Connectivity test passed!");
            println!("📥 Response preview: {}", 
                     response.content.chars().take(100).collect::<String>());
            if let Some(usage) = response.usage {
                println!("📊 Token usage: {} total", usage.total_tokens);
            }
        }
        Err(e) => {
            println!("❌ Connectivity test failed: {}", e);
        }
    }
}