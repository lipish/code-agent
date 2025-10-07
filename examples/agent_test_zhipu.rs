//! Agent Testing with Zhipu AI
//!
//! This example tests the agent with different scenarios using Zhipu AI API.
//! It records all chat information and agent details for analysis.

use task_runner::agent::TaskAgent;
use task_runner::config::{AgentConfig, ModelConfig, ModelProvider};
use task_runner::models::LlmModel;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
    name: String,
    agent_type: String,
    task: String,
    expected_behavior: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestResult {
    test_case: String,
    agent_type: String,
    task: String,
    success: bool,
    duration_ms: u64,
    response_summary: String,
    response_details: String,
    model_calls: u32,
    tokens_used: Option<u64>,
    error: Option<String>,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestReport {
    provider: String,
    model: String,
    total_tests: usize,
    passed: usize,
    failed: usize,
    total_duration_ms: u64,
    results: Vec<TestResult>,
    analysis: TestAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestAnalysis {
    strengths: Vec<String>,
    weaknesses: Vec<String>,
    optimization_suggestions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🧪 Agent Testing with Zhipu AI\n");
    println!("{}", "=".repeat(80));

    // Load API key from keys.yaml
    let keys_content = fs::read_to_string("keys.yaml")?;
    let keys: serde_yaml::Value = serde_yaml::from_str(&keys_content)?;

    let zhipu_config = &keys["providers"]["zhipu"];
    let api_key = zhipu_config["api_key"].as_str().unwrap();
    let model = zhipu_config["models"][0].as_str().unwrap(); // glm-4.6

    println!("📡 Provider: Zhipu AI");
    println!("🤖 Model: {}", model);
    println!("{}", "=".repeat(80));
    println!();

    // Create model config
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: model.to_string(),
        api_key: Some(api_key.to_string()),
        endpoint: None,
        max_tokens: 2000,
        temperature: 0.7,
    };

    // Create LLM model
    let llm_model = LlmModel::from_config(model_config)?;

    // Create agent config
    let agent_config = AgentConfig::default();

    // Define test cases
    let test_cases = vec![
        TestCase {
            name: "Simple Code Task".to_string(),
            agent_type: "code".to_string(),
            task: "Write a Rust function to calculate fibonacci numbers".to_string(),
            expected_behavior: "Should provide a working Rust function".to_string(),
        },
        TestCase {
            name: "Data Analysis Task".to_string(),
            agent_type: "data".to_string(),
            task: "Analyze the performance metrics: CPU 80%, Memory 60%, Disk 40%".to_string(),
            expected_behavior: "Should provide analysis and recommendations".to_string(),
        },
        TestCase {
            name: "Simple Question".to_string(),
            agent_type: "generic".to_string(),
            task: "What is Rust programming language?".to_string(),
            expected_behavior: "Should provide a clear explanation".to_string(),
        },
    ];

    let mut results = Vec::new();
    let total_start = Instant::now();

    // Run test cases
    for (i, test_case) in test_cases.iter().enumerate() {
        println!("\n📋 Test Case {}/{}: {}", i + 1, test_cases.len(), test_case.name);
        println!("   Agent Type: {}", test_case.agent_type);
        println!("   Task: {}", test_case.task);
        println!("   Expected: {}", test_case.expected_behavior);
        println!("   {}", "-".repeat(76));

        let start = Instant::now();

        // Create agent (need to clone the model for each test)
        let model_config = ModelConfig {
            provider: ModelProvider::Zhipu,
            model_name: model.to_string(),
            api_key: Some(api_key.to_string()),
            endpoint: None,
            max_tokens: 2000,
            temperature: 0.7,
        };
        let llm_model = LlmModel::from_config(model_config)?;

        let mut agent = TaskAgent::new(
            Box::new(llm_model),
            agent_config.clone(),
        );

        // Execute task
        let result = agent.process_task(&test_case.task).await;
        let duration = start.elapsed();

        // Record result
        let test_result = match result {
            Ok(response) => {
                println!("   ✅ Success!");
                println!("   Duration: {:?}", duration);
                println!("   Summary: {}", response.summary);
                if let Some(ref details) = response.details {
                    println!("   Details: {}", details);
                }

                TestResult {
                    test_case: test_case.name.clone(),
                    agent_type: test_case.agent_type.clone(),
                    task: test_case.task.clone(),
                    success: response.success,
                    duration_ms: duration.as_millis() as u64,
                    response_summary: response.summary.clone(),
                    response_details: response.details.unwrap_or_default(),
                    model_calls: 1,
                    tokens_used: response.execution_time,
                    error: None,
                    timestamp: Utc::now().to_rfc3339(),
                }
            }
            Err(e) => {
                println!("   ❌ Failed: {}", e);
                println!("   Duration: {:?}", duration);

                TestResult {
                    test_case: test_case.name.clone(),
                    agent_type: test_case.agent_type.clone(),
                    task: test_case.task.clone(),
                    success: false,
                    duration_ms: duration.as_millis() as u64,
                    response_summary: String::new(),
                    response_details: String::new(),
                    model_calls: 0,
                    tokens_used: None,
                    error: Some(e.to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                }
            }
        };

        results.push(test_result);
    }

    let total_duration = total_start.elapsed();

    // Generate analysis
    let passed = results.iter().filter(|r| r.success).count();
    let failed = results.len() - passed;

    let analysis = analyze_results(&results);

    let report = TestReport {
        provider: "Zhipu AI".to_string(),
        model: model.to_string(),
        total_tests: results.len(),
        passed,
        failed,
        total_duration_ms: total_duration.as_millis() as u64,
        results,
        analysis,
    };

    // Save report
    let report_json = serde_json::to_string_pretty(&report)?;
    fs::write("agent_test_report.json", &report_json)?;

    // Print summary
    println!("\n\n");
    println!("=" .repeat(80));
    println!("📊 Test Summary");
    println!("=" .repeat(80));
    println!("Total Tests: {}", report.total_tests);
    println!("Passed: {} ✅", report.passed);
    println!("Failed: {} ❌", report.failed);
    println!("Success Rate: {:.1}%", (report.passed as f64 / report.total_tests as f64) * 100.0);
    println!("Total Duration: {:?}", total_duration);
    println!("Average Duration: {:?}", total_duration / report.total_tests as u32);
    println!();
    println!("📈 Analysis:");
    println!();
    println!("Strengths:");
    for strength in &report.analysis.strengths {
        println!("  ✅ {}", strength);
    }
    println!();
    println!("Weaknesses:");
    for weakness in &report.analysis.weaknesses {
        println!("  ⚠️  {}", weakness);
    }
    println!();
    println!("💡 Optimization Suggestions:");
    for suggestion in &report.analysis.optimization_suggestions {
        println!("  🔧 {}", suggestion);
    }
    println!();
    println!("📄 Full report saved to: agent_test_report.json");
    println!("=" .repeat(80));

    Ok(())
}

fn analyze_results(results: &[TestResult]) -> TestAnalysis {
    let mut strengths = Vec::new();
    let mut weaknesses = Vec::new();
    let mut suggestions = Vec::new();

    let success_rate = results.iter().filter(|r| r.success).count() as f64 / results.len() as f64;
    let avg_duration = results.iter().map(|r| r.duration_ms).sum::<u64>() / results.len() as u64;

    // Analyze success rate
    if success_rate >= 0.8 {
        strengths.push(format!("High success rate: {:.1}%", success_rate * 100.0));
    } else if success_rate < 0.5 {
        weaknesses.push(format!("Low success rate: {:.1}%", success_rate * 100.0));
        suggestions.push("Review error handling and retry logic".to_string());
    }

    // Analyze performance
    if avg_duration < 5000 {
        strengths.push(format!("Fast response time: {}ms average", avg_duration));
    } else if avg_duration > 10000 {
        weaknesses.push(format!("Slow response time: {}ms average", avg_duration));
        suggestions.push("Optimize prompt length and model parameters".to_string());
    }

    // Analyze by agent type
    let mut type_performance: HashMap<String, (usize, usize)> = HashMap::new();
    for result in results {
        let entry = type_performance.entry(result.agent_type.clone()).or_insert((0, 0));
        entry.0 += 1;
        if result.success {
            entry.1 += 1;
        }
    }

    for (agent_type, (total, success)) in type_performance {
        let rate = success as f64 / total as f64;
        if rate == 1.0 {
            strengths.push(format!("{} agent: 100% success", agent_type));
        } else if rate < 0.5 {
            weaknesses.push(format!("{} agent: low success rate ({:.1}%)", agent_type, rate * 100.0));
            suggestions.push(format!("Review {} agent prompt template", agent_type));
        }
    }

    // General suggestions
    if suggestions.is_empty() {
        suggestions.push("Consider adding more test cases".to_string());
        suggestions.push("Monitor token usage for cost optimization".to_string());
    }

    TestAnalysis {
        strengths,
        weaknesses,
        optimization_suggestions: suggestions,
    }
}

