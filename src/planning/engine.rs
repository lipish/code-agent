//! Task Planning Engine - AI-powered task analysis and execution planning

use crate::errors::AgentError;
use crate::models::LanguageModel;
use crate::prompts::{PromptBuilder, PromptTemplate};
use crate::types::{TaskComplexity, TaskPlan};
use std::sync::Arc;

/// Configuration for the planning engine
#[derive(Debug, Clone)]
pub struct PlanningConfig {
    /// Whether to enable verbose logging
    pub verbose: bool,
    /// Maximum retries for AI model calls
    pub max_retries: u32,
    /// Whether to use task type inference
    pub auto_infer_type: bool,
}

impl Default for PlanningConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            max_retries: 3,
            auto_infer_type: true,
        }
    }
}

/// Planning engine for analyzing tasks and creating execution plans
///
/// This engine uses AI models to:
/// - Analyze task requirements and intent
/// - Create detailed execution plans
/// - Estimate task complexity
/// - Identify required tools and resources
///
/// # Features
///
/// - **Automatic task type inference**: Detects task category automatically
/// - **Custom prompt templates**: Supports domain-specific prompts
/// - **Configurable behavior**: Adjustable retry logic and logging
/// - **Retry mechanism**: Automatic retry on failures
///
/// # Examples
///
/// ```no_run
/// use task_runner::planning::{PlanningEngine, PlanningConfig};
/// use task_runner::models::MockModel;
/// use std::sync::Arc;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let model = Arc::new(MockModel::new("gpt-4".to_string()));
/// let engine = PlanningEngine::new(model);
/// let plan = engine.analyze_task("Build a REST API server").await?;
/// println!("Plan: {}", plan.approach);
/// # Ok(())
/// # }
/// ```
pub struct PlanningEngine {
    model: Arc<dyn LanguageModel>,
    prompt_template: PromptTemplate,
    config: PlanningConfig,
}

impl PlanningEngine {
    /// Create a new planning engine with default template and config
    pub fn new(model: Arc<dyn LanguageModel>) -> Self {
        Self {
            model,
            prompt_template: PromptTemplate::default(),
            config: PlanningConfig::default(),
        }
    }

    /// Create a new planning engine with custom template
    pub fn with_template(model: Arc<dyn LanguageModel>, template: PromptTemplate) -> Self {
        Self {
            model,
            prompt_template: template,
            config: PlanningConfig::default(),
        }
    }

    /// Create a new planning engine with custom config
    pub fn with_config(model: Arc<dyn LanguageModel>, config: PlanningConfig) -> Self {
        Self {
            model,
            prompt_template: PromptTemplate::default(),
            config,
        }
    }

    /// Create a new planning engine with custom template and config
    pub fn with_template_and_config(
        model: Arc<dyn LanguageModel>,
        template: PromptTemplate,
        config: PlanningConfig,
    ) -> Self {
        Self {
            model,
            prompt_template: template,
            config,
        }
    }

    /// Load template from file
    pub fn load_template(&mut self, path: &str) -> Result<(), AgentError> {
        self.prompt_template = PromptTemplate::from_file(path)
            .map_err(|e| AgentError::ConfigError(format!("Failed to load prompt template: {}", e)))?;
        Ok(())
    }

    /// Analyze a task and create an execution plan
    ///
    /// This is the main entry point for task analysis. It uses AI to understand
    /// the task requirements and creates a detailed execution plan.
    ///
    /// # Arguments
    ///
    /// * `request` - The task request in natural language
    ///
    /// # Returns
    ///
    /// A `TaskPlan` containing the analysis results
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use task_runner::planning::{PlanningEngine, PlanningConfig};
    /// use task_runner::models::MockModel;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let model = Arc::new(MockModel::new("test".to_string()));
    /// let engine = PlanningEngine::new(model);
    /// let plan = engine.analyze_task("Create a configuration loader").await?;
    /// println!("Task complexity: {:?}", plan.complexity);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn analyze_task(&self, request: &str) -> Result<TaskPlan, AgentError> {
        self.analyze_task_with_type(request, None).await
    }



    /// Analyze a task with specific task type (scenario)
    ///
    /// # Arguments
    ///
    /// * `request` - The task request in natural language
    /// * `task_type` - Optional task type/scenario (e.g., "code_generation", "debugging")
    ///
    /// # Returns
    ///
    /// A `TaskPlan` containing the analysis results
    pub async fn analyze_task_with_type(
        &self,
        request: &str,
        task_type: Option<&str>,
    ) -> Result<TaskPlan, AgentError> {
        if self.config.verbose {
            tracing::info!("🧠 Starting task analysis for: {}", request);
        }

        let prompt = self.build_understanding_prompt(request, task_type);

        if self.config.verbose {
            tracing::debug!("📝 Sending prompt to AI model");
            tracing::trace!("Full prompt:\n{}", prompt);
        }

        // Call AI model with retry logic
        let response = self.call_model_with_retry(&prompt).await?;

        if self.config.verbose {
            tracing::debug!("🤖 AI model response: {}", response);
        }

        let plan = self.parse_task_plan(&response)?;

        if self.config.verbose {
            tracing::info!(
                "📋 Task plan created - Complexity: {:?}, Steps: {}",
                plan.complexity,
                plan.estimated_steps.unwrap_or(0)
            );
        }

        Ok(plan)
    }

    /// Call AI model with retry logic
    async fn call_model_with_retry(&self, prompt: &str) -> Result<String, AgentError> {
        let mut last_error = None;

        for attempt in 1..=self.config.max_retries {
            match self.model.complete(prompt).await {
                Ok(response) => return Ok(response.content),
                Err(e) => {
                    if self.config.verbose {
                        tracing::warn!("AI model call failed (attempt {}/{}): {}",
                            attempt, self.config.max_retries, e);
                    }
                    last_error = Some(e.clone());

                    // Don't sleep on the last attempt
                    if attempt < self.config.max_retries {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }

        Err(AgentError::ModelError(
            last_error.unwrap_or_else(||
                crate::errors::ModelError::APIError("Unknown error after retries".to_string())
            )
        ))
    }

    /// Build the prompt for task understanding using the template system
    fn build_understanding_prompt(&self, request: &str, task_type: Option<&str>) -> String {
        let mut builder = PromptBuilder::new(self.prompt_template.clone());

        // Set task type if provided
        if let Some(tt) = task_type {
            builder = builder.task_type(tt);
        } else {
            // Try to infer task type from request
            let inferred_type = self.infer_task_type(request);
            if let Some(tt) = inferred_type {
                builder = builder.task_type(&tt);
            }
        }

        builder.build(request)
    }

    /// Infer task type from request content
    fn infer_task_type(&self, request: &str) -> Option<String> {
        let lower = request.to_lowercase();

        if lower.contains("test") || lower.contains("unit test") {
            Some("testing".to_string())
        } else if lower.contains("refactor") || lower.contains("improve") {
            Some("refactoring".to_string())
        } else if lower.contains("debug") || lower.contains("fix") || lower.contains("error") {
            Some("debugging".to_string())
        } else if lower.contains("document") || lower.contains("doc") {
            Some("documentation".to_string())
        } else if lower.contains("optimize") || lower.contains("performance") {
            Some("optimization".to_string())
        } else if lower.contains("design") || lower.contains("architecture") {
            Some("architecture".to_string())
        } else if lower.contains("read") || lower.contains("write") || lower.contains("file") {
            Some("file_operations".to_string())
        } else if lower.contains("run") || lower.contains("execute") || lower.contains("command") {
            Some("command_execution".to_string())
        } else if lower.contains("create") || lower.contains("generate") || lower.contains("implement") {
            Some("code_generation".to_string())
        } else {
            None
        }
    }

    /// Parse the AI response into a structured task plan
    fn parse_task_plan(&self, response: &str) -> Result<TaskPlan, AgentError> {
        let mut understanding = String::new();
        let mut approach = String::new();
        let mut complexity = TaskComplexity::Moderate;
        let mut requirements = Vec::new();

        for line in response.lines() {
            let line = line.trim();
            if line.to_uppercase().starts_with("UNDERSTANDING:") {
                understanding = line
                    .strip_prefix("UNDERSTANDING:")
                    .or_else(|| line.strip_prefix("understanding:"))
                    .unwrap_or("")
                    .trim()
                    .to_string();
            } else if line.to_uppercase().starts_with("APPROACH:") {
                approach = line
                    .strip_prefix("APPROACH:")
                    .or_else(|| line.strip_prefix("approach:"))
                    .unwrap_or("")
                    .trim()
                    .to_string();
            } else if line.to_uppercase().starts_with("COMPLEXITY:") {
                let complexity_str = line
                    .strip_prefix("COMPLEXITY:")
                    .or_else(|| line.strip_prefix("complexity:"))
                    .unwrap_or("")
                    .trim()
                    .to_uppercase();
                
                complexity = match complexity_str.as_str() {
                    "SIMPLE" => TaskComplexity::Simple,
                    "COMPLEX" => TaskComplexity::Complex,
                    _ => TaskComplexity::Moderate,
                };
            } else if line.to_uppercase().starts_with("REQUIREMENTS:") {
                let req_text = line
                    .strip_prefix("REQUIREMENTS:")
                    .or_else(|| line.strip_prefix("requirements:"))
                    .unwrap_or("")
                    .trim();
                
                if req_text != "None" && !req_text.is_empty() {
                    requirements = req_text
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
            }
        }

        // Validate that we got at least some understanding
        if understanding.is_empty() {
            understanding = "Task analysis in progress".to_string();
        }
        if approach.is_empty() {
            approach = "Determining best approach".to_string();
        }

        let estimated_steps = match complexity {
            TaskComplexity::Simple => 1,
            TaskComplexity::Moderate => 5,
            TaskComplexity::Complex => 10,
        };

        Ok(TaskPlan {
            understanding,
            approach,
            complexity,
            estimated_steps: Some(estimated_steps),
            requirements,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MockModel;

    #[tokio::test]
    async fn test_planning_engine_creation() {
        let model = Arc::new(MockModel::new("test".to_string()));
        let _engine = PlanningEngine::new(model);
        // Engine created successfully
    }

    #[tokio::test]
    async fn test_parse_task_plan() {
        let model = Arc::new(MockModel::new("test".to_string()));
        let engine = PlanningEngine::new(model);

        let response = "UNDERSTANDING: Read a file\nAPPROACH: Use read_file tool\nCOMPLEXITY: Simple\nREQUIREMENTS: None";
        let plan = engine.parse_task_plan(response).unwrap();

        assert_eq!(plan.understanding, "Read a file");
        assert_eq!(plan.approach, "Use read_file tool");
        assert!(matches!(plan.complexity, TaskComplexity::Simple));
        assert_eq!(plan.estimated_steps, Some(1));
    }

    #[tokio::test]
    async fn test_parse_task_plan_with_requirements() {
        let model = Arc::new(MockModel::new("test".to_string()));
        let engine = PlanningEngine::new(model);

        let response = "UNDERSTANDING: Complex task\nAPPROACH: Multi-step\nCOMPLEXITY: Complex\nREQUIREMENTS: file access, network";
        let plan = engine.parse_task_plan(response).unwrap();

        assert_eq!(plan.requirements.len(), 2);
        assert!(plan.requirements.contains(&"file access".to_string()));
        assert!(plan.requirements.contains(&"network".to_string()));
    }
}

