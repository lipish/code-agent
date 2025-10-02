# AI-Native 代码助手

一个用 Rust 构建的 AI-Native 代码助手，旨在最大化 AI 自主性，同时提供可靠的执行能力。

## 项目概述

本项目实现了一个极简的 AI-Native 代码助手，核心设计理念是给予 AI 模型最大的决策自由度，同时确保系统的安全性和可靠性。与传统的强工作流代码助手不同，该系统充分信任 AI 的判断能力，提供灵活的工具集以实现自主任务执行。

## 核心特性

- **AI-Native 架构**：最大化 AI 自主权，最小化约束限制
- **多模型支持**：支持 OpenAI、Anthropic 和本地模型
- **工具系统**：可扩展的工具包，支持文件操作、命令执行等
- **配置管理**：通过配置文件和环境变量进行灵活配置
- **错误处理**：强大的错误恢复和重试机制
- **交互模式**：提供交互式命令行界面进行任务执行

## 系统架构

系统围绕三个核心组件构建：

1. **理解引擎 (Understanding Engine)**：分析和分解用户任务
2. **执行引擎 (Execution Engine)**：使用 AI 驱动的决策执行任务
3. **工具注册表 (Tool Registry)**：管理和执行各种工具

## 快速开始

### 环境要求

- Rust 1.75 或更高版本
- OpenAI API 密钥或 Anthropic API 密钥（用于云模型）
- 本地模型设置（可选，用于本地模型部署）

### 安装步骤

1. 克隆项目仓库：
```bash
git clone <repository-url>
cd ai-agent
```

2. 设置环境变量：
```bash
cp .env.example .env
# 编辑 .env 文件，填入你的 API 密钥
```

3. 构建项目：
```bash
cargo build --release
```

### 使用指南

#### 单任务执行

```bash
# 使用 OpenAI 模型
OPENAI_API_KEY=your_key cargo run -- task "读取 package.json 文件并添加测试脚本"

# 使用配置文件
cargo run -- task "创建 hello world 文件" --config config.toml --output json
```

#### 交互模式

```bash
cargo run -- interactive
```

#### 查看可用工具

```bash
cargo run -- tools
```

#### 显示当前配置

```bash
cargo run -- config
```

## 配置说明

系统支持多种配置方式：

1. **配置文件**（`config.toml`）
2. **环境变量**
3. **命令行参数**

### 配置文件示例

```toml
[model]
provider = "openai"
model_name = "gpt-4-turbo-preview"
api_key = "${OPENAI_API_KEY}"
max_tokens = 4000
temperature = 0.7

[execution]
max_steps = 50
timeout_seconds = 300
max_retries = 3
retry_delay_seconds = 2

[safety]
enable_safety_checks = true
allowed_directories = [".", "/tmp"]
blocked_commands = ["rm -rf /", "format", "fdisk"]

[tools]
auto_discovery = true
enabled_tools = ["read_file", "write_file", "run_command", "list_files"]

[logging]
level = "info"
file = "agent.log"
console = true
format = "pretty"
```

## 内置工具

- **read_file**：读取文件内容
- **write_file**：向文件写入内容
- **run_command**：执行 shell 命令
- **list_files**：列出目录内容

## 支持的模型

### OpenAI 模型
- GPT-4、GPT-4 Turbo
- GPT-3.5 Turbo

### Anthropic 模型
- Claude 3 Opus、Sonnet、Haiku

### 本地模型
- Ollama 兼容模型
- 自定义本地模型端点

## 开发指南

### 项目结构

```
src/
├── agent/          # 核心代理实现
├── models/         # 语言模型适配器
├── tools/          # 工具系统和实现
├── config/         # 配置管理
├── cli/            # 命令行界面
└── errors/         # 错误类型和处理
```

### 构建命令

```bash
# 调试版本构建
cargo build

# 发布版本构建
cargo build --release

# 运行测试
cargo test
```

### 添加自定义工具

实现 `Tool` trait：

```rust
use async_trait::async_trait;
use crate::tools::{Tool, ToolResult, ToolError, ToolArgs};

pub struct MyTool;

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str { "my_tool" }
    fn description(&self) -> &str { "我的工具描述" }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter::required("param1", "string", "第一个参数"),
        ]
    }

    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult, ToolError> {
        // 实现具体功能
        Ok(ToolResult::text("任务完成".to_string()))
    }
}
```

## 安全特性

- **文件访问限制**：防止访问敏感系统文件
- **命令阻止机制**：阻止危险的 shell 命令执行
- **目录约束**：将文件操作限制在允许的目录范围内
- **资源限制**：可配置的超时时间和步骤限制

## 设计理念

该代理采用 AI-Native 设计方法：

1. **信任 AI**：给予 AI 最大的决策自主权
2. **最小约束**：只保留必要的安全和操作限制
3. **灵活执行**：AI 自主决定每个任务的最佳执行方式
4. **强大错误处理**：从失败中优雅恢复
5. **可扩展性**：易于添加新工具和功能

## 开源协议

MIT 协议 - 详见 LICENSE 文件。

## 贡献指南

欢迎贡献代码！请随时提交 Pull Request。

## 发展路线图

- [ ] 基于 Web 的用户界面
- [ ] 工具插件系统
- [ ] 高级错误恢复策略
- [ ] 性能优化
- [ ] 与更多 AI 模型的集成
- [ ] 自定义工具开发框架

## 技术亮点

- **极简架构**：专注于核心功能，避免过度设计
- **高度可扩展**：模块化设计，易于扩展新功能
- **安全可靠**：完善的安全机制和错误处理
- **多模型兼容**：支持主流 AI 模型和本地部署
- **配置灵活**：多种配置方式，适应不同使用场景