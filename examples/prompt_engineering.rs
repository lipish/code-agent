//! Example: Using the Prompt Engineering System
//!
//! This example demonstrates how to use the flexible prompt system
//! with custom templates and scenarios.

use task_runner::models::MockModel;
use task_runner::planning::PlanningEngine;
use task_runner::prompts::{PromptBuilder, PromptTemplate, ProjectRules, ScenarioPrompt};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== Prompt Engineering System Examples ===\n");

    // Example 1: Using default template
    example_1_default_template().await?;

    // Example 2: Using custom template from file
    example_2_custom_template().await?;

    // Example 3: Building prompts with PromptBuilder
    example_3_prompt_builder()?;

    // Example 4: Task type inference
    example_4_task_type_inference().await?;

    // Example 5: Dynamic template modification
    example_5_dynamic_template()?;

    Ok(())
}

/// Example 1: Using default template
async fn example_1_default_template() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 1: Default Template ---");

    let model = Arc::new(MockModel::new(
        "UNDERSTANDING: Create a configuration loader\n\
         APPROACH: Use serde for deserialization\n\
         COMPLEXITY: Simple\n\
         REQUIREMENTS: serde, serde_yaml"
            .to_string(),
    ));

    let engine = PlanningEngine::new(model);
    let plan = engine
        .analyze_task("Create a function to load YAML config")
        .await?;

    println!("Understanding: {}", plan.understanding);
    println!("Approach: {}", plan.approach);
    println!("Complexity: {:?}", plan.complexity);
    println!("Requirements: {:?}\n", plan.requirements);

    Ok(())
}

/// Example 2: Using custom template from file
async fn example_2_custom_template() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 2: Custom Template from File ---");

    let model = Arc::new(MockModel::new(
        "UNDERSTANDING: Optimize string concatenation\n\
         APPROACH: Use String::with_capacity and reduce allocations\n\
         COMPLEXITY: Moderate\n\
         REQUIREMENTS: criterion for benchmarking"
            .to_string(),
    ));

    // Try to load Rust-specific template
    let template = match PromptTemplate::from_file("prompts/rust-project.yaml") {
        Ok(t) => t,
        Err(_) => {
            println!("Note: rust-project.yaml not found, using default template");
            PromptTemplate::default()
        }
    };

    let engine = PlanningEngine::with_template(model, template);
    let plan = engine
        .analyze_task("Optimize the prompt building performance")
        .await?;

    println!("Understanding: {}", plan.understanding);
    println!("Approach: {}", plan.approach);
    println!("Complexity: {:?}\n", plan.complexity);

    Ok(())
}

/// Example 3: Building prompts with PromptBuilder
fn example_3_prompt_builder() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 3: PromptBuilder ---");

    let template = PromptTemplate::default();
    let builder = PromptBuilder::new(template);

    let prompt = builder
        .task_type("code_generation")
        .context("language", "Rust")
        .context("framework", "Tokio")
        .context("current_module", "src/prompts.rs")
        .build("Create an async function to load templates from directory");

    println!("Generated Prompt (first 500 chars):");
    println!("{}\n", &prompt[..prompt.len().min(500)]);

    Ok(())
}

/// Example 4: Task type inference
async fn example_4_task_type_inference() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 4: Task Type Inference ---");

    let model = Arc::new(MockModel::new(
        "UNDERSTANDING: Write unit tests\n\
         APPROACH: Use #[tokio::test] for async tests\n\
         COMPLEXITY: Moderate\n\
         REQUIREMENTS: tokio-test"
            .to_string(),
    ));

    let engine = PlanningEngine::new(model);

    // These requests will automatically infer task types
    let test_cases = vec![
        ("Write unit tests for PromptBuilder", "testing"),
        ("Refactor the understanding engine", "refactoring"),
        ("Fix the compilation error", "debugging"),
        ("Document the API", "documentation"),
        ("Optimize string operations", "optimization"),
        ("Read configuration from file", "file_operations"),
        ("Run cargo test command", "command_execution"),
        ("Create a new module", "code_generation"),
    ];

    for (request, expected_type) in test_cases {
        println!("Request: {}", request);
        println!("Expected type: {}", expected_type);

        let plan = engine.analyze_task(request).await?;
        println!("Result: {} ({})\n", plan.understanding, plan.approach);
    }

    Ok(())
}

/// Example 5: Dynamic template modification
fn example_5_dynamic_template() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 5: Dynamic Template Modification ---");

    let mut template = PromptTemplate::default();

    // Add project rules
    template.set_project_rules(ProjectRules {
        tech_stack: vec![
            "Rust 2021".to_string(),
            "Tokio".to_string(),
            "Custom Framework".to_string(),
        ],
        conventions: vec![
            "Use snake_case for functions".to_string(),
            "Add doc comments".to_string(),
            "Write comprehensive tests".to_string(),
        ],
        context: Some("Building a custom AI task runner".to_string()),
        architecture: Some("Modular, layered architecture".to_string()),
    });

    // Add custom scenario
    template.add_scenario(
        "custom_integration".to_string(),
        ScenarioPrompt {
            name: "Custom Integration".to_string(),
            description: "Integrate with external systems".to_string(),
            instructions: vec![
                "Design the integration interface".to_string(),
                "Implement error handling".to_string(),
                "Add retry logic".to_string(),
                "Write integration tests".to_string(),
            ],
            output_structure: Some(
                "UNDERSTANDING: [integration requirements]\n\
                 APPROACH: [integration strategy]\n\
                 COMPLEXITY: [Simple/Moderate/Complex]\n\
                 REQUIREMENTS: [dependencies]"
                    .to_string(),
            ),
            examples: vec![],
        },
    );

    // Save modified template
    match template.to_file("prompts/custom-generated.yaml") {
        Ok(_) => println!("Template saved to prompts/custom-generated.yaml"),
        Err(e) => println!("Note: Could not save template: {}", e),
    }

    // Use the modified template
    let builder = PromptBuilder::new(template);
    let prompt = builder
        .task_type("custom_integration")
        .context("system", "External API")
        .build("Integrate with payment gateway");

    println!("\nGenerated prompt with custom scenario (first 600 chars):");
    println!("{}\n", &prompt[..prompt.len().min(600)]);

    Ok(())
}

