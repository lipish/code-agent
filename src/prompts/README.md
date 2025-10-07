# Prompts Module

提示词工程系统，提供灵活的分层提示词模板。

## 📁 文件结构

```
src/prompts/
├── mod.rs          # 主模块 - 核心数据结构和 API
├── defaults.rs     # 默认提示词 - 硬编码的默认值
└── README.md       # 本文档
```

## 🎯 快速修改提示词

### 方式 1: 修改硬编码默认值 (推荐用于开发)

编辑 `src/prompts/defaults.rs`：

```rust
// 修改系统角色
pub const SYSTEM_ROLE: &str = "\
Your custom system role here...
";

// 修改输出字段
pub const REQUIRED_FIELDS: &[&str] = &[
    "YOUR_FIELD_1",
    "YOUR_FIELD_2",
];

// 修改约束条件
pub const CORE_PRINCIPLES: &[&str] = &[
    "Your principle 1",
    "Your principle 2",
];
```

**优点**:
- ✅ 简单直接
- ✅ 类型安全
- ✅ 编译时检查
- ✅ 无需外部文件

**缺点**:
- ❌ 需要重新编译
- ❌ 不能运行时修改

---

### 方式 2: 使用 YAML 文件 (推荐用于生产)

创建或修改 `prompts/custom-template.yaml`：

```yaml
global:
  system_role: |
    Your custom system role here...
  
  output_format:
    format_type: "structured_text"
    required_fields:
      - "YOUR_FIELD_1"
      - "YOUR_FIELD_2"
    field_descriptions:
      YOUR_FIELD_1: "Description 1"
      YOUR_FIELD_2: "Description 2"
  
  constraints:
    - "Your constraint 1"
    - "Your constraint 2"

project:
  tech_stack:
    - "Your tech stack"
  conventions:
    - "Your conventions"

scenarios:
  your_scenario:
    name: "Your Scenario"
    description: "Scenario description"
    instructions:
      - "Instruction 1"
      - "Instruction 2"
```

然后在代码中加载：

```rust
use task_runner::prompts::PromptTemplate;

// 加载自定义模板
let template = PromptTemplate::from_file("prompts/custom-template.yaml")?;
```

**优点**:
- ✅ 无需重新编译
- ✅ 运行时可修改
- ✅ 易于版本控制
- ✅ 支持多个模板

**缺点**:
- ❌ 需要外部文件
- ❌ 运行时错误可能

---

## 📝 默认提示词说明

### 系统角色 (SYSTEM_ROLE)

定义 AI 助手的个性和行为：

```rust
pub const SYSTEM_ROLE: &str = "\
You are a precise, safe, and helpful coding assistant with full autonomy. \
You analyze tasks, plan solutions, and execute them efficiently.

Your personality is concise, direct, and friendly. You communicate efficiently, \
keeping the user clearly informed without unnecessary detail. You prioritize \
actionable guidance, clearly stating assumptions and next steps.";
```

**关键特征**:
- Precise (精确)
- Safe (安全)
- Helpful (有帮助)
- Concise (简洁)
- Direct (直接)
- Friendly (友好)

---

### 输出格式 (OUTPUT_FORMAT)

定义 AI 输出的结构：

```rust
pub const REQUIRED_FIELDS: &[&str] = &[
    "UNDERSTANDING",  // 任务理解 (1-2 句话)
    "APPROACH",       // 解决方法 (2-3 要点)
    "PLAN",          // 执行计划 (多步骤任务)
    "EXECUTION",     // 具体操作 (文件路径和命令)
];
```

**字段说明**:
- `UNDERSTANDING` - 简要理解任务
- `APPROACH` - 高层次解决方案
- `PLAN` - 分步执行计划
- `EXECUTION` - 具体可执行操作

---

### 约束条件 (CONSTRAINTS)

定义 AI 行为的规则：

#### 核心原则 (4 条)
```rust
pub const CORE_PRINCIPLES: &[&str] = &[
    "Be concise and direct - avoid verbose explanations",
    "Fix problems at root cause, not surface-level patches",
    "Keep changes minimal and focused on the task",
    "Avoid unneeded complexity in solutions",
];
```

#### 代码质量 (4 条)
```rust
pub const CODE_QUALITY: &[&str] = &[
    "Follow existing codebase style and conventions",
    "Consider edge cases and error handling",
    "Update documentation as necessary",
    "Do not add inline comments unless requested",
];
```

#### 安全性 (4 条)
```rust
pub const SAFETY: &[&str] = &[
    "Never add copyright/license headers unless requested",
    "Do not fix unrelated bugs or broken tests",
    "Validate work with tests when available",
    "Use git log/blame for additional context if needed",
];
```

---

## 🔧 常见修改场景

### 场景 1: 修改系统角色语气

如果你想要更正式的语气：

```rust
// defaults.rs
pub const SYSTEM_ROLE: &str = "\
You are a professional software engineering assistant. \
You provide precise, well-documented solutions following industry best practices.";
```

### 场景 2: 添加新的输出字段

```rust
// defaults.rs
pub const REQUIRED_FIELDS: &[&str] = &[
    "UNDERSTANDING",
    "APPROACH",
    "PLAN",
    "EXECUTION",
    "TESTING",      // 新增：测试策略
    "VALIDATION",   // 新增：验证方法
];

pub const FIELD_DESCRIPTIONS: &[(&str, &str)] = &[
    // ... 现有字段 ...
    ("TESTING", "Testing strategy and test cases"),
    ("VALIDATION", "How to validate the solution"),
];
```

### 场景 3: 添加项目特定约束

```rust
// defaults.rs
pub const CODE_QUALITY: &[&str] = &[
    "Follow existing codebase style and conventions",
    "Consider edge cases and error handling",
    "Update documentation as necessary",
    "Do not add inline comments unless requested",
    "Always use async/await for I/O operations",  // 新增
    "Prefer composition over inheritance",         // 新增
];
```

### 场景 4: 为特定语言定制

创建 `prompts/rust-specific.yaml`：

```yaml
global:
  system_role: |
    You are a Rust programming expert with deep knowledge of:
    - Ownership and borrowing
    - Async programming with Tokio
    - Error handling with Result
    - Zero-cost abstractions
  
  constraints:
    - "Use Result<T, E> for error handling, never panic"
    - "Prefer &str over String when possible"
    - "Use ? operator for error propagation"
    - "Add #[derive(Debug)] to all structs"
    - "Use #[cfg(test)] for test modules"
```

---

## 🧪 测试

运行测试确保修改正确：

```bash
# 运行所有测试
cargo test --all-features

# 只运行 prompts 模块测试
cargo test prompts

# 运行 defaults 模块测试
cargo test prompts::defaults
```

---

## 📚 相关文档

- `doc/PROMPT_ENGINEERING.md` - 提示词工程完整文档
- `doc/CODEX_PROMPT_ANALYSIS.md` - Codex CLI 提示词分析
- `prompts/optimized-template.yaml` - 优化的 YAML 模板示例

---

## 💡 最佳实践

### 1. 保持简洁
- ✅ 简短明确的约束
- ❌ 冗长的解释

### 2. 具体而非抽象
- ✅ "Use Result<T, E> for error handling"
- ❌ "Handle errors properly"

### 3. 可操作的指导
- ✅ "Fix problems at root cause"
- ❌ "Be thorough"

### 4. 分类组织
- 核心原则
- 代码质量
- 安全性

### 5. 版本控制
- 修改前备份
- 记录修改原因
- 测试验证

---

## 🔄 修改流程

1. **编辑** `src/prompts/defaults.rs`
2. **编译** `cargo build`
3. **测试** `cargo test prompts`
4. **验证** 运行实际任务测试效果
5. **提交** 如果满意，提交更改

---

## ⚠️ 注意事项

1. **编译时间** - 修改 defaults.rs 需要重新编译
2. **向后兼容** - 修改字段名可能影响现有代码
3. **测试覆盖** - 确保测试通过
4. **文档同步** - 更新相关文档

---

## 🎯 总结

- **快速修改**: 编辑 `defaults.rs`
- **灵活配置**: 使用 YAML 文件
- **类型安全**: Rust 编译时检查
- **易于维护**: 清晰的模块结构

