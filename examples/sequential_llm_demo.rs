//! Sequential Execution with Real LLM Integration Demo
//!
//! This example demonstrates the sequential execution system with actual LLM calls.
//! It shows Understanding → Approach → Planning phases with real AI responses.

use agent_runner::config::{ModelConfig, ModelProvider};
use agent_runner::execution::{SequentialExecutor, ExecutionConfig, ExecutionPhase};
use agent_runner::models::LlmModel;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 Sequential Execution with Real LLM Integration");
    println!("================================================================================\n");

    // Demo 1: Simple task
    demo_simple_task().await?;

    // Demo 2: Complex task
    demo_complex_task().await?;

    Ok(())
}

async fn demo_simple_task() -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Demo 1: Simple Task - Read Configuration File");
    println!("────────────────────────────────────────\n");

    let model: Arc<dyn agent_runner::models::LanguageModel> = match create_model() {
        Ok(llm_model) => Arc::new(llm_model),
        Err(_) => {
            use agent_runner::models::MockModel;
            Arc::new(MockModel::new("sequential-demo".to_string()))
        }
    };
    
    let config = ExecutionConfig {
        max_retries_per_phase: 2,
        require_confirmation: false,
        min_confidence_threshold: 0.6,
        enable_auto_rollback: true,
        verbose_logging: true,
    };

    let executor = SequentialExecutor::new(model, config);

    let task = "Read the Cargo.toml file and extract the project name and version";

    let plan = executor.execute_task(task).await?;

    print_execution_result(&plan);

    println!();
    Ok(())
}

async fn demo_complex_task() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📖 Demo 2: Complex Task - Build Web Service");
    println!("────────────────────────────────────────\n");

    let model: Arc<dyn agent_runner::models::LanguageModel> = match create_model() {
        Ok(llm_model) => Arc::new(llm_model),
        Err(_) => {
            use agent_runner::models::MockModel;
            Arc::new(MockModel::new("sequential-demo".to_string()))
        }
    };
    
    let config = ExecutionConfig {
        max_retries_per_phase: 3,
        require_confirmation: false,
        min_confidence_threshold: 0.7,
        enable_auto_rollback: true,
        verbose_logging: true,
    };

    let executor = SequentialExecutor::new(model, config);

    let task = "Create a RESTful API service with user authentication, using Rust and Axum framework";

    let plan = executor.execute_task(task).await?;

    print_execution_result(&plan);

    println!();
    Ok(())
}

fn create_model() -> Result<LlmModel, Box<dyn std::error::Error>> {
    // Try to use real LLM if API key is available, otherwise use mock
    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        println!("✓ Using OpenAI GPT-4");
        let config = ModelConfig {
            provider: ModelProvider::OpenAI,
            model_name: "gpt-4".to_string(),
            api_key: Some(api_key),
            endpoint: None,
            max_tokens: 2000,
            temperature: 0.7,
        };
        Ok(LlmModel::from_config(config)?)
    } else if let Ok(api_key) = std::env::var("DEEPSEEK_API_KEY") {
        println!("✓ Using DeepSeek");
        let config = ModelConfig {
            provider: ModelProvider::DeepSeek,
            model_name: "deepseek-chat".to_string(),
            api_key: Some(api_key),
            endpoint: None,
            max_tokens: 2000,
            temperature: 0.7,
        };
        Ok(LlmModel::from_config(config)?)
    } else if let Ok(api_key) = std::env::var("LONGCAT_API_KEY") {
        println!("✓ Using LongCat");
        let config = ModelConfig {
            provider: ModelProvider::LongCat,
            model_name: "LongCat-Flash-Chat".to_string(),
            api_key: Some(api_key),
            endpoint: None,
            max_tokens: 2000,
            temperature: 0.7,
        };
        Ok(LlmModel::from_config(config)?)
    } else {
        println!("⚠️  No API key found, using mock model");
        println!("   Set OPENAI_API_KEY, DEEPSEEK_API_KEY, or LONGCAT_API_KEY to use real LLM\n");
        
        // Return error to use MockModel in the caller
        Err("No API key available".into())
    }
}

fn print_execution_result(plan: &agent_runner::execution::SequentialExecutionPlan) {
    println!("\n📊 Execution Result");
    println!("================================================================================");
    
    match &plan.current_phase {
        ExecutionPhase::Completed => println!("✅ Status: Completed"),
        ExecutionPhase::Failed { reason, .. } => println!("❌ Status: Failed - {}", reason),
        phase => println!("⏸️  Status: Paused at {:?}", phase),
    }
    
    println!("⏱️  Total Duration: {:.2} minutes", plan.total_duration_minutes());
    println!();

    // Phase 1: Understanding
    if let Some(understanding_result) = &plan.understanding {
        println!("🧠 Phase 1: Understanding");
        println!("  Status: {:?}", understanding_result.status);
        println!("  Confidence: {:.2}", understanding_result.validation.confidence);
        println!("  Duration: {} ms", understanding_result.duration_ms);
        println!("  Retries: {}", understanding_result.retry_count);
        
        if let Some(understanding) = &understanding_result.output {
            println!("  Understanding: {}", understanding.understanding);
            println!("  Task Type: {}", understanding.task_type);
            println!("  Complexity: {:?}", understanding.complexity);
            if !understanding.key_requirements.is_empty() {
                println!("  Key Requirements:");
                for (i, req) in understanding.key_requirements.iter().take(3).enumerate() {
                    println!("    {}. {}", i + 1, req);
                }
                if understanding.key_requirements.len() > 3 {
                    println!("    ... and {} more", understanding.key_requirements.len() - 3);
                }
            }
            if !understanding.potential_risks.is_empty() {
                println!("  Potential Risks:");
                for (i, risk) in understanding.potential_risks.iter().take(2).enumerate() {
                    println!("    {}. {}", i + 1, risk);
                }
            }
        }
        println!();
    }

    // Phase 2: Approach
    if let Some(approach_result) = &plan.approach {
        println!("🎯 Phase 2: Approach");
        println!("  Status: {:?}", approach_result.status);
        println!("  Confidence: {:.2}", approach_result.validation.confidence);
        println!("  Duration: {} ms", approach_result.duration_ms);
        println!("  Retries: {}", approach_result.retry_count);
        
        if let Some(approach) = &approach_result.output {
            println!("  Approach: {}", approach.approach);
            if !approach.tech_stack.is_empty() {
                println!("  Tech Stack: {}", approach.tech_stack.join(", "));
            }
            println!("  Architecture: {}", approach.architecture_pattern);
            if !approach.expected_outcomes.is_empty() {
                println!("  Expected Outcomes:");
                for (i, outcome) in approach.expected_outcomes.iter().take(2).enumerate() {
                    println!("    {}. {}", i + 1, outcome);
                }
            }
        }
        println!();
    }

    // Phase 3: Planning
    if let Some(plan_result) = &plan.plan {
        println!("📋 Phase 3: Planning");
        println!("  Status: {:?}", plan_result.status);
        println!("  Confidence: {:.2}", plan_result.validation.confidence);
        println!("  Duration: {} ms", plan_result.duration_ms);
        println!("  Retries: {}", plan_result.retry_count);
        
        if let Some(detailed_plan) = &plan_result.output {
            println!("  Steps: {}", detailed_plan.steps.len());
            println!("  Estimated Duration: {} minutes", detailed_plan.estimated_duration);
            if !detailed_plan.required_resources.is_empty() {
                println!("  Required Resources: {}", detailed_plan.required_resources.join(", "));
            }
            if !detailed_plan.success_criteria.is_empty() {
                println!("  Success Criteria:");
                for (i, criterion) in detailed_plan.success_criteria.iter().take(2).enumerate() {
                    println!("    {}. {}", i + 1, criterion);
                }
            }
        }
        println!();
    }

    // Validation messages
    println!("💬 Validation Messages:");
    if let Some(understanding) = &plan.understanding {
        for msg in &understanding.validation.messages {
            println!("  ✓ {}", msg);
        }
        for warn in &understanding.validation.warnings {
            println!("  ⚠️  {}", warn);
        }
    }
    if let Some(approach) = &plan.approach {
        for msg in &approach.validation.messages {
            println!("  ✓ {}", msg);
        }
    }
    if let Some(planning) = &plan.plan {
        for msg in &planning.validation.messages {
            println!("  ✓ {}", msg);
        }
    }
}
