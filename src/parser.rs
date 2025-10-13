//! Text Parsing Utilities
//!
//! This module provides functions to extract structured information from natural language text.
//!
//! # Features
//!
//! - Extract file paths from text
//! - Extract shell commands from text
//! - Extract directory paths from text
//!
//! # Examples
//!
//! ```
//! use agent_runner::parser::extract_file_path;
//!
//! let text = "Read the file config.toml";
//! assert_eq!(extract_file_path(text), Some("config.toml".to_string()));
//! ```

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
/// use agent_runner::parser::extract_file_path;
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
/// use agent_runner::parser::extract_command;
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

// Note: File and command operations have been moved to:
// - src/execution/file_ops.rs for file operations
// - src/execution/command_ops.rs for command execution
// This module now focuses solely on text parsing.

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

    // Note: Tests for file and command operations have been moved to:
    // - src/execution/file_ops.rs
    // - src/execution/command_ops.rs
}

