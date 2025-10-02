//! Execution Engine - AI-powered task execution

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::types::{ExecutionContext, ExecutionDecision, ExecutionResult, ActionType, StepResult};
use crate::models::LanguageModel;
use crate::tools::{ToolRegistry, ToolCall, ToolArgs};
use crate::errors::AgentError;

/// Execution engine for running tasks
pub struct ExecutionEngine {
    model: Arc<dyn LanguageModel>,
    tools: Arc<Mutex<ToolRegistry>>,
    config: crate::config::ExecutionConfig,
}

impl ExecutionEngine {
    pub fn new(model: Arc<dyn LanguageModel>, tools: Arc<Mutex<ToolRegistry>>, config: crate::config::ExecutionConfig) -> Self {
        Self { model, tools, config }
    }

    /// Execute a task plan
    pub async fn execute(&self, task_id: &str, plan: crate::types::TaskPlan) -> Result<ExecutionResult, AgentError> {
        let start_time = std::time::Instant::now();
        let mut context = ExecutionContext {
            task_id: task_id.to_string(),
            plan,
            current_step: 0,
            results: vec![],
            variables: HashMap::new(),
        };

        loop {
            // Check step limit
            if context.current_step >= self.config.max_steps {
                return Ok(ExecutionResult {
                    success: false,
                    summary: "Maximum steps exceeded".to_string(),
                    details: "Task exceeded maximum allowed steps".to_string(),
                    execution_time: start_time.elapsed().as_secs(),
                });
            }

            // AI decides next action
            let decision = self.make_execution_decision(&context).await?;

            let action_taken = decision.action_type.clone();
            let reasoning_used = decision.reasoning.clone();
            let confidence_used = decision.confidence;

            match action_taken {
                ActionType::UseTool { ref tool_name, ref arguments } => {
                    let tool_result = self.execute_tool(tool_name, arguments.clone()).await?;

                    context.results.push(crate::types::ExecutionStep {
                        step_number: context.current_step,
                        action: crate::types::Action {
                            action_type: action_taken.clone(),
                            reasoning: reasoning_used.clone(),
                            confidence: confidence_used,
                        },
                        result: Some(tool_result.clone()),
                        timestamp: chrono::Utc::now(),
                    });

                    if !tool_result.success {
                        return Ok(ExecutionResult {
                            success: false,
                            summary: "Tool execution failed".to_string(),
                            details: tool_result.error.unwrap_or_default(),
                            execution_time: start_time.elapsed().as_secs(),
                        });
                    }
                }

                ActionType::Complete { ref summary } => {
                    return Ok(ExecutionResult {
                        success: true,
                        summary: summary.clone(),
                        details: self.format_execution_details(&context),
                        execution_time: start_time.elapsed().as_secs(),
                    });
                }

                ActionType::Think { reasoning: _ } => {
                    context.results.push(crate::types::ExecutionStep {
                        step_number: context.current_step,
                        action: crate::types::Action {
                            action_type: action_taken.clone(),
                            reasoning: reasoning_used.clone(),
                            confidence: confidence_used,
                        },
                        result: None,
                        timestamp: chrono::Utc::now(),
                    });
                }

                ActionType::AskClarification { ref question } => {
                    return Ok(ExecutionResult {
                        success: false,
                        summary: "Clarification needed".to_string(),
                        details: format!("Question: {}", question),
                        execution_time: start_time.elapsed().as_secs(),
                    });
                }
            }

            context.current_step += 1;
        }
    }

    async fn make_execution_decision(&self, context: &ExecutionContext) -> Result<ExecutionDecision, AgentError> {
        let prompt = self.build_execution_prompt(context);
        let response = self.model.complete_with_tools(&prompt, &self.get_tool_definitions()).await?;

        self.parse_execution_decision(&response.content)
    }

    fn build_execution_prompt(&self, context: &ExecutionContext) -> String {
        format!(
            "You are working on this task: {}

Current progress: Step {} of estimated {}

You have these tools available: {}

You have complete freedom to decide your next action. You can:
- Use a tool to accomplish part of the task
- Think through the next steps
- Complete the task if you believe it's done
- Ask for clarification if needed

Decide your next action and respond in this format:
Action: [UseTool/Think/Complete/AskClarification]
Tool: [tool name if using tool]
Arguments: [tool arguments if using tool]
Reasoning: [your reasoning]
Confidence: [0.0-1.0]",
            context.plan.understanding,
            context.current_step + 1,
            context.plan.estimated_steps.unwrap_or(0),
            self.get_tool_descriptions()
        )
    }

    fn get_tool_descriptions(&self) -> String {
        let tools = self.tools.blocking_lock();
        let mut descriptions = Vec::new();

        for tool_name in tools.get_tool_names() {
            if let Some(tool) = tools.get_tool(&tool_name) {
                descriptions.push(format!("{}: {}", tool.name(), tool.description()));
            }
        }

        descriptions.join("\n")
    }

    fn get_tool_definitions(&self) -> Vec<crate::models::ToolDefinition> {
        let tools = self.tools.blocking_lock();
        let mut definitions = Vec::new();

        for tool_name in tools.get_tool_names() {
            if let Some(tool) = tools.get_tool(&tool_name) {
                let _parameters: Vec<serde_json::Value> = tool.parameters()
                    .into_iter()
                    .map(|p| {
                        serde_json::json!({
                            "type": "string",
                            "description": p.description
                        })
                    })
                    .collect();

                definitions.push(crate::models::ToolDefinition {
                    name: tool.name().to_string(),
                    description: tool.description().to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {},
                        "required": []
                    }),
                });
            }
        }

        definitions
    }

    async fn execute_tool(&self, tool_name: &str, arguments: HashMap<String, serde_json::Value>) -> Result<StepResult, AgentError> {
        let tools = self.tools.lock().await;
        let tool_call = ToolCall {
            name: tool_name.to_string(),
            args: ToolArgs::from_map(arguments),
        };

        match tools.execute(&tool_call).await {
            Ok(result) => Ok(StepResult {
                success: result.success,
                output: serde_json::Value::String(result.content),
                error: result.error,
                execution_time: 0, // Simplified implementation
            }),
            Err(error) => Ok(StepResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(error.to_string()),
                execution_time: 0,
            }),
        }
    }

    fn format_execution_details(&self, context: &ExecutionContext) -> String {
        format!(
            "Task completed in {} steps. All objectives achieved.",
            context.current_step
        )
    }

    fn parse_execution_decision(&self, response: &str) -> Result<ExecutionDecision, AgentError> {
        // Simple parsing logic for demonstration
        let lines: Vec<&str> = response.lines().collect();
        let mut action_type = ActionType::Think {
            reasoning: "Continuing analysis".to_string(),
        };
        let mut reasoning = "AI reasoning".to_string();
        let mut confidence = 0.7;

        for line in lines {
            if line.to_lowercase().starts_with("action:") {
                let action = line[7..].trim().to_lowercase();
                action_type = match action.as_str() {
                    "usetool" => ActionType::UseTool {
                        tool_name: "unknown".to_string(),
                        arguments: HashMap::new(),
                    },
                    "complete" => ActionType::Complete {
                        summary: "Task completed".to_string(),
                    },
                    "think" => ActionType::Think {
                        reasoning: "Thinking through next steps".to_string(),
                    },
                    "askclarification" => ActionType::AskClarification {
                        question: "Need clarification".to_string(),
                    },
                    _ => action_type,
                };
            } else if line.to_lowercase().starts_with("reasoning:") {
                reasoning = line[10..].trim().to_string();
            } else if line.to_lowercase().starts_with("confidence:") {
                if let Ok(conf) = line[11..].trim().parse::<f32>() {
                    confidence = conf;
                }
            }
        }

        Ok(ExecutionDecision {
            action_type,
            reasoning,
            confidence,
        })
    }
}