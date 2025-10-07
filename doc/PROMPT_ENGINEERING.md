# Prompt Engineering System

## 概述

Task Runner 实现了一个灵活、分层的提示词工程系统，灵感来自 OpenAI Codex 和 Roo-Code 的最佳实践。该系统支持：

- **分层结构**: 全局模板、项目级规则、场景特定指令
- **可配置**: 通过 YAML 文件自定义提示词
- **可扩展**: 轻松添加新的场景和规则
- **类型安全**: 使用 Rust 类型系统保证正确性
- **外置提示词**: 支持从文件加载自定义提示词模板

## 架构设计

### 三层结构

```
┌─────────────────────────────────────────┐
│         Global Template                 │
│  (系统角色、输出格式、通用约束)          │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│         Project Rules                   │
│  (技术栈、编码规范、项目上下文)          │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│      Scenario-Specific Prompts          │
│  (针对不同任务类型的专门指导)            │
└─────────────────────────────────────────┘
```

### 核心组件

**PromptTemplate**: 完整的提示词模板
- `global`: 全局模板（系统角色、输出格式、约束）
- `project`: 项目级规则（技术栈、规范、上下文）
- `scenarios`: 场景特定提示词（代码生成、重构、调试等）

**PromptBuilder**: 流式 API 构建器
- 支持链式调用
- 动态注入上下文
- 自动推断任务类型

**PlanningEngine**: 集成提示词系统
- 使用模板生成提示词
- 自动任务类型推断
- 支持自定义模板加载

## 使用指南

### 1. 使用默认模板

```rust
use task_runner::planning::PlanningEngine;
use task_runner::models::LlmModel;
use task_runner::config::ModelConfig;
use std::sync::Arc;

// 创建模型
let model = Arc::new(LlmModel::from_config(config.model)?);

// 创建规划引擎（使用默认模板）
let engine = PlanningEngine::new(model);

// 分析任务
let plan = engine.analyze_task("创建一个配置加载器").await?;
```

### 2. 使用自定义模板

```rust
use task_runner::prompts::PromptTemplate;

// 从文件加载模板
let template = PromptTemplate::from_file("prompts/rust-project.yaml")?;

// 创建引擎
let engine = PlanningEngine::with_template(model, template);

// 分析任务
let plan = engine.analyze_task("重构错误处理").await?;
```

### 3. 指定任务类型

```rust
// 显式指定任务类型
let plan = engine
    .analyze_task_with_type("优化字符串拼接", Some("optimization"))
    .await?;
```

### 4. 使用 PromptBuilder

```rust
use task_runner::prompts::{PromptBuilder, PromptTemplate};

let template = PromptTemplate::default();
let builder = PromptBuilder::new(template);

let prompt = builder
    .task_type("code_generation")
    .context("language", "Rust")
    .context("framework", "Tokio")
    .build("创建异步文件读取函数");

println!("{}", prompt);
```

### 5. 动态加载模板

```rust
let mut engine = PlanningEngine::new(model);

// 运行时加载新模板
engine.load_template("prompts/custom-template.yaml")?;

let plan = engine.analyze_task("实现新功能").await?;
```

## 内置场景

Task Runner 提供了多个预定义场景：

### 通用场景

1. **code_generation** - 代码生成
   - 新功能实现
   - API 设计
   - 数据结构定义

2. **refactoring** - 代码重构
   - 改善代码结构
   - 消除代码异味
   - 提高可维护性

3. **debugging** - 调试
   - 错误诊断
   - 问题修复
   - 根因分析

4. **testing** - 测试
   - 单元测试
   - 集成测试
   - 测试覆盖

5. **documentation** - 文档
   - API 文档
   - 使用示例
   - 架构说明

6. **architecture** - 架构设计
   - 系统设计
   - 组件划分
   - 接口定义

7. **optimization** - 性能优化
   - 性能分析
   - 优化策略
   - 基准测试

8. **file_operations** - 文件操作
   - 文件读写
   - 路径处理
   - 格式解析

9. **command_execution** - 命令执行
   - Shell 命令
   - 进程管理
   - 输出处理

### Rust 特定场景

（在 `prompts/rust-project.yaml` 中）

1. **async_programming** - 异步编程
2. **error_handling** - 错误处理
3. **trait_design** - Trait 设计
4. **module_organization** - 模块组织

## 自定义模板

### 创建自定义模板

创建 `prompts/my-template.yaml`:

```yaml
global:
  system_role: |
    你是一个专业的 [领域] 专家。
    你擅长 [技能]。
  
  output_format:
    format_type: "structured_text"
    required_fields:
      - "UNDERSTANDING"
      - "APPROACH"
      - "COMPLEXITY"
      - "REQUIREMENTS"
    field_descriptions:
      UNDERSTANDING: "任务理解"
      APPROACH: "解决方案"
      COMPLEXITY: "复杂度"
      REQUIREMENTS: "依赖项"
  
  constraints:
    - "约束 1"
    - "约束 2"

project:
  tech_stack:
    - "技术 1"
    - "技术 2"
  conventions:
    - "规范 1"
    - "规范 2"
  context: "项目背景"

scenarios:
  custom_scenario:
    name: "自定义场景"
    description: "场景描述"
    instructions:
      - "指令 1"
      - "指令 2"
    examples:
      - input: "示例输入"
        output: "示例输出"
```

### 模板字段说明

**global.system_role**: 定义 AI 的角色和能力
- 应该清晰描述 AI 的专业领域
- 设定期望的行为和风格

**global.output_format**: 指定输出格式
- `format_type`: 格式类型（structured_text, json, markdown）
- `required_fields`: 必需字段列表
- `field_descriptions`: 字段说明

**global.constraints**: 通用约束
- 代码质量要求
- 最佳实践
- 安全考虑

**project**: 项目特定信息
- `tech_stack`: 使用的技术栈
- `conventions`: 编码规范
- `context`: 项目背景
- `architecture`: 架构说明

**scenarios**: 场景定义
- `name`: 场景名称
- `description`: 场景描述
- `instructions`: 具体指令
- `output_structure`: 输出结构（可选）
- `examples`: 示例（可选）

## 任务类型自动推断

PlanningEngine 会根据请求内容自动推断任务类型：

| 关键词 | 推断类型 |
|--------|----------|
| test, unit test | testing |
| refactor, improve | refactoring |
| debug, fix, error | debugging |
| document, doc | documentation |
| optimize, performance | optimization |
| design, architecture | architecture |
| read, write, file | file_operations |
| run, execute, command | command_execution |
| create, generate, implement | code_generation |

示例：

```rust
// 自动推断为 "testing"
engine.analyze_task("为 PromptBuilder 编写单元测试").await?;

// 自动推断为 "refactoring"
engine.analyze_task("重构 agent.rs 的执行逻辑").await?;

// 自动推断为 "debugging"
engine.analyze_task("修复编译错误").await?;
```

## 最佳实践

### 1. 分层设计

- **全局层**: 定义通用的角色和约束
- **项目层**: 注入项目特定的技术栈和规范
- **场景层**: 提供针对性的指导

### 2. 使用分隔符

在提示词中使用清晰的分隔符：
- 使用 `---` 分隔不同部分
- 使用 ` ``` ` 包裹代码和用户输入
- 使用 `#` 标题组织结构

### 3. 提供示例

为复杂场景提供 few-shot 示例：
```yaml
examples:
  - input: "具体输入"
    output: "期望输出"
```

### 4. 结构化输出

要求 AI 以结构化格式输出：
- 使用固定的字段名（UNDERSTANDING, APPROACH 等）
- 提供字段描述
- 便于程序解析

### 5. 上下文注入

动态注入相关上下文：
```rust
builder
    .context("current_file", "src/agent.rs")
    .context("error_message", "missing module")
    .build(request)
```

## 高级用法

### 1. 动态场景注册

```rust
use task_runner::prompts::{PromptTemplate, ScenarioPrompt};

let mut template = PromptTemplate::default();

// 添加自定义场景
template.add_scenario(
    "custom_task".to_string(),
    ScenarioPrompt {
        name: "Custom Task".to_string(),
        description: "My custom task type".to_string(),
        instructions: vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
        ],
        output_structure: None,
        examples: vec![],
    },
);
```

### 2. 模板保存

```rust
// 保存修改后的模板
template.to_file("prompts/modified-template.yaml")?;
```

### 3. 多模板管理

```rust
// 为不同项目使用不同模板
let rust_template = PromptTemplate::from_file("prompts/rust-project.yaml")?;
let python_template = PromptTemplate::from_file("prompts/python-project.yaml")?;

let rust_engine = PlanningEngine::with_template(model.clone(), rust_template);
let python_engine = PlanningEngine::with_template(model.clone(), python_template);
```

## 与 Codex/Roo-Code 对比

| 特性 | Task Runner | Codex | Roo-Code |
|------|-------------|-------|----------|
| 分层结构 | ✅ 三层 | ✅ 多层 | ✅ 三层 |
| 外置配置 | ✅ YAML | ✅ 多格式 | ✅ JSON |
| 场景支持 | ✅ 9+ 场景 | ✅ 丰富 | ✅ 可扩展 |
| 类型安全 | ✅ Rust | ❌ Python | ❌ TypeScript |
| 动态加载 | ✅ 支持 | ✅ 支持 | ✅ 支持 |
| 示例支持 | ✅ Few-shot | ✅ Few-shot | ✅ Few-shot |

## 参考资源

- [OpenAI Prompt Engineering Guide](https://platform.openai.com/docs/guides/prompt-engineering)
- [Anthropic Prompt Engineering](https://docs.anthropic.com/claude/docs/prompt-engineering)
- [Roo-Code Documentation](https://github.com/RooVetGit/Roo-Code)
- [Task Runner Architecture](./CODE_STRUCTURE.md)

