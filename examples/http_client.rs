//! HTTP client example for AI Agent Service

use ai_agent::service::{AiAgentClient, ApiClientBuilder, TaskRequest, BatchTaskRequest, BatchExecutionMode};
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🌐 AI Agent HTTP Client Example");
    println!("===============================");

    // Configuration
    let base_url = std::env::var("AI_AGENT_API_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key = std::env::var("AI_AGENT_API_KEY").ok();

    println!("🔗 Connecting to: {}", base_url);
    if api_key.is_some() {
        println!("🔑 Using API key authentication");
    }

    // Create HTTP client
    let client = if let Some(key) = api_key {
        AiAgentClient::new(ApiClientBuilder::http_with_auth(base_url, key))
    } else {
        AiAgentClient::new(ApiClientBuilder::http(base_url))
    };

    // Test service connection
    println!("\n🏥 Testing service connection...");
    match client.get_service_status().await {
        Ok(status) => {
            println!("✅ Service is healthy: {:?}", status.status);
            println!("📊 Version: {}", status.version);
            println!("⏱️  Uptime: {}s", status.uptime_seconds);
        }
        Err(e) => {
            println!("❌ Failed to connect to service: {}", e);
            println!("Make sure the service is running at {}", base_url);
            return Err(e);
        }
    }

    // Example 1: Simple file operation
    println!("\n📁 Example 1: File operations");
    println!("----------------------------");

    let file_tasks = vec![
        "Create a test file called 'example.txt' with some content",
        "Read the content of 'example.txt'",
        "List all files in the current directory",
    ];

    for task in file_tasks {
        println!("📝 Task: {}", task);
        match client.execute_simple_task(task).await {
            Ok(response) => {
                println!("✅ Success: {}", response.result.unwrap_or_default().summary);
                if let Some(details) = response.result.unwrap_or_default().details {
                    println!("📄 Details: {}", details);
                }
            }
            Err(e) => {
                println!("❌ Failed: {}", e);
            }
        println!();
        }
    }

    // Example 2: Code generation and analysis
    println!("💻 Example 2: Code generation and analysis");
    println!("---------------------------------------");

    let code_tasks = vec![
        "Write a Rust function that calculates factorial",
        "Create a Python script that processes CSV files",
        "Generate a simple HTML page with CSS styling",
    ];

    for task in code_tasks {
        println!("📝 Task: {}", task);
        match client.execute_simple_task(task).await {
            Ok(response) => {
                println!("✅ Success: {}", response.result.unwrap_or_default().summary);
            }
            Err(e) => {
                println!("❌ Failed: {}", e);
            }
        }
        println!();
    }

    // Example 3: System operations
    println!("🖥️  Example 3: System operations");
    println!("------------------------------");

    let system_tasks = vec![
        "Show current system information",
        "Check disk usage in current directory",
        "List running processes (if available)",
    ];

    for task in system_tasks {
        println!("📝 Task: {}", task);
        match client.execute_simple_task(task).await {
            Ok(response) => {
                println!("✅ Success: {}", response.result.unwrap_or_default().summary);
            }
            Err(e) => {
                println!("❌ Failed: {}", e);
            }
        }
        println!();
    }

    // Example 4: Batch processing
    println!("📦 Example 4: Batch processing");
    println!("---------------------------");

    let batch_request = BatchTaskRequest {
        tasks: vec![
            TaskRequest {
                task: "Get current date and time".to_string(),
                task_id: None,
                context: None,
                priority: None,
                metadata: None,
            },
            TaskRequest {
                task: "Show system environment variables".to_string(),
                task_id: None,
                context: None,
                priority: None,
                metadata: None,
            },
            TaskRequest {
                task: "Create a summary of this project".to_string(),
                task_id: None,
                context: None,
                priority: None,
                metadata: None,
            },
        ],
        mode: BatchExecutionMode::Parallel,
        continue_on_error: true,
    };

    println!("📦 Executing batch of 3 tasks in parallel...");
    match client.execute_batch(batch_request).await {
        Ok(batch_response) => {
            println!("✅ Batch completed successfully");
            println!("📊 Statistics:");
            println!("  • Total tasks: {}", batch_response.statistics.total_tasks);
            println!("  • Completed: {}", batch_response.statistics.completed_tasks);
            println!("  • Failed: {}", batch_response.statistics.failed_tasks);
            println!("  • Total time: {}s", batch_response.statistics.total_execution_time);
            println!("  • Average time: {:.2}s", batch_response.statistics.average_execution_time);

            println!("\n📋 Individual results:");
            for (i, response) in batch_response.responses.iter().enumerate() {
                println!("  {}. {:?} - {}", i + 1, response.status,
                    response.result.as_ref().map(|r| &r.summary).unwrap_or(&"No summary".to_string()));
            }
        }
        Err(e) => {
            println!("❌ Batch failed: {}", e);
        }
    }

    // Example 5: Get service metrics
    println!("\n📊 Example 5: Service metrics");
    println!("--------------------------");

    match client.get_metrics().await {
        Ok(metrics) => {
            println!("✅ Metrics retrieved successfully");
            println!("📈 Performance metrics:");
            println!("  • Uptime: {}s", metrics.uptime_seconds);
            println!("  • Total tasks: {}", metrics.total_tasks);
            println!("  • Completed: {}", metrics.completed_tasks);
            println!("  • Failed: {}", metrics.failed_tasks);
            println!("  • Average execution time: {:.2}s", metrics.average_execution_time_seconds);

            if !metrics.tool_usage.is_empty() {
                println!("🛠️  Tool usage statistics:");
                for (tool, count) in &metrics.tool_usage {
                    println!("  • {}: {} uses", tool, count);
                }
            }

            if !metrics.error_counts.is_empty() {
                println!("⚠️  Error statistics:");
                for (error_type, count) in &metrics.error_counts {
                    println!("  • {}: {} occurrences", error_type, count);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to get metrics: {}", e);
        }
    }

    // Example 6: Task monitoring (polling)
    println!("\n🔍 Example 6: Task monitoring");
    println!("----------------------------");

    let long_task = "Analyze all Rust source files in the project and create a comprehensive report";

    println!("📝 Starting long-running task: {}", long_task);
    let task_response = client.execute_simple_task(long_task).await?;

    let task_id = task_response.task_id;
    println!("🆔 Task ID: {}", task_id);

    // Poll for task status (simplified example)
    println!("⏳ Monitoring task progress...");
    for i in 1..=5 {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        match client.get_task_status(&task_id).await {
            Ok(status) => {
                println!("📊 Check {} - Status: {:?}", i, status.status);
                if matches!(status.status, ai_agent::service::TaskStatus::Completed | ai_agent::service::TaskStatus::Failed) {
                    println!("🏁 Task completed with status: {:?}", status.status);
                    break;
                }
            }
            Err(e) => {
                println!("❌ Failed to get task status: {}", e);
                break;
            }
        }
    }

    println!("\n🎉 HTTP client examples completed!");
    println!("==================================");

    Ok(())
}