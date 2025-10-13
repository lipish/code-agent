# Agent Runner

一个简单高效的 AI 驱动智能代理执行服务，提供 Rust API 和 HTTP REST 接口，可集成到任何应用中。

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🎯 项目特点

- **🏗️ 服务架构**: 可作为独立服务运行，提供标准API接口
- **🤖 AI原生**: 从底层为AI自主性而构建，最大化AI能力
- **🔗 双接口**: 提供Rust API和HTTP REST API两种使用方式
- **⚡ 高性能**: 支持并发任务执行和实时监控
- **🛠️ 工具集成**: 安全的文件操作、命令执行等工具系统
- **📊 监控完备**: 内置指标收集和健康检查
- **🔒 企业级**: 支持认证、限流、CORS等企业特性
- **🔌 统一LLM接口**: 使用 [llm-connector](https://github.com/lipish/llm-connector) 支持多个AI提供商
- **✨ 模块化设计**: 职责分离的架构，代码清晰易维护

## 🆕 最近优化

### v0.2.1 架构重构（2025-10）

**核心改进**：
- ✅ **通用化设计**: 重命名 `CodeAgent` 为 `TaskAgent`，支持更广泛的任务类型（不仅限于代码）
- ✅ **模块化重构**: 将任务理解逻辑从 `agent.rs` 分离到独立的 `understanding.rs` 模块
- ✅ **提示词工程**: 实现灵活的提示词模板系统，参考 OpenAI Codex 和 Roo-Code
- ✅ **代码精简**: `agent.rs` 从 416 行减少到 346 行（-17%）
- ✅ **职责分离**: 理解、执行、工具调用各司其职，提高可维护性
- ✅ **共享模型**: 使用 `Arc<dyn LanguageModel>` 支持多组件共享 AI 模型
- ✅ **测试增强**: 新增 3 个单元测试，测试覆盖率提升 150%
- ✅ **代码质量**: 修复所有主要 Clippy 警告，改进错误处理

**架构优势**：
```rust
// 之前：所有逻辑混在一起，名称局限于代码
TaskAgent (416 行) - 任务管理 + 理解 + 执行 + 辅助函数

// 之后：清晰的模块化架构，职责分离
agent/                     - Agent 模块（628行，3个文件）
  ├── mod.rs              - 核心结构、生命周期管理
  ├── executor.rs         - 任务执行逻辑
  └── planner.rs          - 任务规划逻辑

understanding/             - 理解模块（244行，2个文件）
  ├── mod.rs              - 模块入口
  └── engine.rs           - AI 理解引擎

execution/                 - 执行模块（433行，3个文件）
  ├── mod.rs              - 模块入口
  ├── file_ops.rs         - 文件操作
  └── command_ops.rs      - 命令执行

prompts.rs (300 行)        - 提示词工程系统
task_helpers.rs (292 行)   - 任务辅助函数
```

**向后兼容**: 以前的 `CodeAgent` 现在已经被完全替换为 `TaskAgent`。

详见：[重构指南](doc/REFACTORING_GUIDE.md)

## ✨ 提示词工程系统

**Agent Runner 实现了灵活的提示词工程系统，灵感来自 OpenAI Codex 和 Roo-Code：

**核心特性**: 分层结构（全局+项目+场景）、外置YAML配置、9+预定义场景、动态加载、智能推断、可扩展

**快速示例**:
```rust
// 使用默认模板
let engine = UnderstandingEngine::new(model);
let plan = engine.understand_task("创建配置加载器").await?;

// 使用自定义模板
let template = PromptTemplate::from_file("prompts/rust-project.yaml")?;
let engine = UnderstandingEngine::with_template(model, template);
```

**内置场景**: code_generation, refactoring, debugging, testing, documentation, architecture, optimization, file_operations, command_execution

详见：[提示词工程文档](doc/PROMPT_ENGINEERING.md)

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- 配置的AI模型API密钥（Zhipu GLM-4、OpenAI GPT-4等）

### 方式一：命令行工具

```bash
# 克隆项目
git clone https://github.com/lipish/agent-runner.git
cd agent-runner

# 配置API密钥
cp .env.example .env
# 编辑 .env 文件，添加你的API密钥

# 运行CLI
cargo run -- task "分析这个项目并创建摘要"
```

### 方式二：HTTP服务

```bash
# 启动HTTP服务
cargo run --bin agent-runner-server

# 在另一个终端测试
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"task": "Hello, Agent Runner!"}'
```

## 📋 使用方式

### 1. Rust API 集成

```rust
use agent_runner::{
    service::{TaskRunnerService, ServiceConfig, TaskRunnerClient, ApiClientBuilder},
    config::AgentConfig
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建服务实例
    let service = Arc::new(TaskRunnerService::new(
        ServiceConfig::default(),
        AgentConfig::load_with_fallback("config.toml")?
    ).await?);

    // 创建客户端
    let client = TaskRunnerClient::new(ApiClientBuilder::in_process(service));

    // 执行任务
    let response = client.execute_simple_task("创建一个Hello World程序").await?;
    println!("结果: {}", response.result.unwrap().summary);

    Ok(())
}
```

### 2. HTTP REST API

```bash
# 执行任务
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "task": "读取README.md文件并总结内容",
    "priority": "high"
  }'

# 批量执行任务
curl -X POST http://localhost:8080/api/v1/tasks/batch \
  -H "Content-Type: application/json" \
  -d '{
    "tasks": [
      {"task": "任务1"},
      {"task": "任务2"}
    ],
    "mode": "parallel"
  }'

# 获取任务状态
curl http://localhost:8080/api/v1/tasks/{task_id}

# 获取服务状态
curl http://localhost:8080/api/v1/status

# 获取指标
curl http://localhost:8080/api/v1/metrics
```

### 3. 远程配置管理 🔧

**Code Agent 支持通过 API 远程配置模型和 API key，无需重启服务！**

#### 获取当前配置
```bash
curl http://localhost:8080/api/v1/config
```

#### 动态更新模型配置
```bash
curl -X PUT http://localhost:8080/api/v1/config/model \
  -H "Content-Type: application/json" \
  -d '{
    "provider": "zhipu",
    "model_name": "glm-4",
    "api_key": "your-new-api-key",
    "max_tokens": 4000,
    "temperature": 0.7
  }'
```

#### 验证配置
```bash
curl -X POST http://localhost:8080/api/v1/config/validate \
  -H "Content-Type: application/json" \
  -d '{
    "config": {
      "model": {
        "provider": "zhipu",
        "model_name": "glm-4",
        "api_key": "test-key"
      }
    }
  }'
```

**配置管理特性:**
- ✅ **动态配置**: 无需重启服务即可更新模型和 API key
- ✅ **配置验证**: 提交前验证配置的正确性
- ✅ **错误处理**: 详细的错误信息和警告提示
- ✅ **安全性**: API key 等敏感信息的安全处理

### 4. 命令行工具

```bash
# 基本用法
cargo run -- task "你的任务描述"

# 交互模式
cargo run -- interactive

# 详细输出
cargo run -- task "任务" --output verbose

# JSON输出
cargo run -- task "任务" --output json
```

## 🔧 配置

### 基本配置 (config.toml)

```toml
[model]
provider = "zhipu"  # zhipu, openai, anthropic, local
model_name = "glm-4"
api_key = "your-api-key"
max_tokens = 4000
temperature = 0.7

[execution]
max_steps = 10
max_retries = 3
retry_delay_seconds = 1
timeout_seconds = 300

[tools]
enable_file_operations = true
enable_command_execution = true
working_directory = "."
allowed_paths = [".", "./src"]
forbidden_commands = ["rm -rf", "format", "fdisk"]

[service]
max_concurrent_tasks = 10
default_task_timeout = 300
enable_metrics = true
log_level = "info"

[service.cors]
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "DELETE"]
allowed_headers = ["*"]
allow_credentials = false

[service.rate_limiting]
requests_per_minute = 60
burst_size = 10
```

### 环境变量

```bash
# 服务配置
AGENT_RUNNER_MAX_CONCURRENT_TASKS=10
AGENT_RUNNER_DEFAULT_TASK_TIMEOUT=300
AGENT_RUNNER_ENABLE_METRICS=true
AGENT_RUNNER_LOG_LEVEL=info

# 服务器配置
BIND_ADDRESS=0.0.0.0:8080

# AI模型配置
AGENT_RUNNER_MODEL_PROVIDER=zhipu
AGENT_RUNNER_MODEL_NAME=glm-4
AGENT_RUNNER_API_KEY=your-api-key

# CORS配置
AGENT_RUNNER_CORS_ALLOWED_ORIGINS=*
```

## 📊 API 文档

### 核心 API 端点

| 端点 | 方法 | 描述 |
|------|------|------|
| `/health` | GET | 健康检查 |
| `/api/v1/status` | GET | 服务状态 |
| `/api/v1/metrics` | GET | 服务指标 |
| `/api/v1/tools` | GET | 可用工具 |
| `/api/v1/tasks` | POST | 执行任务 |
| `/api/v1/tasks/batch` | POST | 批量执行 |
| `/api/v1/tasks/{id}` | GET | 任务状态 |
| `/api/v1/tasks/{id}` | DELETE | 取消任务 |
| `/api/v1/config` | GET | 获取配置 |
| `/api/v1/config` | PUT | 更新配置 |
| `/api/v1/config/model` | PUT | 更新模型配置 |
| `/api/v1/config/validate` | POST | 验证配置 |

### 任务请求格式

```json
{
  "task": "任务描述",
  "task_id": "可选的自定义ID",
  "context": {
    "working_directory": "/path/to/dir",
    "environment": {"VAR": "value"},
    "tools": ["read_file", "write_file"],
    "constraints": {
      "max_execution_time": 300,
      "max_steps": 10,
      "allowed_paths": ["/safe/path"]
    }
  },
  "priority": "low|normal|high|critical",
  "metadata": {"key": "value"}
}
```

### 任务响应格式

```json
{
  "task_id": "uuid",
  "status": "completed",
  "result": {
    "success": true,
    "summary": "任务摘要",
    "details": "详细结果",
    "artifacts": [],
    "execution_time": 30
  },
  "plan": {
    "understanding": "AI对任务的理解",
    "approach": "AI的解决方法",
    "complexity": "simple|moderate|complex",
    "estimated_steps": 3,
    "requirements": ["tool1", "tool2"]
  },
  "steps": [...],
  "metrics": {...}
}
```

## 📈 监控和指标

### Prometheus 指标

服务在 `/metrics` 端点导出Prometheus指标：

- `ai_agent_requests_total` - API请求总数
- `ai_agent_request_duration_seconds` - 请求耗时分布
- `ai_agent_tasks_total` - 处理任务总数
- `ai_agent_tasks_completed_total` - 完成任务数
- `ai_agent_tasks_failed_total` - 失败任务数
- `ai_agent_active_tasks` - 当前活跃任务数
- `ai_agent_cpu_usage_percent` - CPU使用率
- `ai_agent_memory_usage_mb` - 内存使用量

## 🧪 测试

### 单元测试
```bash
cargo test
```

### 集成测试
```bash
cd examples
cargo run --example rust_client --features service
cargo run --example http_client --features service
cargo run --example in_process_service --features service
```

### 负载测试
```bash
# 安装hey
go install github.com/rakyll/hey@latest

# 负载测试
hey -n 1000 -c 50 \
  -H "Content-Type: application/json" \
  -d '{"task": "测试任务"}' \
  http://localhost:8080/api/v1/tasks
```

## 🏗️ 架构

### 系统架构图

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Rust Client   │    │  HTTP Client    │    │  Other Clients  │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────┴─────────────┐
                    │   Agent Runner Service    │
                    │  (Core Business Logic)  │
                    └─────────────┬─────────────┘
                                 │
          ┌──────────────────────┼──────────────────────┐
          │                      │                      │
    ┌─────┴─────┐        ┌──────┴───────┐        ┌────┴──────┐
    │  Models   │        │   Tools      │        │  Metrics  │
    │ (Zhipu,   │        │ (File Ops,   │        │(Prometheus│
    │ OpenAI,   │        │ Commands,    │        │  Export)  │
    │ etc.)     │        │ etc.)        │        │           │
    └───────────┘        └──────────────┘        └───────────┘
```

### 核心模块架构（优化后）

Agent Runner 采用**模块化、职责分离**的架构设计，确保代码清晰、可维护：

```
┌─────────────────────────────────────────────────────────┐
│                    agent_runner                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────────────┐      ┌──────────────────────┐   │
│  │   agent.rs       │─────▶│ understanding.rs     │   │
│  │  (345 行)        │      │  (185 行)            │   │
│  │                  │      │                      │   │
│  │ • 任务管理        │      │ • 提示词构建          │   │
│  │ • 执行协调        │      │ • AI 模型调用        │   │
│  │ • 工具调用        │      │ • 响应解析           │   │
│  │ • 结果生成        │      │ • 计划生成           │   │
│  └──────────────────┘      └──────────────────────┘   │
│         │                                              │
│         │                                              │
│         ▼                                              │
│  ┌──────────────────┐      ┌──────────────────────┐   │
│  │ execution.rs     │      │     tools.rs         │   │
│  │                  │      │                      │   │
│  │ • 执行引擎        │      │ • 工具注册           │   │
│  │ • 步骤管理        │      │ • 工具执行           │   │
│  │ • 上下文维护      │      │ • 安全检查           │   │
│  └──────────────────┘      └──────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**架构优化亮点**：
- ✅ **职责分离**: 理解逻辑从 agent.rs 分离到独立的 understanding.rs
- ✅ **代码精简**: agent.rs 从 416 行减少到 345 行（-17%）
- ✅ **共享模型**: 使用 `Arc<dyn LanguageModel>` 支持多组件共享
- ✅ **易于测试**: 每个模块都有独立的单元测试
- ✅ **可扩展性**: 清晰的模块边界便于功能扩展

### 任务执行工作流

Agent Runner 采用**模块化、AI 驱动**的执行模式，通过职责分离实现清晰的处理流程。

#### 完整执行流程

```
用户请求
   │
   ▼
┌─────────────────────────────────────────────────────────────┐
│  TaskAgent::process_task()                                  │
│  ─────────────────────────────────────────────────────      │
│  • 创建任务实例                                              │
│  • 设置任务状态为 InProgress                                 │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  阶段 1: 任务理解 (UnderstandingEngine)                      │
│  ─────────────────────────────────────────────────────      │
│  模块: understanding.rs                                      │
│                                                             │
│  1. build_understanding_prompt()                            │
│     • 构建结构化提示词                                       │
│     • 包含任务分析要求                                       │
│                                                             │
│  2. model.complete()                                        │
│     • 调用 AI 模型分析任务                                   │
│     • 获取 AI 响应                                          │
│                                                             │
│  3. parse_task_plan()                                       │
│     • 解析 AI 响应                                          │
│     • 提取关键信息：                                         │
│       - understanding: 对任务的理解                          │
│       - approach: 解决方法                                   │
│       - complexity: 复杂度 (Simple/Moderate/Complex)        │
│       - estimated_steps: 预估步骤数                         │
│       - requirements: 依赖和要求                            │
│                                                             │
│  返回: TaskPlan                                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  阶段 2: 任务执行 (TaskAgent)                                │
│  ─────────────────────────────────────────────────────      │
│  模块: agent.rs                                              │
│                                                             │
│  execute_task_real()                                        │
│     • 基于 TaskPlan 执行任务                                │
│     • 模式匹配识别任务类型                                   │
│     • 调用相应工具：                                         │
│       - read_file: 读取文件                                 │
│       - write_file: 写入文件                                │
│       - list_files: 列出文件                                │
│       - run_command: 执行命令                               │
│                                                             │
│  返回: ExecutionResult                                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  阶段 3: 结果生成                                            │
│  ─────────────────────────────────────────────────────      │
│  • 构建 TaskResult                                          │
│  • 包含执行摘要、详情、耗时                                  │
│  • 更新任务状态 (Completed/Failed)                          │
│  • 记录执行时间                                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
                    返回结果给用户
```

#### 模块协作关系

```
┌──────────────────┐
│   TaskAgent      │
│   (agent.rs)     │
└────────┬─────────┘
         │
         │ 1. 调用理解引擎
         ▼
┌──────────────────────┐
│ UnderstandingEngine  │
│ (understanding.rs)   │
└────────┬─────────────┘
         │
         │ 2. 调用 AI 模型
         ▼
┌──────────────────────┐
│   LanguageModel      │
│   (models.rs)        │
└────────┬─────────────┘
         │
         │ 3. 返回 TaskPlan
         ▼
┌──────────────────┐
│   TaskAgent      │
│   执行任务        │
└────────┬─────────┘
         │
         │ 4. 调用工具
         ▼
┌──────────────────────┐
│   ToolRegistry       │
│   (tools.rs)         │
└──────────────────────┘
```

#### 核心特点

- ✅ **职责分离**: 理解、执行、工具调用各司其职
- ✅ **AI 驱动**: 使用 AI 模型理解任务意图
- ✅ **模块化设计**: 每个模块独立可测试
- ✅ **共享资源**: 通过 Arc 共享 AI 模型实例
- ✅ **类型安全**: 使用 Rust 类型系统保证正确性

## 🔒 安全性

### 认证和授权
- API密钥认证
- 请求速率限制
- CORS配置
- 权限控制

### 执行安全
- 沙箱文件访问
- 危险命令过滤
- 超时保护
- 资源限制

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 📚 文档

- [API文档](doc/SERVICE_API.md) - 详细的API参考
- [系统设计](doc/system-design.md) - 架构设计文档
- [部署指南](doc/DEPLOYMENT.md) - 生产部署指南
- [示例代码](examples/README.md) - 完整使用示例

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🔗 相关链接

- [GitHub仓库](https://github.com/lipish/agent-runner)
- [Docker Hub](https://hub.docker.com/r/agent-runner/service)
- [API文档](doc/SERVICE_API.md)

---

**Agent Runner** - 简单高效的 AI 驱动智能代理执行服务。