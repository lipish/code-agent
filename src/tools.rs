//! Tool system for the AI-Native Code Agent

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::ToolError;

/// Tool trait
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Vec<Parameter>;
    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult, ToolError>;
}

/// Tool parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub parameter_type: ParameterType,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

impl Parameter {
    pub fn required(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            required: true,
            parameter_type: ParameterType::String,
            default_value: None,
        }
    }

    pub fn optional(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            required: false,
            parameter_type: ParameterType::String,
            default_value: None,
        }
    }
}

/// Tool arguments
#[derive(Debug, Clone)]
pub struct ToolArgs {
    args: HashMap<String, serde_json::Value>,
}

impl ToolArgs {
    pub fn from_map(args: HashMap<String, serde_json::Value>) -> Self {
        Self { args }
    }

    pub fn get_string(&self, key: &str) -> Result<String, ToolError> {
        self.args.get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| ToolError::InvalidParameters(format!("Missing or invalid parameter: {}", key)))
    }

    pub fn get_string_or(&self, key: &str, default: &str) -> String {
        self.args.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .to_string()
    }
}

/// Tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub content: String,
    pub summary: String,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl ToolResult {
    pub fn text(content: String) -> Self {
        Self {
            success: true,
            summary: content.clone(),
            content,
            data: None,
            error: None,
        }
    }

    pub fn json(data: serde_json::Value) -> Self {
        Self {
            success: true,
            summary: "Operation completed successfully".to_string(),
            content: "Operation completed successfully".to_string(),
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            summary: error.clone(),
            content: String::new(),
            data: None,
            error: Some(error),
        }
    }
}

/// Tool call
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub name: String,
    pub args: ToolArgs,
}

/// Tool registry
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.insert(tool.name().to_string(), Box::new(tool));
    }

    pub async fn execute(&self, tool_call: &ToolCall) -> Result<ToolResult, ToolError> {
        let tool = self.tools.get(&tool_call.name)
            .ok_or_else(|| ToolError::ToolNotFound(tool_call.name.clone()))?;

        tool.execute(&tool_call.args).await
    }

    pub fn get_tool_names(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|tool| tool.as_ref())
    }

    pub fn get_all_tools(&self) -> Vec<&dyn Tool> {
        self.tools.values().map(|tool| tool.as_ref()).collect()
    }
}

// Basic tool implementations

/// Read file tool
pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Read the contents of a file"
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter::required("path", "File path to read")
        ]
    }

    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult, ToolError> {
        let path = args.get_string("path")?;

        // Safety check
        if path.contains("..") || path.starts_with("/") {
            return Err(ToolError::PermissionDenied("Access to this path is not allowed".to_string()));
        }

        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| ToolError::ExecutionError(e.to_string()))?;

        Ok(ToolResult::text(content))
    }
}

/// Write file tool
pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &str {
        "write_file"
    }

    fn description(&self) -> &str {
        "Write content to a file"
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter::required("path", "File path to write"),
            Parameter::required("content", "Content to write"),
        ]
    }

    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult, ToolError> {
        let path = args.get_string("path")?;
        let content = args.get_string("content")?;

        // Safety check
        if path.contains("..") || path.starts_with("/") {
            return Err(ToolError::PermissionDenied("Access to this path is not allowed".to_string()));
        }

        tokio::fs::write(path, content)
            .await
            .map_err(|e| ToolError::ExecutionError(e.to_string()))?;

        Ok(ToolResult::text("File written successfully".to_string()))
    }
}

/// List files tool
pub struct ListFilesTool;

#[async_trait]
impl Tool for ListFilesTool {
    fn name(&self) -> &str {
        "list_files"
    }

    fn description(&self) -> &str {
        "List files and directories in a given path"
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter::required("path", "Directory path to list")
        ]
    }

    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult, ToolError> {
        let path = args.get_string("path")?;

        // Safety check
        if path.contains("..") || path.starts_with("/") {
            return Err(ToolError::PermissionDenied("Access to this path is not allowed".to_string()));
        }

        let mut entries = Vec::new();
        let mut dir = tokio::fs::read_dir(path)
            .await
            .map_err(|e| ToolError::ExecutionError(e.to_string()))?;

        while let Some(entry) = dir.next_entry().await.map_err(|e| ToolError::ExecutionError(e.to_string()))? {
            let metadata = std::fs::metadata(entry.path()).ok();
            entries.push((
                entry.file_name().to_string_lossy().to_string(),
                metadata.map(|m| m.is_dir()).unwrap_or(false)
            ));
        }

        entries.sort_by(|a, b| {
            // Directories first, then files
            match (a.1, b.1) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.0.cmp(&b.0),
            }
        });

        let list_text = entries.iter()
            .map(|(name, is_dir)| {
                let prefix = if *is_dir { "DIR  " } else { "FILE " };
                format!("{}{}", prefix, name)
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ToolResult::text(list_text))
    }
}

/// Run command tool
pub struct RunCommandTool;

#[async_trait]
impl Tool for RunCommandTool {
    fn name(&self) -> &str {
        "run_command"
    }

    fn description(&self) -> &str {
        "Execute a shell command"
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter::required("command", "Command to execute"),
            Parameter::optional("working_dir", "Working directory"),
        ]
    }

    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult, ToolError> {
        let command = args.get_string("command")?;
        let working_dir = args.get_string_or("working_dir", ".");

        // Safety checks for dangerous commands
        let dangerous_commands = vec![
            "rm -rf /", "format", "fdisk", "dd if=", "shutdown", "reboot",
        ];

        for dangerous in &dangerous_commands {
            if command.contains(dangerous) {
                return Err(ToolError::PermissionDenied(format!("Command '{}' is not allowed", dangerous)));
            }
        }

        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .current_dir(working_dir)
            .output()
            .await
            .map_err(|e| ToolError::ExecutionError(e.to_string()))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(ToolResult::text(stdout.to_string()))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Ok(ToolResult::error(stderr.to_string()))
        }
    }
}