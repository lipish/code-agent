//! Understanding Engine - AI-powered task analysis

use crate::types::TaskPlan;
use crate::models::LanguageModel;
use crate::errors::AgentError;

/// Understanding engine for analyzing tasks
pub struct UnderstandingEngine {
    model: std::sync::Arc<dyn LanguageModel>,
}

impl UnderstandingEngine {
    pub fn new(model: std::sync::Arc<dyn LanguageModel>) -> Self {
        Self { model }
    }

    /// Understand and analyze a task
    pub async fn understand(&self, request: &str) -> Result<TaskPlan, AgentError> {
        let prompt = self.build_understanding_prompt(request);
        let response = self.model.complete(&prompt).await?;

        self.parse_task_plan(&response.content)
    }

    fn build_understanding_prompt(&self, request: &str) -> String {
        format!(
            "You are an intelligent coding assistant with full autonomy.

TASK TO ANALYZE: {request}

Please analyze this task and provide:
1. Your understanding of what the user wants
2. Your approach to solving it
3. Assessment of complexity (Simple/Moderate/Complex)
4. Any requirements or dependencies you identify

You have complete freedom in how to structure your response. Be thorough but concise."
        )
    }

    fn parse_task_plan(&self, response: &str) -> Result<TaskPlan, AgentError> {
        let complexity = if response.to_lowercase().contains("simple") {
            crate::types::TaskComplexity::Simple
        } else if response.to_lowercase().contains("complex") {
            crate::types::TaskComplexity::Complex
        } else {
            crate::types::TaskComplexity::Moderate
        };

        let estimated_steps = if response.to_lowercase().contains("step") {
            Some(self.extract_step_count(response))
        } else {
            None
        };

        Ok(TaskPlan {
            understanding: response.to_string(),
            approach: "AI-determined approach".to_string(),
            complexity,
            estimated_steps,
            requirements: vec![],
        })
    }

    fn extract_step_count(&self, response: &str) -> u32 {
        // Simple step count extraction logic
        let words: Vec<&str> = response.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if word.to_lowercase() == "step" && i > 0 {
                if let Some(num_word) = words.get(i - 1) {
                    if let Ok(num) = num_word.parse::<u32>() {
                        return num;
                    }
                }
            }
        }
        3 // Default value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MockModel;

    #[tokio::test]
    async fn test_understanding_engine() {
        let model = std::sync::Arc::new(MockModel::new("test".to_string()));
        let engine = UnderstandingEngine::new(model);

        let plan = engine.understand("Create a simple hello world program").await.unwrap();

        assert!(!plan.understanding.is_empty());
        assert!(!plan.approach.is_empty());
    }
}