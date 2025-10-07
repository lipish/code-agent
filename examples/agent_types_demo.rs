//! Agent Types Demo
//!
//! This example demonstrates the different agent types available
//! and how to use them for specialized tasks.

use task_runner::prompts::{AgentType, PromptBuilder, PromptTemplate, global_template_for_agent};

fn main() {
    println!("=== Agent Types Demo ===\n");
    println!("Task Runner supports multiple specialized agent types:\n");

    // Show all available agent types
    println!("📋 Available Agent Types:\n");
    for (i, agent_type) in AgentType::all().iter().enumerate() {
        println!("{}. {:?} Agent", i + 1, agent_type);
    }
    println!();

    // Demonstrate each agent type
    demonstrate_agent(AgentType::Generic, "Help me with this task");
    demonstrate_agent(AgentType::Code, "Refactor error handling in agent.rs");
    demonstrate_agent(AgentType::Data, "Analyze user activity logs and generate report");
    demonstrate_agent(AgentType::DevOps, "Set up CI/CD pipeline with GitHub Actions");
    demonstrate_agent(AgentType::Api, "Design REST API for user management");
    demonstrate_agent(AgentType::Testing, "Write integration tests for authentication");
    demonstrate_agent(AgentType::Documentation, "Document the API endpoints");
    demonstrate_agent(AgentType::Security, "Review authentication implementation for vulnerabilities");

    println!("\n\n💡 How to Use Different Agent Types:\n");
    println!("1. Choose the appropriate agent type for your task");
    println!("2. Create a template with that agent type");
    println!("3. Build prompts using the specialized template");
    println!("\nExample code:");
    println!("```rust");
    println!("// Create a Data Agent template");
    println!("let template = PromptTemplate {{");
    println!("    global: global_template_for_agent(AgentType::Data),");
    println!("    project: None,");
    println!("    scenarios: HashMap::new(),");
    println!("}};");
    println!();
    println!("// Use it with PromptBuilder");
    println!("let builder = PromptBuilder::new(template);");
    println!("let prompt = builder.build(\"Analyze sales data\");");
    println!("```");
}

fn demonstrate_agent(agent_type: AgentType, task: &str) {
    println!("\n{}", "=".repeat(80));
    println!("🤖 {:?} Agent", agent_type);
    println!("{}", "=".repeat(80));
    println!("\n📝 Task: {}\n", task);

    // Create template for this agent type
    let mut template = PromptTemplate::default();
    template.global = global_template_for_agent(agent_type);

    // Build prompt
    let builder = PromptBuilder::new(template);
    let prompt = builder.build(task);

    // Extract and show the system role
    if let Some(role_start) = prompt.find("# System Role\n") {
        let role_section_start = role_start + "# System Role\n".len();
        let role_section_end = prompt[role_section_start..]
            .find("\n\n")
            .map(|p| role_section_start + p)
            .unwrap_or(prompt.len());
        
        let role = &prompt[role_section_start..role_section_end];
        
        // Show first few lines of the role
        let lines: Vec<&str> = role.lines().take(5).collect();
        println!("🎭 Agent Role (preview):");
        for line in lines {
            println!("   {}", line);
        }
        if role.lines().count() > 5 {
            println!("   ...");
        }
    }

    // Show what this agent is good at
    println!("\n✨ Specialized For:");
    match agent_type {
        AgentType::Generic => {
            println!("   • Adapts to any task or domain");
            println!("   • No predefined specialization");
            println!("   • Results depend entirely on your prompt");
        }
        AgentType::Code => {
            println!("   • Software development and architecture");
            println!("   • Code refactoring and optimization");
            println!("   • Debugging and error handling");
        }
        AgentType::Data => {
            println!("   • Data extraction, transformation, loading (ETL)");
            println!("   • Data analysis and visualization");
            println!("   • Statistical analysis and reporting");
        }
        AgentType::DevOps => {
            println!("   • CI/CD pipeline design");
            println!("   • Container orchestration");
            println!("   • Infrastructure as Code");
        }
        AgentType::Api => {
            println!("   • RESTful and GraphQL API design");
            println!("   • API documentation");
            println!("   • Authentication and authorization");
        }
        AgentType::Testing => {
            println!("   • Unit, integration, end-to-end testing");
            println!("   • Test-driven development (TDD)");
            println!("   • Test automation frameworks");
        }
        AgentType::Documentation => {
            println!("   • API documentation");
            println!("   • User guides and tutorials");
            println!("   • Architecture documentation");
        }
        AgentType::Security => {
            println!("   • Security vulnerability assessment");
            println!("   • Secure coding practices");
            println!("   • Authentication and authorization");
        }
    }
}

