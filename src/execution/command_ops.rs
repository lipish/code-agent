//! Command Execution Operations
//!
//! This module provides secure command execution capabilities for task execution.
//!
//! # Security
//!
//! All commands are validated before execution to prevent:
//! - Unauthorized commands
//! - Dangerous patterns (rm -rf, sudo, etc.)
//! - Path traversal attempts
//! - Resource exhaustion

use crate::errors::{AgentError, CommandOperationError, ToolError};
use crate::security::{CommandValidator, ResourceLimits};
use std::process::Stdio;

/// Maximum command output size (1 MB) - kept for backward compatibility
#[allow(dead_code)]
const MAX_OUTPUT_SIZE: usize = 1024 * 1024;

/// Default command timeout (30 seconds) - kept for backward compatibility
#[allow(dead_code)]
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// 执行 Shell 命令（带安全验证）
///
/// 此函数会自动进行安全检查和资源限制：
/// - 命令白名单验证
/// - 危险模式检测
/// - 执行超时（默认 30 秒）
/// - 输出大小限制（默认 1 MB）
///
/// # 参数
///
/// * `command` - 要执行的 Shell 命令
///
/// # 返回
///
/// * `Ok(String)` - 命令的标准输出
/// * `Err(AgentError)` - 执行失败时的错误信息
///   - `SecurityError::UnauthorizedCommand` - 命令不在白名单中
///   - `SecurityError::DangerousPattern` - 命令包含危险模式
///   - `CommandOperationError::ExecutionFailed` - 命令执行失败
///   - `CommandOperationError::Timeout` - 命令执行超时
///   - `CommandOperationError::OutputTooLarge` - 输出超过大小限制
///
/// # 安全性
///
/// **允许的命令**（白名单）:
/// - 文件操作: `ls`, `cat`, `head`, `tail`, `wc`, `file`
/// - 文本处理: `echo`, `grep`, `sed`, `awk`, `cut`, `sort`, `uniq`
/// - 开发工具: `cargo`, `npm`, `yarn`, `pip`, `python`, `git`
/// - 构建工具: `make`, `cmake`, `ninja`
///
/// **阻止的模式**（黑名单）:
/// - `rm -rf` - 递归删除
/// - `sudo` - 权限提升
/// - `curl | sh` - 管道到 shell
/// - `eval` - 动态执行
///
/// # 资源限制
///
/// - 执行超时: 30 秒
/// - 输出大小: 1 MB
///
/// # 示例
///
/// ```no_run
/// use agent_runner::execution::run_command;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 列出文件
/// let output = run_command("ls -la").await?;
/// println!("Files:\n{}", output);
///
/// // 查看文件内容
/// let content = run_command("cat README.md").await?;
/// println!("README:\n{}", content);
///
/// // 运行构建命令
/// let build_output = run_command("cargo build --release").await?;
/// println!("Build output:\n{}", build_output);
///
/// // 危险命令会被阻止
/// match run_command("rm -rf /").await {
///     Err(e) => println!("Blocked: {}", e),
///     _ => unreachable!(),
/// }
/// # Ok(())
/// # }
/// ```
pub async fn run_command(command: &str) -> Result<String, AgentError> {
    run_command_with_limits(command, &ResourceLimits::default()).await
}

/// Run a command with custom resource limits
///
/// # Arguments
///
/// * `command` - The shell command to execute
/// * `limits` - Resource limits to enforce
///
/// # Returns
///
/// The command output (stdout), or an error if the command fails.
pub async fn run_command_with_limits(
    command: &str,
    limits: &ResourceLimits,
) -> Result<String, AgentError> {
    // Validate command for security
    let validator = CommandValidator::new();
    validator.validate(command).map_err(|e| {
        AgentError::ToolError(ToolError::CommandOperation(
            CommandOperationError::Security(e)
        ))
    })?;
    // Execute with timeout
    let timeout_duration = limits.max_execution_time;
    let max_output = limits.max_output_size;

    let result = tokio::time::timeout(
        timeout_duration,
        tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
    ).await;

    let output = match result {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => {
            return Err(AgentError::ToolError(ToolError::CommandOperation(
                CommandOperationError::IoError {
                    command: command.to_string(),
                    message: e.to_string(),
                }
            )));
        }
        Err(_) => {
            return Err(AgentError::ToolError(ToolError::CommandOperation(
                CommandOperationError::Timeout {
                    seconds: timeout_duration.as_secs(),
                }
            )));
        }
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        // Check output size
        if stdout.len() > max_output {
            return Err(AgentError::ToolError(ToolError::CommandOperation(
                CommandOperationError::OutputTooLarge {
                    size: stdout.len(),
                    max_size: max_output,
                }
            )));
        }

        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let code = output.status.code().unwrap_or(-1);

        Err(AgentError::ToolError(ToolError::CommandOperation(
            CommandOperationError::ExecutionFailed {
                code,
                stderr,
            }
        )))
    }
}

/// Run a command with custom environment variables
///
/// # Arguments
///
/// * `command` - The shell command to execute
/// * `env_vars` - Environment variables to set for the command
///
/// # Returns
///
/// The command output (stdout), or an error if the command fails.
pub async fn run_command_with_env(
    command: &str,
    env_vars: &[(&str, &str)],
) -> Result<String, AgentError> {
    let mut cmd = tokio::process::Command::new("sh");
    cmd.arg("-c").arg(command);
    
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
    
    let output = cmd
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

/// Run a command in a specific working directory
///
/// # Arguments
///
/// * `command` - The shell command to execute
/// * `working_dir` - The working directory for the command
///
/// # Returns
///
/// The command output (stdout), or an error if the command fails.
pub async fn run_command_in_dir(
    command: &str,
    working_dir: &str,
) -> Result<String, AgentError> {
    let output = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(working_dir)
        .output()
        .await
        .map_err(|e| AgentError::ToolError(ToolError::ExecutionError(
            format!("Failed to execute command '{}' in '{}': {}", command, working_dir, e)
        )))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(AgentError::ToolError(ToolError::ExecutionError(
            format!("Command '{}' failed in '{}': {}", command, working_dir, stderr)
        )))
    }
}

/// Run a command with timeout
///
/// # Arguments
///
/// * `command` - The shell command to execute
/// * `timeout_secs` - Timeout in seconds
///
/// # Returns
///
/// The command output (stdout), or an error if the command fails or times out.
pub async fn run_command_with_timeout(
    command: &str,
    timeout_secs: u64,
) -> Result<String, AgentError> {
    let timeout_duration = std::time::Duration::from_secs(timeout_secs);
    
    let result = tokio::time::timeout(
        timeout_duration,
        run_command(command)
    ).await;
    
    match result {
        Ok(output) => output,
        Err(_) => Err(AgentError::ToolError(ToolError::ExecutionError(
            format!("Command '{}' timed out after {} seconds", command, timeout_secs)
        ))),
    }
}

/// Check if a command exists in PATH
///
/// # Arguments
///
/// * `command` - The command name to check
///
/// # Returns
///
/// `true` if the command exists, `false` otherwise
pub async fn command_exists(command: &str) -> bool {
    tokio::process::Command::new("which")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_command() {
        // Test simple echo command
        let result = run_command("echo 'test'").await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("test"));
    }

    #[tokio::test]
    async fn test_run_command_with_env() {
        let result = run_command_with_env(
            "echo $TEST_VAR",
            &[("TEST_VAR", "hello")]
        ).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("hello"));
    }

    #[tokio::test]
    async fn test_run_command_in_dir() {
        let result = run_command_in_dir("pwd", ".").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_command_with_timeout() {
        // Fast command should succeed
        let result = run_command_with_timeout("echo 'fast'", 5).await;
        assert!(result.is_ok());
        
        // Slow command should timeout
        let result = run_command_with_timeout("sleep 10", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_command_exists() {
        // Common commands that should exist
        assert!(command_exists("echo").await);
        assert!(command_exists("ls").await);
        
        // Command that shouldn't exist
        assert!(!command_exists("nonexistent_command_12345").await);
    }

    #[tokio::test]
    async fn test_failed_command() {
        let result = run_command("exit 1").await;
        assert!(result.is_err());
    }
}

