//! Task Planner
//!
//! This module handles task planning and strategy determination based on
//! the understanding provided by the AI model.

use crate::types::{TaskComplexity, TaskPlan};

/// Task Planner
///
/// Responsible for creating execution plans based on task understanding.
/// It analyzes the task requirements and determines the best execution strategy.
pub struct TaskPlanner {
    // Future: Add planning strategies, optimization rules, etc.
}

impl TaskPlanner {
    /// Create a new task planner
    pub fn new() -> Self {
        Self {}
    }

    /// Create an execution plan based on task understanding
    ///
    /// # Arguments
    ///
    /// * `understanding` - The AI's understanding of the task
    /// * `approach` - The proposed approach to solve the task
    /// * `complexity` - The estimated complexity of the task
    ///
    /// # Returns
    ///
    /// A `TaskPlan` containing the execution strategy
    pub fn create_plan(
        &self,
        understanding: String,
        approach: String,
        complexity: TaskComplexity,
    ) -> TaskPlan {
        let estimated_steps = self.estimate_steps(&complexity);
        let requirements = self.extract_requirements(&approach);

        TaskPlan {
            understanding,
            approach,
            complexity,
            estimated_steps: Some(estimated_steps),
            requirements,
        }
    }

    /// Estimate the number of steps required based on complexity
    fn estimate_steps(&self, complexity: &TaskComplexity) -> u32 {
        match complexity {
            TaskComplexity::Simple => 1,
            TaskComplexity::Moderate => 3,
            TaskComplexity::Complex => 5,
        }
    }

    /// Extract requirements from the approach description
    fn extract_requirements(&self, approach: &str) -> Vec<String> {
        let mut requirements = Vec::new();
        let lower = approach.to_lowercase();

        // Common dependencies
        if lower.contains("serde") {
            requirements.push("serde".to_string());
        }
        if lower.contains("tokio") {
            requirements.push("tokio".to_string());
        }
        if lower.contains("async") {
            requirements.push("async runtime".to_string());
        }
        if lower.contains("file") {
            requirements.push("file system access".to_string());
        }
        if lower.contains("network") || lower.contains("http") {
            requirements.push("network access".to_string());
        }

        requirements
    }

    /// Analyze task complexity based on keywords
    pub fn analyze_complexity(&self, task_description: &str) -> TaskComplexity {
        let lower = task_description.to_lowercase();
        let mut complexity_score = 0;

        // Increase complexity for certain keywords
        if lower.contains("multiple") || lower.contains("several") {
            complexity_score += 2;
        }
        if lower.contains("complex") || lower.contains("advanced") {
            complexity_score += 2;
        }
        if lower.contains("integrate") || lower.contains("combine") {
            complexity_score += 1;
        }
        if lower.contains("optimize") || lower.contains("refactor") {
            complexity_score += 1;
        }

        match complexity_score {
            0..=1 => TaskComplexity::Simple,
            2..=3 => TaskComplexity::Moderate,
            _ => TaskComplexity::Complex,
        }
    }

    /// Determine if a task requires specific tools
    pub fn required_tools(&self, task_description: &str) -> Vec<String> {
        let mut tools = Vec::new();
        let lower = task_description.to_lowercase();

        if lower.contains("read") && lower.contains("file") {
            tools.push("read_file".to_string());
        }
        if lower.contains("write") && lower.contains("file") {
            tools.push("write_file".to_string());
        }
        if lower.contains("list") && lower.contains("file") {
            tools.push("list_files".to_string());
        }
        if lower.contains("run") && lower.contains("command") {
            tools.push("run_command".to_string());
        }

        tools
    }
}

impl Default for TaskPlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planner_creation() {
        let planner = TaskPlanner::new();
        let plan = planner.create_plan(
            "Simple task".to_string(),
            "Just do it".to_string(),
            TaskComplexity::Simple,
        );
        
        assert_eq!(plan.complexity, TaskComplexity::Simple);
        assert_eq!(plan.estimated_steps, Some(1));
    }

    #[test]
    fn test_estimate_steps() {
        let planner = TaskPlanner::new();
        
        assert_eq!(planner.estimate_steps(&TaskComplexity::Simple), 1);
        assert_eq!(planner.estimate_steps(&TaskComplexity::Moderate), 3);
        assert_eq!(planner.estimate_steps(&TaskComplexity::Complex), 5);
    }

    #[test]
    fn test_extract_requirements() {
        let planner = TaskPlanner::new();
        
        let reqs = planner.extract_requirements("Use serde for serialization");
        assert!(reqs.contains(&"serde".to_string()));
        
        let reqs = planner.extract_requirements("Async file operations with tokio");
        assert!(reqs.contains(&"tokio".to_string()));
        assert!(reqs.contains(&"async runtime".to_string()));
        assert!(reqs.contains(&"file system access".to_string()));
    }

    #[test]
    fn test_analyze_complexity() {
        let planner = TaskPlanner::new();
        
        assert_eq!(
            planner.analyze_complexity("Read a file"),
            TaskComplexity::Simple
        );
        
        assert_eq!(
            planner.analyze_complexity("Integrate multiple services"),
            TaskComplexity::Moderate
        );
        
        assert_eq!(
            planner.analyze_complexity("Complex optimization of multiple advanced systems"),
            TaskComplexity::Complex
        );
    }

    #[test]
    fn test_required_tools() {
        let planner = TaskPlanner::new();
        
        let tools = planner.required_tools("Read file config.toml");
        assert!(tools.contains(&"read_file".to_string()));
        
        let tools = planner.required_tools("List files in directory");
        assert!(tools.contains(&"list_files".to_string()));
        
        let tools = planner.required_tools("Run command ls -la");
        assert!(tools.contains(&"run_command".to_string()));
    }
}

