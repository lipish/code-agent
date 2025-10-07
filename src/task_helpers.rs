//! Task Helper Functions
//!
//! This module contains utility functions for task execution,
//! including file operations, command execution, and text parsing.

use crate::errors::{AgentError, ToolError};

/// Supported file extensions for file path extraction
const SUPPORTED_FILE_EXTENSIONS: &[&str] = &[
    // Text and documentation
    ".txt", ".md", ".rst", ".adoc",
    // Code files
    ".rs", ".py", ".js", ".ts", ".go", ".java", ".c", ".cpp", ".h", ".hpp",
    // Configuration
    ".toml", ".yaml", ".yml", ".json", ".xml", ".ini", ".conf",
    // Scripts
    ".sh", ".bash", ".zsh", ".fish", ".ps1",
    // Web
    ".html", ".css", ".scss", ".sass", ".vue", ".jsx", ".tsx",
];

/// Common command keywords for command extraction
const COMMAND_KEYWORDS: &[&str] = &[
    "echo", "ls", "cat", "grep", "find", "sed", "awk",
    "cargo", "npm", "yarn", "pnpm", "pip", "poetry",
    "git", "docker", "kubectl", "terraform",
    "make", "cmake", "ninja",
];

/// Extract file path from text
///
/// Attempts to find file paths in the given text by looking for common file extensions.
///
/// # Examples
///
/// ```
/// use task_runner::task_helpers::extract_file_path;
///
/// let text = "Read the file config.toml";
/// assert_eq!(extract_file_path(text), Some("config.toml".to_string()));
/// ```
pub fn extract_file_path(text: &str) -> Option<String> {
    // Simple extraction by looking for common file extensions
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for (i, word) in words.iter().enumerate() {
        if *word == "file" && i + 1 < words.len() {
            let next_word = words[i + 1];
            if has_file_extension(next_word) {
                return Some(next_word.trim_matches('"').trim_matches('\'').to_string());
            }
        }
    }
    
    // Also check for standalone file paths
    for word in words.iter() {
        if has_file_extension(word) {
            return Some(word.trim_matches('"').trim_matches('\'').to_string());
        }
    }
    
    None
}

/// Check if a word has a common file extension
fn has_file_extension(word: &str) -> bool {
    SUPPORTED_FILE_EXTENSIONS.iter().any(|&ext| word.ends_with(ext))
}

/// Extract command from text
///
/// Attempts to extract shell commands from the given text.
///
/// # Examples
///
/// ```
/// use task_runner::task_helpers::extract_command;
///
/// let text = "Run the command echo 'hello'";
/// assert_eq!(extract_command(text), Some("echo 'hello'".to_string()));
/// ```
pub fn extract_command(text: &str) -> Option<String> {
    let lower = text.to_lowercase();

    // Look for common command patterns using predefined keywords
    for keyword in COMMAND_KEYWORDS.iter() {
        if lower.contains(keyword) {
            if let Some(start) = lower.find(keyword) {
                let command_part = &text[start..];
                
                // Try to find the end of the command
                // Look for quotes, newlines, or common sentence endings
                let end_markers = ['\n', '.', '?', '!'];
                let mut end_pos = command_part.len();
                
                for marker in end_markers.iter() {
                    if let Some(pos) = command_part.find(*marker) {
                        if pos < end_pos && pos > 0 {
                            end_pos = pos;
                        }
                    }
                }
                
                let command = command_part[..end_pos].trim().to_string();
                if !command.is_empty() {
                    return Some(command);
                }
            }
        }
    }
    
    None
}

/// Read a file asynchronously
///
/// # Arguments
///
/// * `path` - The path to the file to read
///
/// # Returns
///
/// The file contents as a string, or an error if the file cannot be read.
pub async fn read_file(path: &str) -> Result<String, AgentError> {
    let content = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| AgentError::ToolError(ToolError::ExecutionError(
            format!("Failed to read file '{}': {}", path, e)
        )))?;
    Ok(content)
}

/// List files in a directory asynchronously
///
/// # Arguments
///
/// * `path` - The directory path to list
///
/// # Returns
///
/// A formatted string listing all files and directories, or an error.
pub async fn list_files(path: &str) -> Result<String, AgentError> {
    let mut entries = tokio::fs::read_dir(path)
        .await
        .map_err(|e| AgentError::ToolError(ToolError::ExecutionError(
            format!("Failed to read directory '{}': {}", path, e)
        )))?;

    let mut files = Vec::new();
    while let Some(entry) = entries.next_entry().await
        .map_err(|e| AgentError::ToolError(ToolError::ExecutionError(e.to_string())))? {
        let name = entry.file_name().to_string_lossy().to_string();
        let metadata = entry.metadata().await
            .map_err(|e| AgentError::ToolError(ToolError::ExecutionError(e.to_string())))?;
        let file_type = if metadata.is_dir() { "DIR" } else { "FILE" };
        files.push(format!("{}: {}", file_type, name));
    }

    files.sort();
    Ok(files.join("\n"))
}

/// Run a shell command asynchronously
///
/// # Arguments
///
/// * `command` - The shell command to execute
///
/// # Returns
///
/// The command output (stdout), or an error if the command fails.
///
/// # Security
///
/// This function executes arbitrary shell commands. Ensure proper validation
/// and sanitization of user input before calling this function.
pub async fn run_command(command: &str) -> Result<String, AgentError> {
    let output = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .await
        .map_err(|e| AgentError::ToolError(ToolError::ExecutionError(
            format!("Failed to execute command '{}': {}", command, e)
        )))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(AgentError::ToolError(ToolError::ExecutionError(
            format!("Command '{}' failed: {}", command, stderr)
        )))
    }
}

/// Extract directory path from text
///
/// Attempts to find directory paths in the given text.
pub fn extract_directory_path(text: &str) -> Option<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for (i, word) in words.iter().enumerate() {
        if (*word == "directory" || *word == "dir" || *word == "folder") && i + 1 < words.len() {
            let next_word = words[i + 1];
            return Some(next_word.trim_matches('"').trim_matches('\'').to_string());
        }
    }
    
    // Look for path-like patterns
    for word in words.iter() {
        if word.starts_with("./") || word.starts_with("../") || word.starts_with('/') {
            return Some(word.trim_matches('"').trim_matches('\'').to_string());
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_file_path() {
        assert_eq!(
            extract_file_path("Read the file config.toml"),
            Some("config.toml".to_string())
        );
        
        assert_eq!(
            extract_file_path("Check README.md for details"),
            Some("README.md".to_string())
        );
        
        assert_eq!(
            extract_file_path("No file here"),
            None
        );
    }

    #[test]
    fn test_extract_command() {
        assert_eq!(
            extract_command("Run echo 'hello world'"),
            Some("echo 'hello world'".to_string())
        );
        
        assert_eq!(
            extract_command("Execute ls -la"),
            Some("ls -la".to_string())
        );
        
        assert_eq!(
            extract_command("No command here"),
            None
        );
    }

    #[test]
    fn test_extract_directory_path() {
        assert_eq!(
            extract_directory_path("List directory ./src"),
            Some("./src".to_string())
        );
        
        assert_eq!(
            extract_directory_path("Check folder /tmp"),
            Some("/tmp".to_string())
        );
    }

    #[test]
    fn test_has_file_extension() {
        assert!(has_file_extension("test.rs"));
        assert!(has_file_extension("config.toml"));
        assert!(has_file_extension("README.md"));
        assert!(!has_file_extension("noextension"));
    }

    #[tokio::test]
    async fn test_read_file() {
        // Test reading Cargo.toml (should exist in project root)
        let result = read_file("Cargo.toml").await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("task-runner"));
    }

    #[tokio::test]
    async fn test_list_files() {
        // Test listing src directory
        let result = list_files("src").await;
        assert!(result.is_ok());
        let files = result.unwrap();
        // Check for lib.rs which should always exist
        assert!(files.contains("lib.rs") || files.contains("FILE: lib.rs"));
    }

    #[tokio::test]
    async fn test_run_command() {
        // Test simple echo command
        let result = run_command("echo 'test'").await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("test"));
    }
}

