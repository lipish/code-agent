//! Prompt Engineering System
//!
//! A flexible, layered prompt system inspired by OpenAI Codex and Roo-Code.
//! Supports global templates, project-level rules, and scenario-specific instructions.

mod defaults;

pub use defaults::{
    SYSTEM_ROLE, OUTPUT_FORMAT_TYPE, REQUIRED_FIELDS, FIELD_DESCRIPTIONS,
    CORE_PRINCIPLES, CODE_QUALITY, SAFETY,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Prompt template with layered structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    /// Global base template
    pub global: GlobalTemplate,
    /// Project-level rules (optional)
    pub project: Option<ProjectRules>,
    /// Scenario-specific instructions
    pub scenarios: HashMap<String, ScenarioPrompt>,
}

/// Global template - base framework for all prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTemplate {
    /// System role definition
    pub system_role: String,
    /// Output format requirements
    pub output_format: OutputFormat,
    /// General constraints
    pub constraints: Vec<String>,
}

/// Project-level rules - tech stack and conventions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRules {
    /// Technology stack
    pub tech_stack: Vec<String>,
    /// Coding conventions
    pub conventions: Vec<String>,
    /// Project context
    pub context: Option<String>,
    /// Architecture guidelines
    pub architecture: Option<String>,
}

/// Scenario-specific prompt for different task types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioPrompt {
    /// Scenario name
    pub name: String,
    /// Scenario description
    pub description: String,
    /// Specific instructions
    pub instructions: Vec<String>,
    /// Expected output structure
    pub output_structure: Option<String>,
    /// Examples (optional)
    pub examples: Vec<PromptExample>,
}

/// Output format specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFormat {
    /// Format type (json, markdown, structured_text)
    pub format_type: String,
    /// Required fields
    pub required_fields: Vec<String>,
    /// Field descriptions
    pub field_descriptions: HashMap<String, String>,
}

/// Example for few-shot learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptExample {
    /// Input example
    pub input: String,
    /// Expected output
    pub output: String,
}

/// Prompt builder with fluent API
pub struct PromptBuilder {
    template: PromptTemplate,
    task_type: Option<String>,
    context: HashMap<String, String>,
}

impl PromptBuilder {
    /// Create a new prompt builder with template
    pub fn new(template: PromptTemplate) -> Self {
        Self {
            template,
            task_type: None,
            context: HashMap::new(),
        }
    }

    /// Set task type (scenario)
    pub fn task_type(mut self, task_type: &str) -> Self {
        self.task_type = Some(task_type.to_string());
        self
    }

    /// Add context variable
    pub fn context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// Build the final prompt
    pub fn build(&self, user_request: &str) -> String {
        let mut prompt = String::new();

        // 1. System role (global)
        prompt.push_str(&format!("# System Role\n{}\n\n", self.template.global.system_role));

        // 2. Project context (if available)
        if let Some(ref project) = self.template.project {
            prompt.push_str("# Project Context\n");
            
            if !project.tech_stack.is_empty() {
                prompt.push_str(&format!("**Tech Stack**: {}\n", project.tech_stack.join(", ")));
            }
            
            if !project.conventions.is_empty() {
                prompt.push_str("**Conventions**:\n");
                for conv in &project.conventions {
                    prompt.push_str(&format!("- {}\n", conv));
                }
            }
            
            if let Some(ref ctx) = project.context {
                prompt.push_str(&format!("\n**Context**: {}\n", ctx));
            }
            
            prompt.push('\n');
        }

        // 3. Scenario-specific instructions
        if let Some(ref task_type) = self.task_type {
            if let Some(scenario) = self.template.scenarios.get(task_type) {
                prompt.push_str(&format!("# Task Type: {}\n", scenario.name));
                prompt.push_str(&format!("{}\n\n", scenario.description));
                
                if !scenario.instructions.is_empty() {
                    prompt.push_str("**Instructions**:\n");
                    for (i, instr) in scenario.instructions.iter().enumerate() {
                        prompt.push_str(&format!("{}. {}\n", i + 1, instr));
                    }
                    prompt.push('\n');
                }

                // Add examples if available
                if !scenario.examples.is_empty() {
                    prompt.push_str("**Examples**:\n");
                    for (i, example) in scenario.examples.iter().enumerate() {
                        prompt.push_str(&format!("\nExample {}:\n", i + 1));
                        prompt.push_str(&format!("Input: {}\n", example.input));
                        prompt.push_str(&format!("Output: {}\n", example.output));
                    }
                    prompt.push('\n');
                }
            }
        }

        // 4. User request (with delimiters)
        prompt.push_str("---\n\n");
        prompt.push_str(&format!("# User Request\n```\n{}\n```\n\n", user_request));

        // 5. Output format requirements
        prompt.push_str("# Output Format\n");
        prompt.push_str(&format!("Format: {}\n\n", self.template.global.output_format.format_type));
        
        if !self.template.global.output_format.required_fields.is_empty() {
            prompt.push_str("**Required Fields**:\n");
            for field in &self.template.global.output_format.required_fields {
                if let Some(desc) = self.template.global.output_format.field_descriptions.get(field) {
                    prompt.push_str(&format!("- `{}`: {}\n", field, desc));
                } else {
                    prompt.push_str(&format!("- `{}`\n", field));
                }
            }
            prompt.push('\n');
        }

        // 6. Constraints
        if !self.template.global.constraints.is_empty() {
            prompt.push_str("# Constraints\n");
            for constraint in &self.template.global.constraints {
                prompt.push_str(&format!("- {}\n", constraint));
            }
            prompt.push('\n');
        }

        // 7. Additional context variables
        if !self.context.is_empty() {
            prompt.push_str("# Additional Context\n");
            for (key, value) in &self.context {
                prompt.push_str(&format!("**{}**: {}\n", key, value));
            }
            prompt.push('\n');
        }

        prompt.push_str("---\n\n");
        prompt.push_str("Please analyze the request and provide your response following the specified format.\n");

        prompt
    }
}

impl Default for PromptTemplate {
    fn default() -> Self {
        defaults::default_template()
    }
}

impl PromptTemplate {
    /// Load template from YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let template: PromptTemplate = serde_yaml::from_str(&content)?;
        Ok(template)
    }

    /// Save template to YAML file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add a scenario
    pub fn add_scenario(&mut self, key: String, scenario: ScenarioPrompt) {
        self.scenarios.insert(key, scenario);
    }

    /// Set project rules
    pub fn set_project_rules(&mut self, rules: ProjectRules) {
        self.project = Some(rules);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_template() {
        let template = PromptTemplate::default();
        assert_eq!(template.global.output_format.required_fields.len(), 4);
    }

    #[test]
    fn test_prompt_builder() {
        let template = PromptTemplate::default();
        let builder = PromptBuilder::new(template);
        let prompt = builder
            .context("language", "Rust")
            .build("Create a hello world program");
        
        assert!(prompt.contains("System Role"));
        assert!(prompt.contains("User Request"));
        assert!(prompt.contains("hello world"));
    }
}

