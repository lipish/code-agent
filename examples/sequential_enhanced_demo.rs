//! Sequential Execution - Enhanced Features Demo
//!
//! This demo showcases the enhanced implementation with:
//! - Detailed step parsing from LLM
//! - Real file operations
//! - Real command execution
//! - Code generation with LLM
//! - Testing integration
//! - Guardrail integration

use agent_runner::execution::{
    SequentialExecutor,
    ExecutionConfig,
    ExecutionPhase,
};
use agent_runner::models::{MockModel, LanguageModel};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 Sequential Execution - Enhanced Features Demo");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    // Demo 1: File Operations
    demo_file_operations().await?;
    
    println!();
    
    // Demo 2: Code Generation
    demo_code_generation().await?;
    
    println!();
    
    // Demo 3: Complex Multi-Step Task
    demo_complex_task().await?;

    Ok(())
}

/// Demo 1: File Operations
async fn demo_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("📁 Demo 1: File Operations");
    println!("────────────────────────────────────────");
    
    let model = create_test_model()?;
    let config = ExecutionConfig {
        max_retries_per_phase: 1,
        require_confirmation: false,
        min_confidence_threshold: 0.6,
        enable_auto_rollback: true,
        verbose_logging: true,
    };

    let executor = SequentialExecutor::new(model, config);
    
    let task = "Create three configuration files: app.toml, database.yaml, and logging.json";
    
    println!("📝 Task: {}", task);
    println!();
    
    let plan = executor.execute_task(task).await?;
    
    print_execution_summary(&plan);
    
    Ok(())
}

/// Demo 2: Code Generation
async fn demo_code_generation() -> Result<(), Box<dyn std::error::Error>> {
    println!("💻 Demo 2: Code Generation");
    println!("────────────────────────────────────────");
    
    let model = create_test_model()?;
    let config = ExecutionConfig {
        max_retries_per_phase: 1,
        require_confirmation: false,
        min_confidence_threshold: 0.6,
        enable_auto_rollback: true,
        verbose_logging: true,
    };

    let executor = SequentialExecutor::new(model, config);
    
    let task = "Generate a Rust function that calculates the factorial of a number";
    
    println!("📝 Task: {}", task);
    println!();
    
    let plan = executor.execute_task(task).await?;
    
    print_execution_summary(&plan);
    
    Ok(())
}

/// Demo 3: Complex Multi-Step Task
async fn demo_complex_task() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Demo 3: Complex Multi-Step Task");
    println!("────────────────────────────────────────");
    
    let model = create_test_model()?;
    let config = ExecutionConfig {
        max_retries_per_phase: 2,
        require_confirmation: false,
        min_confidence_threshold: 0.7,
        enable_auto_rollback: true,
        verbose_logging: true,
    };

    let executor = SequentialExecutor::new(model, config);
    
    let task = r#"Create a simple HTTP server in Rust:
1. Create project structure
2. Generate server code
3. Create configuration
4. Run tests"#;
    
    println!("📝 Task:\n{}", task);
    println!();
    
    let plan = executor.execute_task(task).await?;
    
    print_execution_summary(&plan);
    print_detailed_execution(&plan);
    
    Ok(())
}

/// Create test model (MockModel only for this demo)
fn create_test_model() -> Result<Arc<dyn LanguageModel>, Box<dyn std::error::Error>> {
    // Use mock model for demo
    println!("⚠️  Using MockModel for enhanced features demo");
    println!("   (Real LLM integration available via sequential_llm_demo)");
    Ok(Arc::new(MockModel::new("enhanced-demo".to_string())))
}

/// Print execution summary
fn print_execution_summary(plan: &agent_runner::execution::SequentialExecutionPlan) {
    println!("📊 Execution Result");
    println!("═══════════════════════════════════════════════════════════════════");
    
    // Status
    let status = match plan.current_phase {
        ExecutionPhase::Completed => "✅ Completed",
        ExecutionPhase::Failed { .. } => "❌ Failed",
        _ => "⏸️  In Progress",
    };
    println!("{}", status);
    
    // Duration
    let total_duration = plan.updated_at.signed_duration_since(plan.started_at);
    println!("⏱️  Total Duration: {:.2} minutes", total_duration.num_milliseconds() as f64 / 60000.0);
    println!();
    
    // Phase summaries
    if let Some(understanding) = &plan.understanding {
        println!("🧠 Phase 1: Understanding");
        println!("  Status: {:?}", understanding.status);
        println!("  Confidence: {:.2}", understanding.validation.confidence);
        println!("  Duration: {} ms", understanding.duration_ms);
        if let Some(output) = &understanding.output {
            println!("  Task Type: {}", output.task_type);
            println!("  Complexity: {:?}", output.complexity);
        }
        println!();
    }
    
    if let Some(approach) = &plan.approach {
        println!("🎯 Phase 2: Approach");
        println!("  Status: {:?}", approach.status);
        println!("  Confidence: {:.2}", approach.validation.confidence);
        println!("  Duration: {} ms", approach.duration_ms);
        if let Some(output) = &approach.output {
            println!("  Architecture: {}", output.architecture_pattern);
            if !output.tech_stack.is_empty() {
                println!("  Tech Stack: {}", output.tech_stack.join(", "));
            }
        }
        println!();
    }
    
    if let Some(planning) = &plan.plan {
        println!("📋 Phase 3: Planning");
        println!("  Status: {:?}", planning.status);
        println!("  Confidence: {:.2}", planning.validation.confidence);
        println!("  Duration: {} ms", planning.duration_ms);
        if let Some(output) = &planning.output {
            println!("  Steps: {}", output.steps.len());
            println!("  Estimated Duration: {} minutes", output.estimated_duration);
        }
        println!();
    }
    
    // Execution history
    if !plan.execution_history.is_empty() {
        println!("⚙️  Phase 4: Execution");
        println!("  Total Steps: {}", plan.execution_history.len());
        let successful = plan.execution_history.iter()
            .filter(|r| r.status == agent_runner::execution::PhaseStatus::Success)
            .count();
        println!("  Successful: {}", successful);
        println!("  Failed: {}", plan.execution_history.len() - successful);
        println!();
    }
}

/// Print detailed execution information
fn print_detailed_execution(plan: &agent_runner::execution::SequentialExecutionPlan) {
    if plan.execution_history.is_empty() {
        return;
    }
    
    println!("📜 Execution Details");
    println!("═══════════════════════════════════════════════════════════════════");
    
    for (idx, step_result) in plan.execution_history.iter().enumerate() {
        println!();
        println!("Step {} - {:?}", idx + 1, step_result.status);
        println!("────────────────────────────────────────");
        println!("Duration: {} ms", step_result.duration_ms);
        
        if let Some(output) = &step_result.output {
            if !output.generated_files.is_empty() {
                println!("Generated Files:");
                for file in &output.generated_files {
                    println!("  📄 {}", file);
                }
            }
            
            if !output.modified_files.is_empty() {
                println!("Modified Files:");
                for file in &output.modified_files {
                    println!("  ✏️  {}", file);
                }
            }
            
            if !output.executed_commands.is_empty() {
                println!("Executed Commands:");
                for cmd in &output.executed_commands {
                    println!("  ⚙️  {}", cmd);
                }
            }
            
            if !output.logs.is_empty() {
                println!("Logs:");
                for log in &output.logs {
                    println!("  {}", log);
                }
            }
        }
        
        if !step_result.validation.messages.is_empty() {
            println!("Validation:");
            for msg in &step_result.validation.messages {
                println!("  ✓ {}", msg);
            }
        }
        
        if !step_result.validation.warnings.is_empty() {
            println!("Warnings:");
            for warn in &step_result.validation.warnings {
                println!("  ⚠️  {}", warn);
            }
        }
    }
}
