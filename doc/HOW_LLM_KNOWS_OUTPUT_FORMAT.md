# LLM 如何知道输出格式

## 概述

LLM 通过 **System Prompt** (系统提示词) 了解应该使用什么格式输出。本文档详细解释这个机制。

## 🔄 完整流程

```
用户请求 → PromptBuilder → 构建完整提示词 → 发送给 LLM → LLM 按格式输出
```

---

## 📝 Step 1: 用户发起请求

```rust
// 用户代码
let engine = PlanningEngine::new(model);
let result = engine.analyze_task("重构 agent.rs 的错误处理").await?;
```

---

## 🏗️ Step 2: PromptBuilder 构建提示词

`PromptBuilder::build()` 方法会构建一个完整的提示词，包含：

### 构建的提示词结构

```markdown
# System Role
You are a precise, safe, and helpful coding assistant...

# Project Context
**Tech Stack**: Rust, Tokio, Serde
**Conventions**:
- Use Result<T, E> for error handling
- Prefer composition over inheritance

# Task Type: Code Refactoring
Improve code structure, eliminate duplication...

**Instructions**:
1. Identify the root cause of code smell
2. Preserve existing behavior
3. Follow DRY, SRP, KISS principles

---

# User Request
```
重构 agent.rs 的错误处理
```

# Output Format
Format: structured_text

**Required Fields**:
- `UNDERSTANDING`: Brief understanding of the task (1-2 sentences)
- `APPROACH`: High-level approach to solve it (2-3 key points)
- `PLAN`: Step-by-step plan with clear phases (if multi-step task)
- `EXECUTION`: Concrete actions to take with file paths and commands

# Constraints
- Be concise and direct - avoid verbose explanations
- Fix problems at root cause, not surface-level patches
- Keep changes minimal and focused on the task
- ...

---

Please analyze the request and provide your response following the specified format.
```

---

## 🎯 Step 3: LLM 看到的完整提示词

让我们看一个实际例子：

### 实际发送给 LLM 的提示词

```markdown
# System Role
You are a precise, safe, and helpful coding assistant with full autonomy. 
You analyze tasks, plan solutions, and execute them efficiently.

Your personality is concise, direct, and friendly. You communicate efficiently, 
keeping the user clearly informed without unnecessary detail. You prioritize 
actionable guidance, clearly stating assumptions and next steps.

# Project Context
**Tech Stack**: Rust, Tokio, Serde, Cargo
**Conventions**:
- Use snake_case for functions and variables
- Use PascalCase for types and traits
- Prefer Result<T, E> over panicking
- Use ? operator for error propagation

**Context**: Task Runner is an AI-native code assistant system focused on:
- Task understanding and planning
- Execution with file operations and commands
- Security validation

---

# User Request
```
重构 agent.rs 的错误处理
```

# Output Format
Format: structured_text

**Required Fields**:
- `UNDERSTANDING`: Brief understanding of the task (1-2 sentences)
- `APPROACH`: High-level approach to solve it (2-3 key points)
- `PLAN`: Step-by-step plan with clear phases (if multi-step task)
- `EXECUTION`: Concrete actions to take with file paths and commands

# Constraints
- Be concise and direct - avoid verbose explanations
- Fix problems at root cause, not surface-level patches
- Keep changes minimal and focused on the task
- Avoid unneeded complexity in solutions
- Follow existing codebase style and conventions
- Consider edge cases and error handling
- Update documentation as necessary
- Do not add inline comments unless requested
- Never add copyright/license headers unless requested
- Do not fix unrelated bugs or broken tests
- Validate work with tests when available
- Use git log/blame for additional context if needed

---

Please analyze the request and provide your response following the specified format.
```

---

## 🤖 Step 4: LLM 理解并输出

LLM 读取提示词后，会：

1. **理解系统角色** - "我是一个精确、安全、有帮助的编码助手"
2. **理解项目上下文** - "这是一个 Rust 项目，使用 Tokio..."
3. **理解用户请求** - "用户想重构错误处理"
4. **理解输出格式** - "我需要输出 4 个字段：UNDERSTANDING, APPROACH, PLAN, EXECUTION"
5. **理解约束条件** - "我要简洁、修复根因、最小改动..."

### LLM 的输出

```
UNDERSTANDING:
需要改进 agent.rs 中的错误处理机制，使用 Result 类型替代 unwrap()，
并添加更详细的错误信息。

APPROACH:
1. 识别所有使用 unwrap() 和 expect() 的地方
2. 创建自定义错误类型 AgentError
3. 使用 ? 操作符传播错误，提供清晰的错误上下文

PLAN:
Phase 1: 准备工作
- 创建 src/errors.rs 定义 AgentError
- 添加 thiserror 依赖到 Cargo.toml

Phase 2: 重构核心模块
- 修改 agent.rs 中的函数签名返回 Result
- 替换所有 unwrap() 为 ? 操作符
- 添加错误上下文信息

Phase 3: 更新调用方
- 修改 main.rs 处理新的错误类型
- 更新相关测试用例

Phase 4: 验证
- 运行 cargo build 确保编译通过
- 运行 cargo test 确保测试通过

EXECUTION:
1. 创建错误类型定义
   文件: src/errors.rs
   代码:
   ```rust
   use thiserror::Error;
   
   #[derive(Error, Debug)]
   pub enum AgentError {
       #[error("Task execution failed: {0}")]
       ExecutionError(String),
       
       #[error("IO error: {0}")]
       IoError(#[from] std::io::Error),
   }
   ```

2. 添加依赖
   命令: cargo add thiserror

3. 修改 agent.rs
   文件: src/agent.rs:42
   修改前: fn execute_task(&self, task: Task) -> ()
   修改后: fn execute_task(&self, task: Task) -> Result<(), AgentError>

4. 验证
   命令: 
   cargo build --all-features
   cargo test --all-features
```

---

## 🔍 关键代码解析

### 1. 构建输出格式部分

```rust
// src/prompts/mod.rs:176-190

// 5. Output format requirements
prompt.push_str("# Output Format\n");
prompt.push_str(&format!("Format: {}\n\n", self.template.global.output_format.format_type));

if !self.template.global.output_format.required_fields.is_empty() {
    prompt.push_str("**Required Fields**:\n");
    for field in &self.template.global.output_format.required_fields {
        if let Some(desc) = self.template.global.output_format.field_descriptions.get(field) {
            // 关键：告诉 LLM 每个字段的含义
            prompt.push_str(&format!("- `{}`: {}\n", field, desc));
        } else {
            prompt.push_str(&format!("- `{}`\n", field));
        }
    }
    prompt.push('\n');
}
```

### 2. 字段定义

```rust
// src/prompts/defaults.rs

pub const REQUIRED_FIELDS: &[&str] = &[
    "UNDERSTANDING",
    "APPROACH",
    "PLAN",
    "EXECUTION",
];

pub const FIELD_DESCRIPTIONS: &[(&str, &str)] = &[
    ("UNDERSTANDING", "Brief understanding of the task (1-2 sentences)"),
    ("APPROACH", "High-level approach to solve it (2-3 key points)"),
    ("PLAN", "Step-by-step plan with clear phases (if multi-step task)"),
    ("EXECUTION", "Concrete actions to take with file paths and commands"),
];
```

这些定义会被转换成提示词中的：

```markdown
**Required Fields**:
- `UNDERSTANDING`: Brief understanding of the task (1-2 sentences)
- `APPROACH`: High-level approach to solve it (2-3 key points)
- `PLAN`: Step-by-step plan with clear phases (if multi-step task)
- `EXECUTION`: Concrete actions to take with file paths and commands
```

---

## 💡 为什么 LLM 能理解

### 1. 训练数据中的模式

LLM 在训练时见过大量的结构化输出：
- Markdown 格式
- 字段标签（如 `UNDERSTANDING:`, `APPROACH:`）
- 分步骤的计划
- 代码块

### 2. 明确的指令

提示词中明确告诉 LLM：
```markdown
# Output Format
Format: structured_text

**Required Fields**:
- `UNDERSTANDING`: Brief understanding...
- `APPROACH`: High-level approach...
```

### 3. 约束条件

约束条件进一步指导 LLM 的行为：
```markdown
# Constraints
- Be concise and direct
- Fix problems at root cause
- Keep changes minimal
```

---

## 🎨 自定义输出格式

### 方式 1: 修改默认字段

编辑 `src/prompts/defaults.rs`:

```rust
pub const REQUIRED_FIELDS: &[&str] = &[
    "UNDERSTANDING",
    "APPROACH",
    "PLAN",
    "EXECUTION",
    "TESTING",      // 新增
    "VALIDATION",   // 新增
];

pub const FIELD_DESCRIPTIONS: &[(&str, &str)] = &[
    ("UNDERSTANDING", "Brief understanding of the task (1-2 sentences)"),
    ("APPROACH", "High-level approach to solve it (2-3 key points)"),
    ("PLAN", "Step-by-step plan with clear phases (if multi-step task)"),
    ("EXECUTION", "Concrete actions to take with file paths and commands"),
    ("TESTING", "Test strategy and test cases"),
    ("VALIDATION", "How to validate the solution"),
];
```

LLM 会自动输出新的字段：

```
UNDERSTANDING: ...
APPROACH: ...
PLAN: ...
EXECUTION: ...
TESTING: 
- 添加单元测试验证错误处理
- 测试边界情况和错误路径
VALIDATION:
- 运行 cargo test 确保所有测试通过
- 手动测试错误场景
```

### 方式 2: 使用 YAML 配置

```yaml
# prompts/custom-format.yaml
global:
  output_format:
    format_type: "json"  # 改为 JSON 格式
    required_fields:
      - "task_analysis"
      - "solution"
      - "implementation"
    field_descriptions:
      task_analysis: "Analyze what needs to be done"
      solution: "Proposed solution approach"
      implementation: "Step-by-step implementation"
```

---

## 🔬 实验：不同的输出格式

### 格式 1: 简洁版

```rust
pub const REQUIRED_FIELDS: &[&str] = &[
    "WHAT",   // 做什么
    "HOW",    // 怎么做
    "CODE",   // 代码
];
```

LLM 输出：
```
WHAT: 重构错误处理
HOW: 使用 Result 和 thiserror
CODE: [具体代码]
```

### 格式 2: 详细版

```rust
pub const REQUIRED_FIELDS: &[&str] = &[
    "ANALYSIS",      // 分析
    "DESIGN",        // 设计
    "IMPLEMENTATION",// 实现
    "TESTING",       // 测试
    "DOCUMENTATION", // 文档
];
```

LLM 输出：
```
ANALYSIS: [详细分析]
DESIGN: [设计方案]
IMPLEMENTATION: [实现步骤]
TESTING: [测试策略]
DOCUMENTATION: [文档更新]
```

---

## 📊 输出格式的影响

| 字段数量 | 输出长度 | 详细程度 | 适用场景 |
|---------|---------|---------|---------|
| 2-3 个 | 短 | 简洁 | 简单任务 |
| 4-5 个 | 中等 | 平衡 | 一般任务 ✅ |
| 6+ 个 | 长 | 详细 | 复杂任务 |

---

## 🎯 总结

### LLM 知道输出格式的原因

1. **明确的指令** - 提示词中清楚说明了格式
2. **字段描述** - 每个字段都有详细说明
3. **训练模式** - LLM 训练时见过类似格式
4. **约束条件** - 进一步指导输出行为

### 关键机制

```
字段定义 (defaults.rs)
    ↓
PromptBuilder 构建
    ↓
完整提示词
    ↓
发送给 LLM
    ↓
LLM 按格式输出
```

### 修改输出格式

- ✅ 编辑 `src/prompts/defaults.rs`
- ✅ 修改 `REQUIRED_FIELDS` 和 `FIELD_DESCRIPTIONS`
- ✅ 重新编译
- ✅ LLM 自动使用新格式

就这么简单！🚀

