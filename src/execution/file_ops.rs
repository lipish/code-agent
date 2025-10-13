//! File Operations
//!
//! This module provides secure file system operations for task execution.
//!
//! # Security
//!
//! All file operations include:
//! - Path validation (no path traversal)
//! - Size limits
//! - Permission checks

use crate::errors::{AgentError, FileOperationError, ToolError};
use crate::security::{PathValidator, ResourceLimits};
use std::io::ErrorKind;

/// Maximum file size for reading (10 MB) - kept for backward compatibility
#[allow(dead_code)]
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// 读取指定路径的文件内容
///
/// 此函数会自动进行安全检查：
/// - 路径验证（防止路径遍历）
/// - 文件大小限制（默认 10 MB）
/// - 权限检查
///
/// # 参数
///
/// * `path` - 文件路径，支持相对路径和绝对路径
///
/// # 返回
///
/// * `Ok(String)` - 文件内容
/// * `Err(AgentError)` - 读取失败时的错误信息
///   - `FileOperationError::NotFound` - 文件不存在
///   - `FileOperationError::PermissionDenied` - 权限不足
///   - `FileOperationError::FileTooLarge` - 文件超过大小限制
///   - `FileOperationError::InvalidPath` - 路径包含危险模式
///
/// # 安全性
///
/// - 阻止路径遍历攻击（如 `../../../etc/passwd`）
/// - 阻止访问敏感目录（如 `/etc`, `/root`）
/// - 限制文件大小防止内存耗尽
///
/// # 示例
///
/// ```no_run
/// use agent_runner::execution::read_file;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 读取配置文件
/// let content = read_file("config.toml").await?;
/// println!("Config: {}", content);
///
/// // 读取源代码
/// let code = read_file("src/main.rs").await?;
/// println!("Lines: {}", code.lines().count());
/// # Ok(())
/// # }
/// ```
pub async fn read_file(path: &str) -> Result<String, AgentError> {
    read_file_with_limits(path, &ResourceLimits::default()).await
}

/// Read a file with custom resource limits
///
/// # Arguments
///
/// * `path` - The path to the file to read
/// * `limits` - Resource limits to enforce
///
/// # Returns
///
/// The file contents as a string, or an error if the file cannot be read.
pub async fn read_file_with_limits(
    path: &str,
    limits: &ResourceLimits,
) -> Result<String, AgentError> {
    // Validate path for security
    PathValidator::validate(path).map_err(|_| {
        AgentError::ToolError(ToolError::FileOperation(
            FileOperationError::InvalidPath {
                path: path.to_string(),
            }
        ))
    })?;

    // Check file size first
    let metadata = tokio::fs::metadata(path)
        .await
        .map_err(|e| {
            let error = match e.kind() {
                ErrorKind::NotFound => FileOperationError::NotFound {
                    path: path.to_string(),
                },
                ErrorKind::PermissionDenied => FileOperationError::PermissionDenied {
                    path: path.to_string(),
                },
                _ => FileOperationError::IoError {
                    path: path.to_string(),
                    message: e.to_string(),
                },
            };
            AgentError::ToolError(ToolError::FileOperation(error))
        })?;

    // Check file size against limits
    let size = metadata.len();
    if size > limits.max_file_size {
        return Err(AgentError::ToolError(ToolError::FileOperation(
            FileOperationError::FileTooLarge {
                size,
                max_size: limits.max_file_size,
            },
        )));
    }

    // Read file content
    let content = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| {
            AgentError::ToolError(ToolError::FileOperation(FileOperationError::IoError {
                path: path.to_string(),
                message: e.to_string(),
            }))
        })?;

    Ok(content)
}

/// 将内容写入指定路径的文件
///
/// 此函数会自动进行安全检查：
/// - 路径验证（防止路径遍历）
/// - 权限检查
///
/// # 参数
///
/// * `path` - 文件路径，如果文件不存在会创建
/// * `content` - 要写入的内容
///
/// # 返回
///
/// * `Ok(())` - 写入成功
/// * `Err(AgentError)` - 写入失败时的错误信息
///   - `FileOperationError::PermissionDenied` - 权限不足
///   - `FileOperationError::InvalidPath` - 路径包含危险模式
///   - `FileOperationError::IoError` - 其他 I/O 错误
///
/// # 安全性
///
/// - 阻止路径遍历攻击
/// - 阻止写入敏感目录
///
/// # 示例
///
/// ```no_run
/// use agent_runner::execution::write_file;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 写入文本文件
/// write_file("output.txt", "Hello, world!").await?;
///
/// // 写入 JSON 数据
/// let json = r#"{"name": "agent-runner", "version": "0.2.1"}"#;
/// write_file("data.json", json).await?;
/// # Ok(())
/// # }
/// ```
pub async fn write_file(path: &str, content: &str) -> Result<(), AgentError> {
    // Validate path for security
    PathValidator::validate(path).map_err(|_| {
        AgentError::ToolError(ToolError::FileOperation(
            FileOperationError::InvalidPath {
                path: path.to_string(),
            }
        ))
    })?;

    tokio::fs::write(path, content)
        .await
        .map_err(|e| {
            let error = match e.kind() {
                ErrorKind::PermissionDenied => FileOperationError::PermissionDenied {
                    path: path.to_string(),
                },
                ErrorKind::AlreadyExists => FileOperationError::AlreadyExists {
                    path: path.to_string(),
                },
                _ => FileOperationError::IoError {
                    path: path.to_string(),
                    message: e.to_string(),
                },
            };
            AgentError::ToolError(ToolError::FileOperation(error))
        })?;
    Ok(())
}

/// 列出目录中的所有文件和子目录
///
/// 返回格式化的文件列表，每行一个条目，格式为 "类型: 名称"。
///
/// # 参数
///
/// * `path` - 目录路径
///
/// # 返回
///
/// * `Ok(String)` - 格式化的文件列表，按字母顺序排序
/// * `Err(AgentError)` - 列出失败时的错误信息
///   - `FileOperationError::DirectoryNotFound` - 目录不存在
///   - `FileOperationError::PermissionDenied` - 权限不足
///   - `FileOperationError::IoError` - 其他 I/O 错误
///
/// # 输出格式
///
/// ```text
/// DIR: subdir1
/// DIR: subdir2
/// FILE: file1.txt
/// FILE: file2.rs
/// ```
///
/// # 示例
///
/// ```no_run
/// use agent_runner::execution::list_files;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 列出当前目录
/// let files = list_files(".").await?;
/// println!("Current directory:\n{}", files);
///
/// // 列出源代码目录
/// let src_files = list_files("./src").await?;
/// for line in src_files.lines() {
///     if line.starts_with("FILE:") && line.ends_with(".rs") {
///         println!("Rust file: {}", line);
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub async fn list_files(path: &str) -> Result<String, AgentError> {
    let mut entries = tokio::fs::read_dir(path)
        .await
        .map_err(|e| {
            let error = match e.kind() {
                ErrorKind::NotFound => FileOperationError::DirectoryNotFound {
                    path: path.to_string(),
                },
                ErrorKind::PermissionDenied => FileOperationError::PermissionDenied {
                    path: path.to_string(),
                },
                _ => FileOperationError::IoError {
                    path: path.to_string(),
                    message: e.to_string(),
                },
            };
            AgentError::ToolError(ToolError::FileOperation(error))
        })?;

    let mut files = Vec::new();
    while let Some(entry) = entries.next_entry().await
        .map_err(|e| AgentError::ToolError(ToolError::FileOperation(
            FileOperationError::IoError {
                path: path.to_string(),
                message: e.to_string(),
            }
        )))? {
        let name = entry.file_name().to_string_lossy().to_string();
        let metadata = entry.metadata().await
            .map_err(|e| AgentError::ToolError(ToolError::FileOperation(
                FileOperationError::IoError {
                    path: path.to_string(),
                    message: e.to_string(),
                }
            )))?;
        let file_type = if metadata.is_dir() { "DIR" } else { "FILE" };
        files.push(format!("{}: {}", file_type, name));
    }

    files.sort();
    Ok(files.join("\n"))
}

/// Check if a file exists
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// `true` if the file exists, `false` otherwise
pub async fn file_exists(path: &str) -> bool {
    tokio::fs::metadata(path).await.is_ok()
}

/// Get file metadata
///
/// # Arguments
///
/// * `path` - The path to the file
///
/// # Returns
///
/// File size in bytes, or an error
pub async fn file_size(path: &str) -> Result<u64, AgentError> {
    let metadata = tokio::fs::metadata(path)
        .await
        .map_err(|e| AgentError::ToolError(ToolError::ExecutionError(
            format!("Failed to get metadata for '{}': {}", path, e)
        )))?;
    Ok(metadata.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_file() {
        // Test reading Cargo.toml (should exist in project root)
        let result = read_file("Cargo.toml").await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("agent-runner"));
    }

    #[tokio::test]
    async fn test_list_files() {
        // Test listing src directory
        let result = list_files("src").await;
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.contains("lib.rs") || files.contains("FILE: lib.rs"));
    }

    #[tokio::test]
    async fn test_file_exists() {
        assert!(file_exists("Cargo.toml").await);
        assert!(!file_exists("nonexistent_file_12345.txt").await);
    }

    #[tokio::test]
    async fn test_file_size() {
        let result = file_size("Cargo.toml").await;
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[tokio::test]
    async fn test_write_and_read_file() {
        let test_file = "test_temp_file.txt";
        let test_content = "Hello, test!";
        
        // Write
        let write_result = write_file(test_file, test_content).await;
        assert!(write_result.is_ok());
        
        // Read
        let read_result = read_file(test_file).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), test_content);
        
        // Cleanup
        let _ = tokio::fs::remove_file(test_file).await;
    }
}

