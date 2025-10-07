# Task Runner 代码风格指南

## 📝 概述

本文档定义了 Task Runner 项目的代码风格规范，确保代码库的一致性和可维护性。

## 🎯 命名规范

### 1. 遵循 Rust 官方命名惯例

| 类型 | 命名风格 | 示例 |
|------|---------|------|
| 模块 | `snake_case` | `parser`, `execution` |
| 函数/方法 | `snake_case` | `read_file`, `process_task` |
| 变量 | `snake_case` | `task_result`, `file_path` |
| 常量 | `SCREAMING_SNAKE_CASE` | `MAX_FILE_SIZE`, `ALLOWED_COMMANDS` |
| 类型/结构体 | `PascalCase` | `TaskAgent`, `ResourceLimits` |
| 枚举 | `PascalCase` | `TaskStatus`, `AgentError` |
| 枚举变体 | `PascalCase` | `TaskStatus::Completed` |
| Trait | `PascalCase` | `LanguageModel`, `Tool` |
| 生命周期 | `'lowercase` | `'a`, `'static` |

### 2. 语义化命名

**好的命名**:
```rust
pub async fn read_file(path: &str) -> Result<String, AgentError>
pub fn validate_command(cmd: &str) -> Result<(), SecurityError>
pub struct ResourceLimits { ... }
```

**避免的命名**:
```rust
pub async fn rf(p: &str) -> Result<String, AgentError>  // 太简短
pub fn check(c: &str) -> Result<(), SecurityError>      // 不明确
pub struct Limits { ... }                                // 不够具体
```

## 📚 文档注释规范

### 1. 公共 API 必须有文档

所有 `pub` 项目都应该有文档注释：

```rust
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
///
/// # 安全性
///
/// - 阻止路径遍历攻击（如 `../../../etc/passwd`）
/// - 阻止访问敏感目录（如 `/etc`, `/root`）
///
/// # 示例
///
/// ```no_run
/// use task_runner::execution::read_file;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let content = read_file("config.toml").await?;
/// println!("Config: {}", content);
/// # Ok(())
/// # }
/// ```
pub async fn read_file(path: &str) -> Result<String, AgentError> {
    // ...
}
```

### 2. 文档注释结构

推荐的文档注释结构：

1. **简短描述**（一句话）
2. **详细说明**（可选）
3. **参数说明** (`# 参数`)
4. **返回值说明** (`# 返回`)
5. **错误说明** (`# 错误`)
6. **安全性说明** (`# 安全性`) - 如果涉及安全
7. **示例** (`# 示例`)
8. **注意事项** (`# 注意`) - 如果有特殊注意事项

### 3. 中英文混合

- 主要描述使用中文
- 代码示例使用英文注释
- 技术术语保留英文（如 `async`, `await`, `Result`）

```rust
/// 执行 Shell 命令（带安全验证）
///
/// 此函数会自动进行安全检查和资源限制。
///
/// # Examples
///
/// ```rust
/// // Execute a safe command
/// let output = run_command("ls -la").await?;
/// ```
```

## 🔧 代码组织

### 1. 导入顺序

```rust
// 1. 标准库
use std::sync::Arc;
use std::time::Duration;

// 2. 外部 crate
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

// 3. 本地模块
use crate::errors::AgentError;
use crate::models::LanguageModel;
```

### 2. 模块结构

```rust
//! 模块级文档注释
//!
//! 描述模块的用途和主要功能

// 导入
use ...;

// 常量
const MAX_SIZE: usize = 1024;

// 类型定义
pub struct MyStruct { ... }

// 实现
impl MyStruct { ... }

// 测试
#[cfg(test)]
mod tests { ... }
```

### 3. 函数顺序

在 `impl` 块中：

1. 构造函数 (`new`, `with_*`, `from_*`)
2. 公共方法（按重要性排序）
3. 私有方法
4. Trait 实现

## 🎨 代码格式

### 1. 使用 rustfmt

```bash
cargo fmt
```

### 2. 行长度

- 最大行长度：100 字符
- 注释行长度：80 字符

### 3. 缩进

- 使用 4 个空格缩进
- 不使用 Tab

## ✅ 代码质量

### 1. 使用 clippy

```bash
cargo clippy -- -D warnings
```

### 2. 避免的模式

**不推荐**:
```rust
// 过度使用 unwrap
let value = option.unwrap();

// 忽略错误
let _ = dangerous_operation();

// 魔法数字
if size > 10485760 { ... }
```

**推荐**:
```rust
// 使用 ? 或 match
let value = option.ok_or(Error::NotFound)?;

// 处理错误
dangerous_operation().map_err(|e| log::error!("Failed: {}", e))?;

// 使用常量
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
if size > MAX_FILE_SIZE { ... }
```

### 3. 错误处理

```rust
// 使用具体的错误类型
pub enum FileOperationError {
    #[error("File not found: {path}")]
    NotFound { path: String },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },
}

// 提供上下文
.map_err(|e| FileOperationError::IoError {
    path: path.to_string(),
    message: e.to_string(),
})?
```

## 🧪 测试规范

### 1. 测试命名

```rust
#[test]
fn test_function_name_expected_behavior() {
    // 测试代码
}

#[tokio::test]
async fn test_async_function_success_case() {
    // 异步测试
}
```

### 2. 测试组织

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 成功案例
    #[test]
    fn test_read_file_success() { ... }

    // 错误案例
    #[test]
    fn test_read_file_not_found() { ... }

    // 边界案例
    #[test]
    fn test_read_file_empty() { ... }
}
```

## 📦 模块设计

### 1. 单一职责

每个模块应该有明确的单一职责：

- `agent/` - 任务代理
- `execution/` - 执行操作
- `security/` - 安全检查
- `understanding/` - 任务理解

### 2. 最小化公共 API

只暴露必要的公共接口：

```rust
// 公共 API
pub struct TaskAgent { ... }
pub fn process_task(...) -> Result<...> { ... }

// 内部实现
fn internal_helper(...) -> ... { ... }
```

## 🔍 代码审查清单

在提交代码前检查：

- [ ] 所有公共 API 都有文档注释
- [ ] 文档注释包含示例
- [ ] 运行 `cargo fmt`
- [ ] 运行 `cargo clippy` 无警告
- [ ] 所有测试通过 `cargo test`
- [ ] 文档生成成功 `cargo doc`
- [ ] 命名符合 Rust 惯例
- [ ] 错误处理完善
- [ ] 没有 `unwrap()` 或 `expect()` 在生产代码中

## 📚 参考资源

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- [Rust Documentation Guidelines](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)

## 🎉 总结

遵循这些规范可以：

- ✅ 提高代码可读性
- ✅ 减少维护成本
- ✅ 改善团队协作
- ✅ 提升代码质量
- ✅ 生成高质量文档

