# Task Runner 代码结构文档

## 📋 目录
- [项目概述](#项目概述)
- [整体架构](#整体架构)
- [核心模块](#核心模块)
- [目录结构](#目录结构)
- [主要功能流程](#主要功能流程)
- [依赖关系](#依赖关系)

## 项目概述

**项目名称**: Task Runner  
**版本**: 0.2.0  
**语言**: Rust (Edition 2021)  
**类型**: AI驱动的任务执行服务  
**许可证**: MIT

Task Runner 是一个AI原生的代码助手服务，支持通过命令行工具和HTTP REST API两种方式使用。它能够理解自然语言任务描述，自主规划并执行任务。

## 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                      用户接口层                              │
│  ┌──────────────┐              ┌──────────────┐            │
│  │  CLI (main)  │              │  HTTP Server │            │
│  └──────┬───────┘              └──────┬───────┘            │
│         │                              │                    │
└─────────┼──────────────────────────────┼────────────────────┘
          │                              │
┌─────────┼──────────────────────────────┼────────────────────┐
│         │        核心服务层            │                    │
│         ▼                              ▼                    │
│  ┌──────────────┐              ┌──────────────┐            │
│  │  CodeAgent   │◄─────────────┤   Service    │            │
│  └──────┬───────┘              └──────────────┘            │
│         │                                                   │
└─────────┼───────────────────────────────────────────────────┘
          │
┌─────────┼───────────────────────────────────────────────────┐
│         │           AI引擎层                                │
│         ▼                                                   │
│  ┌──────────────┐         ┌──────────────┐                │
│  │ Understanding│         │  Execution   │                │
│  │   Engine     │────────►│   Engine     │                │
│  └──────┬───────┘         └──────┬───────┘                │
│         │                        │                         │
└─────────┼────────────────────────┼─────────────────────────┘
          │                        │
┌─────────┼────────────────────────┼─────────────────────────┐
│         │      基础设施层        │                         │
│         ▼                        ▼                         │
│  ┌──────────────┐         ┌──────────────┐                │
│  │ Language     │         │    Tools     │                │
│  │   Models     │         │   Registry   │                │
│  └──────────────┘         └──────────────┘                │
│  • OpenAI                 • ReadFile                       │
│  • Anthropic              • WriteFile                      │
│  • Zhipu GLM              • RunCommand                     │
│  • Local Models           • ListFiles                      │
└────────────────────────────────────────────────────────────┘
```

## 核心模块

### 1. **main.rs** - 程序入口
**路径**: `src/main.rs`  
**功能**: 应用程序的主入口点

**主要功能**:
- 初始化日志系统 (tracing)
- 解析命令行参数
- 调用CLI模块执行命令
- 错误处理和退出

**关键代码**:
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    cli.run().await
}
```

### 2. **lib.rs** - 库入口
**路径**: `src/lib.rs`  
**功能**: 定义公共API和模块导出

**导出的核心类型**:
- `TaskAgent` - 通用任务代理（主要类型）
- `CodeAgent` - TaskAgent 的别名（已废弃，保留向后兼容）
- `AgentConfig` - 配置管理
- `LanguageModel` - AI模型接口
- `Tool` - 工具系统接口
- `PromptTemplate` - 提示词模板
- `PlanningEngine` - 任务理解引擎
- `TaskRequest/TaskResponse` - 服务类型 (需要 "service" feature)

**模块组织**:
```rust
pub mod agent;         // 核心代理 (296行) [重构精简]
pub mod config;        // 配置管理
pub mod models;        // AI模型
pub mod prompts;       // 提示词工程系统 (300行) [新增]
pub mod parser;  // 任务辅助函数 (292行) [新增]
pub mod tools;         // 工具系统
pub mod types;         // 类型定义
pub mod errors;        // 错误处理
pub mod understanding; // 任务理解 (186行) [重构]
pub mod execution;     // 任务执行
pub mod cli;           // 命令行接口

#[cfg(feature = "service")]
pub mod service;       // HTTP服务 (可选)
```

### 3. **agent.rs** - 核心任务代理
**路径**: `src/agent.rs` (296行，重构后减少29%)
**功能**: 实现通用的AI任务代理，专注于主工作流程（理解、执行协调、结果生成）

**核心结构** (已优化):
```rust
/// Main AI-Native Task Agent (struct, not trait)
///
/// A general-purpose agent that can handle various types of tasks including:
/// - Code generation and refactoring
/// - File operations
/// - Command execution
/// - Documentation
/// - Testing
/// - And any other tasks defined through the tool system
pub struct TaskAgent {
    model: Arc<dyn LanguageModel>,           // 改为Arc支持共享
    tools: Arc<Mutex<ToolRegistry>>,
    config: AgentConfig,
    understanding_engine: PlanningEngine, // 新增：专门的理解引擎
    _error_handler: ErrorHandler,
}

/// Type alias for backward compatibility (deprecated)
#[deprecated(since = "0.2.1", note = "Use `TaskAgent` instead")]
pub type CodeAgent = TaskAgent;

impl TaskAgent {
    // 实现方法
}
```

**重要说明**:
- `TaskAgent` 是一个 **struct**（结构体），不是 trait（特征）
- 从 v0.2.1 开始，`CodeAgent` 重命名为 `TaskAgent` 以体现其通用性
- `CodeAgent` 作为别名保留以保持向后兼容，但已标记为 deprecated
- 辅助函数（文件操作、命令执行、文本解析）已移至 `parser` 模块

**职责分离**:
- `agent.rs` - 主工作流程：任务理解、执行协调、结果生成
- `parser.rs` - 辅助功能：文件操作、命令执行、文本解析

**主要方法**:
- `new()` - 创建新的代理实例（自动创建理解引擎）
- `process_task()` - 处理任务的主入口
- `understand_task()` - 委托给 PlanningEngine
- `execute_task_real()` - 执行任务
- `register_tool()` - 注册工具
- `get_model()` - 获取模型引用

**工作流程** (优化后):
1. 接收任务请求
2. 理解阶段: 委托给 PlanningEngine（使用提示词模板）
3. 执行阶段: 根据理解结果执行任务
4. 返回结果

**重构改进**:
- ✅ 职责分离：理解逻辑移到 understanding.rs
- ✅ 代码精简：从 416 行减少到 345 行
- ✅ 共享模型：使用 Arc 支持多组件共享

### 4. **cli.rs** - 命令行接口
**路径**: `src/cli.rs`  
**功能**: 提供命令行交互界面

**支持的命令**:
- `task` - 执行单个任务
- `interactive` - 进入交互模式
- `tools` - 列出可用工具
- `config` - 显示配置

**输出格式**:
- `text` - 纯文本 (默认)
- `json` - JSON格式
- `verbose` - 详细输出

### 5. **config.rs** - 配置管理
**路径**: `src/config.rs`  
**功能**: 管理应用配置

**配置结构**:
```rust
pub struct AgentConfig {
    pub model: ModelConfig,          // AI模型配置
    pub execution: ExecutionConfig,  // 执行配置
    pub safety: SafetyConfig,        // 安全配置
    pub tools: ToolConfig,           // 工具配置
    pub logging: LoggingConfig,      // 日志配置
}
```

**支持的AI提供商**:
- OpenAI (GPT系列)
- Anthropic (Claude系列)
- Zhipu (GLM系列)
- Local (本地模型)

**配置加载方式**:
1. 从TOML文件加载
2. 从环境变量加载
3. 使用默认配置

### 6. **models.rs** - AI模型抽象
**路径**: `src/models.rs`
**功能**: 定义AI模型接口和实现 (使用 llm-connector)

**核心接口**:
```rust
#[async_trait]
pub trait LanguageModel: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse, ModelError>;
    async fn complete_with_tools(&self, prompt: &str, tools: &[ToolDefinition])
        -> Result<ModelResponse, ModelError>;
    fn model_name(&self) -> &str;
    fn supports_tools(&self) -> bool;
}
```

**实现的模型** (通过 llm-connector):
- `OpenAIModel` - OpenAI API集成 (使用 llm-connector)
- `AnthropicModel` - Anthropic API集成 (使用 llm-connector)
- `ZhipuModel` - 智谱AI集成 (使用 llm-connector)
- `LocalModel` - 本地模型支持 (使用 llm-connector)
- `MockModel` - 测试用模拟模型

**重要变更**: 从 v0.2.0 开始,所有 LLM 调用都通过 `llm-connector` crate 统一管理,提供更好的协议适配和错误处理。详见 [LLM Connector 迁移文档](./LLM_CONNECTOR_MIGRATION.md)。

### 7. **tools.rs** - 工具系统
**路径**: `src/tools.rs`  
**功能**: 定义工具接口和内置工具

**工具接口**:
```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Vec<Parameter>;
    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult, ToolError>;
}
```

**内置工具**:
- `ReadFileTool` - 读取文件
- `WriteFileTool` - 写入文件
- `RunCommandTool` - 执行命令
- `ListFilesTool` - 列出文件

**工具注册表**:
```rust
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}
```

### 8. **types.rs** - 类型定义
**路径**: `src/types.rs`  
**功能**: 定义核心数据类型

**主要类型**:
- `Task` - 任务表示
- `TaskStatus` - 任务状态 (Pending/InProgress/Completed/Failed)
- `TaskResult` - 任务结果
- `TaskPlan` - 任务计划
- `TaskComplexity` - 复杂度 (Simple/Moderate/Complex)
- `ExecutionContext` - 执行上下文
- `ExecutionStep` - 执行步骤
- `Action` - 动作类型
- `ActionType` - 动作类型枚举

### 9. **errors.rs** - 错误处理
**路径**: `src/errors.rs`  
**功能**: 定义错误类型和处理逻辑

**错误类型**:
```rust
pub enum AgentError {
    ModelError(ModelError),
    ToolError(ToolError),
    NetworkError(String),
    TimeoutError,
    ConfigError(String),
    UnknownError(String),
}
```

**错误处理器**:
```rust
pub struct ErrorHandler {
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
}
```

支持自动重试机制,针对网络错误、超时和限流错误。

### 10. **understanding.rs** - 任务理解引擎
**路径**: `src/understanding.rs` (186行，重构后新增)
**功能**: 使用AI分析和理解任务，集成提示词工程系统

**核心结构**:
```rust
pub struct PlanningEngine {
    model: Arc<dyn LanguageModel>,
    prompt_template: PromptTemplate,  // 提示词模板
}
```

**主要方法**:
- `new()` - 使用默认模板创建引擎
- `with_template()` - 使用自定义模板创建引擎
- `load_template()` - 从文件加载模板
- `understand_task()` - 分析任务（自动推断类型）
- `understand_task_with_type()` - 分析任务（指定类型）
- `build_understanding_prompt()` - 构建提示词（使用模板系统）
- `infer_task_type()` - 推断任务类型
- `parse_task_plan()` - 解析AI响应

**核心功能**:
- ✅ 使用提示词模板系统构建提示词
- ✅ 自动推断任务类型（9种场景）
- ✅ 调用AI模型分析任务
- ✅ 解析AI响应生成任务计划
- ✅ 评估任务复杂度
- ✅ 识别依赖和需求

**任务类型推断**:
- testing: test, unit test
- refactoring: refactor, improve
- debugging: debug, fix, error
- documentation: document, doc
- optimization: optimize, performance
- architecture: design, architecture
- file_operations: read, write, file
- command_execution: run, execute, command
- code_generation: create, generate, implement

### 11. **prompts.rs** - 提示词工程系统
**路径**: `src/prompts.rs` (300行，新增)
**功能**: 灵活的提示词模板系统，参考 OpenAI Codex 和 Roo-Code

**核心结构**:
```rust
pub struct PromptTemplate {
    pub global: GlobalTemplate,              // 全局模板
    pub project: Option<ProjectRules>,       // 项目规则
    pub scenarios: HashMap<String, ScenarioPrompt>, // 场景提示词
}

pub struct PromptBuilder {
    template: PromptTemplate,
    task_type: Option<String>,
    context: HashMap<String, String>,
}
```

**主要组件**:
- `GlobalTemplate` - 系统角色、输出格式、约束
- `ProjectRules` - 技术栈、规范、上下文
- `ScenarioPrompt` - 场景特定指令和示例
- `OutputFormat` - 输出格式规范
- `PromptExample` - Few-shot 示例

**核心功能**:
- ✅ 分层提示词结构（全局+项目+场景）
- ✅ YAML 外置配置
- ✅ 流式 API 构建器
- ✅ 动态上下文注入
- ✅ 模板加载和保存
- ✅ 9+ 预定义场景

**内置场景**:
1. code_generation - 代码生成
2. refactoring - 代码重构
3. debugging - 调试修复
4. testing - 测试编写
5. documentation - 文档编写
6. architecture - 架构设计
7. optimization - 性能优化
8. file_operations - 文件操作
9. command_execution - 命令执行

**配置文件**:
- `prompts/default.yaml` - 默认模板
- `prompts/rust-project.yaml` - Rust 项目模板

详见：[提示词工程文档](./PROMPT_ENGINEERING.md)

### 12. **parser.rs** - 任务辅助函数
**路径**: `src/parser.rs` (292行，新增)
**功能**: 提供任务执行的辅助功能，从 agent.rs 分离出来

**核心函数**:
```rust
// 文本解析
pub fn extract_file_path(text: &str) -> Option<String>
pub fn extract_command(text: &str) -> Option<String>
pub fn extract_directory_path(text: &str) -> Option<String>

// 文件操作
pub async fn read_file(path: &str) -> Result<String, AgentError>
pub async fn list_files(path: &str) -> Result<String, AgentError>

// 命令执行
pub async fn run_command(command: &str) -> Result<String, AgentError>
```

**主要功能**:
- ✅ **文本解析**: 从自然语言中提取文件路径、命令、目录路径
- ✅ **文件操作**: 读取文件、列出目录内容
- ✅ **命令执行**: 执行 shell 命令并返回输出
- ✅ **错误处理**: 统一的错误处理和友好的错误消息
- ✅ **测试覆盖**: 7 个单元测试 + 2 个文档测试

**设计优势**:
- 独立模块，职责单一
- 纯函数设计，易于测试
- 可被其他模块复用
- 保持 agent.rs 简洁清晰

**支持的文件扩展名**:
- 代码: .rs, .py, .js, .ts, .go, .java, .c, .cpp, .h
- 配置: .toml, .yaml, .yml, .json, .xml
- 文档: .txt, .md
- 脚本: .sh, .bash, .zsh, .fish

### 13. **execution.rs** - 任务执行引擎
**路径**: `src/execution.rs`
**功能**: 执行任务计划

**执行流程**:
1. 创建执行上下文
2. AI决策下一步动作
3. 执行动作 (使用工具/思考/完成)
4. 记录结果
5. 循环直到任务完成

**动作类型**:
- `UseTool` - 使用工具
- `Think` - 思考推理
- `Complete` - 完成任务
- `AskClarification` - 请求澄清

### 12. **service/** - HTTP服务模块
**路径**: `src/service/`  
**功能**: 提供HTTP REST API (需要 "service" feature)

**子模块**:
- `mod.rs` - 模块导出
- `core.rs` - 核心服务实现 (`CodeAgentService`)
- `api.rs` - HTTP API路由和处理器
- `error.rs` - 服务错误类型
- `metrics.rs` / `metrics_simple.rs` - 指标收集

**API端点**:
- `POST /api/v1/tasks` - 执行单个任务
- `POST /api/v1/tasks/batch` - 批量执行任务
- `GET /health` - 健康检查
- `GET /metrics` - 指标查询

### 13. **service_types.rs** - 服务类型定义
**路径**: `src/service_types.rs`  
**功能**: 定义服务相关的数据类型

**主要类型**:
- `TaskRequest` - 任务请求
- `TaskResponse` - 任务响应
- `BatchTaskRequest` - 批量任务请求
- `BatchTaskResponse` - 批量任务响应
- `TaskContext` - 任务上下文
- `TaskPriority` - 任务优先级
- `ServiceConfig` - 服务配置
- `ServiceStatus` - 服务状态

## 目录结构

```
task-runner/
├── Cargo.toml              # 项目配置和依赖
├── Cargo.lock              # 依赖锁定文件
├── config.toml             # 运行时配置文件
├── README.md               # 项目说明 (英文)
├── README_CN.md            # 项目说明 (中文)
│
├── src/                    # 源代码目录
│   ├── main.rs            # 程序入口
│   ├── lib.rs             # 库入口
│   ├── agent.rs           # 核心任务代理 (296行，重构精简)
│   ├── cli.rs             # 命令行接口
│   ├── config.rs          # 配置管理
│   ├── errors.rs          # 错误处理
│   ├── execution.rs       # 执行引擎
│   ├── models.rs          # AI模型
│   ├── prompts.rs         # 提示词工程系统 (300行，新增)
│   ├── parser.rs    # 任务辅助函数 (292行，新增)
│   ├── tools.rs           # 工具系统
│   ├── types.rs           # 类型定义
│   ├── understanding.rs   # 理解引擎 (186行，重构后)
│   ├── service_types.rs   # 服务类型
│   │
│   ├── server/            # 服务器相关
│   │   └── main.rs        # HTTP服务器入口
│   │
│   └── service/           # 服务模块
│       ├── mod.rs         # 模块导出
│       ├── core.rs        # 核心服务
│       ├── api.rs         # API路由
│       ├── error.rs       # 服务错误
│       ├── metrics.rs     # 指标收集
│       └── metrics_simple.rs  # 简化指标
│
├── doc/                   # 文档目录
│   ├── CODE_STRUCTURE.md  # 代码结构文档 (本文件)
│   ├── PROMPT_ENGINEERING.md  # 提示词工程文档 (新增)
│   ├── REFACTORING_GUIDE.md   # 重构指南 (新增)
│   ├── SERVICE_API.md     # API文档
│   ├── RUST_ANALYZER_SETUP.md  # 开发环境配置
│   ├── system-design.md   # 系统设计 (英文)
│   └── system-design-cn.md # 系统设计 (中文)
│
├── prompts/               # 提示词模板目录 (新增)
│   ├── default.yaml       # 默认提示词模板
│   └── rust-project.yaml  # Rust项目专用模板
│
├── examples/              # 示例代码
│   ├── Cargo.toml         # 示例项目配置
│   ├── README.md          # 示例说明
│   ├── http_client.rs     # HTTP客户端示例
│   ├── in_process_service.rs  # 进程内服务示例
│   ├── prompt_engineering.rs  # 提示词工程示例 (新增)
│   └── rust_client.rs     # Rust客户端示例
│
├── ref/                   # 参考实现
│   ├── mcp-client/        # MCP客户端参考
│   ├── mcp-server/        # MCP服务器参考
│   └── mcp-types/         # MCP类型定义
│
└── target/                # 编译输出目录
    └── debug/             # 调试构建
```

## 主要功能流程

### 任务执行流程

```
用户输入任务
    │
    ▼
┌─────────────────────┐
│  CLI.run()          │  解析命令行参数
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ create_agent()      │  创建AI代理实例
│  - 加载配置         │
│  - 初始化模型       │
│  - 注册工具         │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ agent.process_task()│  处理任务
└──────┬──────────────┘
       │
       ├─────────────────────────────────┐
       │                                 │
       ▼                                 ▼
┌──────────────────┐          ┌──────────────────┐
│ understand_task()│          │ execute_task()   │
│  1. 构建提示词    │          │  1. 创建执行上下文│
│  2. 调用AI模型    │──────────►  2. AI决策动作   │
│  3. 解析响应      │          │  3. 执行动作     │
│  4. 生成计划      │          │  4. 记录结果     │
└──────────────────┘          └──────┬───────────┘
                                     │
                                     ▼
                              ┌──────────────────┐
                              │  返回TaskResult  │
                              │  - success       │
                              │  - summary       │
                              │  - details       │
                              │  - execution_time│
                              └──────────────────┘
```

### HTTP服务请求流程

```
HTTP请求
    │
    ▼
┌─────────────────────┐
│  Axum Router        │  路由分发
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│  API Handler        │  处理请求
│  - execute_task     │
│  - batch_execute    │
│  - health_check     │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ CodeAgentService    │  服务层
│  - 任务队列管理     │
│  - 并发控制         │
│  - 指标收集         │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│  CodeAgent          │  核心代理
│  (同上述流程)       │
└─────────────────────┘
```

### AI模型调用流程

```
prompt (提示词)
    │
    ▼
┌─────────────────────┐
│ LanguageModel trait │  模型接口
└──────┬──────────────┘
       │
       ├──────────┬──────────┬──────────┐
       ▼          ▼          ▼          ▼
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│ OpenAI   │ │Anthropic │ │  Zhipu   │ │  Local   │
│  Model   │ │  Model   │ │  Model   │ │  Model   │
└────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘
     │            │            │            │
     └────────────┴────────────┴────────────┘
                  │
                  ▼
          ┌──────────────────┐
          │  ModelResponse   │
          │  - content       │
          │  - tool_calls    │
          │  - usage         │
          └──────────────────┘
```

### 工具执行流程

```
工具调用请求
    │
    ▼
┌─────────────────────┐
│  ToolRegistry       │  工具注册表
│  - get_tool()       │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│  Tool.execute()     │  执行工具
└──────┬──────────────┘
       │
       ├──────────┬──────────┬──────────┐
       ▼          ▼          ▼          ▼
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│ReadFile  │ │WriteFile │ │RunCommand│ │ListFiles │
│  Tool    │ │  Tool    │ │  Tool    │ │  Tool    │
└────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘
     │            │            │            │
     └────────────┴────────────┴────────────┘
                  │
                  ▼
          ┌──────────────────┐
          │   ToolResult     │
          │  - success       │
          │  - content       │
          │  - summary       │
          └──────────────────┘
```

## 依赖关系

### 核心依赖

**异步运行时**:
- `tokio` (1.0) - 异步运行时,支持完整特性
- `async-trait` (0.1) - 异步trait支持
- `futures` (0.3) - Future工具

**HTTP相关**:
- `reqwest` (0.11) - HTTP客户端
- `axum` (0.7) - HTTP服务器框架 [可选]
- `tower` (0.4) - 中间件支持 [可选]
- `tower-http` (0.5) - HTTP中间件 (CORS, tracing) [可选]

**LLM集成**:
- `llm-connector` (0.2.0) - 统一的 LLM 提供商接口

**序列化**:
- `serde` (1.0) - 序列化框架
- `serde_json` (1.0) - JSON支持
- `toml` (0.8) - TOML配置文件

**工具库**:
- `uuid` (1.0) - UUID生成
- `chrono` (0.4) - 时间处理
- `clap` (4.0) - 命令行解析
- `anyhow` (1.0) - 错误处理
- `thiserror` (1.0) - 错误定义

**日志**:
- `tracing` (0.1) - 结构化日志
- `tracing-subscriber` (0.3) - 日志订阅器

**监控** [可选]:
- `metrics` (0.23) - 指标收集
- `metrics-exporter-prometheus` (0.13) - Prometheus导出

### Feature标志

```toml
[features]
default = ["core"]
core = []                    # 核心功能
service = [                  # HTTP服务功能
    "axum",
    "tower",
    "tower-http",
    "metrics",
    "metrics-exporter-prometheus"
]
full = ["core", "service"]   # 完整功能
```

### 模块依赖图

```
main.rs
  └─► cli.rs
       ├─► agent.rs
       │    ├─► models.rs
       │    ├─► tools.rs
       │    ├─► understanding.rs
       │    ├─► execution.rs
       │    ├─► types.rs
       │    ├─► errors.rs
       │    └─► config.rs
       └─► config.rs

lib.rs
  ├─► agent.rs
  ├─► cli.rs
  ├─► config.rs
  ├─► models.rs
  ├─► tools.rs
  ├─► types.rs
  ├─► errors.rs
  ├─► understanding.rs
  ├─► execution.rs
  ├─► service_types.rs [feature="service"]
  └─► service/ [feature="service"]
       ├─► mod.rs
       ├─► core.rs
       ├─► api.rs
       ├─► error.rs
       └─► metrics.rs

server/main.rs [feature="service"]
  └─► service/
       └─► api.rs
```

## 关键设计模式

### 1. Trait抽象模式
使用Rust的trait系统实现接口抽象:
- `LanguageModel` trait - 支持多种AI模型
- `Tool` trait - 可扩展的工具系统

### 2. 异步并发模式
全面使用async/await和tokio运行时:
- 所有IO操作都是异步的
- 支持并发任务执行
- 使用Arc和Mutex实现线程安全的共享状态

### 3. 错误处理模式
使用thiserror定义结构化错误:
- 分层错误类型 (AgentError, ModelError, ToolError)
- 支持错误转换和传播
- 内置重试机制

### 4. 配置管理模式
多层配置加载策略:
- 文件配置 (TOML)
- 环境变量
- 默认值
- 支持环境变量替换

### 5. 服务架构模式
可选的HTTP服务层:
- 核心逻辑与服务层分离
- 支持进程内调用和HTTP调用
- 统一的客户端接口

## 扩展点

### 添加新的AI模型
1. 实现 `LanguageModel` trait
2. 在 `config.rs` 中添加新的 `ModelProvider` 变体
3. 在 `cli.rs` 的 `create_agent()` 中添加创建逻辑

### 添加新的工具
1. 实现 `Tool` trait
2. 在 `agent.rs` 的 `create_agent()` 中注册工具
3. 可选: 在配置文件中添加工具配置

### 添加新的API端点
1. 在 `service/api.rs` 中定义新的处理函数
2. 在路由中注册新端点
3. 在 `service_types.rs` 中定义请求/响应类型

## 测试策略

### 单元测试
每个模块都包含单元测试:
- `agent.rs` - 代理创建和工具注册测试
- `models.rs` - 模型响应测试
- `tools.rs` - 工具执行测试

### 集成测试
在 `examples/` 目录中提供集成测试示例:
- `http_client.rs` - HTTP API测试
- `in_process_service.rs` - 进程内服务测试
- `rust_client.rs` - Rust客户端测试

### 测试工具
- `MockModel` - 模拟AI模型用于测试
- `tokio-test` - 异步测试支持
- `mockall` - Mock框架

## 性能考虑

### 并发处理
- 使用tokio的异步运行时
- 支持并发任务执行
- 工具注册表使用Arc<Mutex<>>实现线程安全

### 资源管理
- 连接池复用 (HTTP客户端)
- 合理的超时设置
- 限流和优先级队列

### 优化选项
- Release模式使用 `opt-level = 3`
- 可选的指标收集 (仅在需要时启用)
- 按需加载的feature标志

## 安全考虑

### 安全配置
- `SafetyConfig` - 安全检查配置
- `allowed_directories` - 允许的目录列表
- `blocked_commands` - 禁止的命令列表

### API安全
- CORS配置
- 可扩展的认证机制
- 请求验证

### 错误处理
- 不暴露敏感信息
- 结构化错误日志
- 优雅的错误恢复

---

**文档版本**: 1.0
**最后更新**: 2025-10-06
**维护者**: Task Runner Team

