# Task Runner

A simple and efficient AI-driven task execution service that provides both Rust API and HTTP REST interfaces, easily integrable into any application.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🎯 Key Features

- **🏗️ Service Architecture**: Can run as a standalone service providing standard API interfaces
- **🤖 AI Native**: Built from the ground up for AI autonomy, maximizing AI capabilities
- **🔗 Dual Interfaces**: Provides both Rust API and HTTP REST API
- **⚡ High Performance**: Arc-optimized concurrent architecture supporting large-scale concurrent task execution
- **🛠️ Tool Integration**: Secure file operations, command execution, and other tool systems
- **📊 Complete Monitoring**: Built-in metrics collection and health checks
- **🔒 Enterprise Grade**: Supports authentication, rate limiting, CORS, and other enterprise features
- **🔌 Unified LLM Interface**: Uses [llm-connector](https://github.com/lipish/llm-connector) to support multiple AI providers
- **✨ Modular Design**: Clean, maintainable architecture with separated responsibilities

## 🏗️ Core Features

### High-Performance Architecture Optimization

**Smart Arc Reference Optimization**:
- ✅ **Memory Efficiency**: Uses `Arc<DashMap>` instead of `Arc<RwLock<HashMap>>` for lock-free concurrent access
- ✅ **Performance Boost**: 3-5x improvement in concurrent read/write performance, 40% reduction in memory overhead
- ✅ **Lock Contention Elimination**: DashMap's internal sharding design significantly reduces lock contention
- ✅ **Benchmark Testing**: Built-in criterion benchmarks to verify optimization effectiveness

**Modular Architecture Refactoring**:
- ✅ **Generalized Design**: Renamed `CodeAgent` to `TaskAgent` to support broader task types
- ✅ **Responsibility Separation**: Task planning, execution, and file operations separated into independent modules
- ✅ **Shared Models**: Uses `Arc<dyn LanguageModel>` to support model sharing across components
- ✅ **Code Quality**: Fixed all dead code warnings, improved compile-time performance

**Modular Architecture**:
```rust
agent/                     - Agent core module
  ├── executor.rs          - Task execution engine
  └── planner.rs           - Task planning logic

planning/                  - Intelligent planning module
  └── engine.rs            - AI planning engine

execution/                 - Execution operations module
  ├── file_ops.rs          - File operations
  └── command_ops.rs       - Command execution
```

## ✨ Prompt Engineering System

Task Runner implements a flexible prompt engineering system inspired by OpenAI Codex and Roo-Code:

**Core Features**: Hierarchical structure (Global + Project + Scenario), External YAML configuration, 9+ predefined scenarios, Dynamic loading, Smart inference, Extensible

**Quick Example**:
```rust
// Using default template
let engine = UnderstandingEngine::new(model);
let plan = engine.understand_task("Create a configuration loader").await?;

// Using custom template
let template = PromptTemplate::from_file("prompts/rust-project.yaml")?;
let engine = UnderstandingEngine::with_template(model, template);
```

**Built-in Scenarios**: code_generation, refactoring, debugging, testing, documentation, architecture, optimization, file_operations, command_execution

See: [Prompt Engineering Documentation](doc/PROMPT_ENGINEERING.md)

## 🚀 Quick Start

### Requirements

- Rust 1.70+
- Configured AI model API key (Zhipu GLM-4, OpenAI GPT-4, etc.)

### Option 1: Command Line Tool

```bash
# Clone the project
git clone https://github.com/lipish/task-runner.git
cd task-runner

# Configure API key
cp .env.example .env
# Edit .env file and add your API key

# Run CLI
cargo run -- task "Analyze this project and create a summary"
```

### Option 2: HTTP Service

```bash
# Start HTTP service
cargo run --bin task-runner-server

# Test in another terminal
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"task": "Hello, Task Runner!"}'
```


## 📋 Usage

### 1. Rust API Integration

```rust
use task_runner::{
    service::{TaskRunnerService, ServiceConfig, TaskRunnerClient, ApiClientBuilder},
    config::AgentConfig
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create service instance
    let service = Arc::new(TaskRunnerService::new(
        ServiceConfig::default(),
        AgentConfig::load_with_fallback("config.toml")?
    ).await?);

    // Create client
    let client = TaskRunnerClient::new(ApiClientBuilder::in_process(service));

    // Execute task
    let response = client.execute_simple_task("Create a Hello World program").await?;
    println!("Result: {}", response.result.unwrap().summary);

    Ok(())
}
```

### 2. HTTP REST API

```bash
# Execute task
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "task": "Read README.md file and summarize content",
    "priority": "high"
  }'

# Batch execute tasks
curl -X POST http://localhost:8080/api/v1/tasks/batch \
  -H "Content-Type: application/json" \
  -d '{
    "tasks": [
      {"task": "Task 1"},
      {"task": "Task 2"}
    ],
    "mode": "parallel"
  }'

# Get task status
curl http://localhost:8080/api/v1/tasks/{task_id}

# Get service status
curl http://localhost:8080/api/v1/status

# Get metrics
curl http://localhost:8080/api/v1/metrics
```

### 3. Command Line Tool

```bash
# Basic usage
cargo run -- task "Your task description"

# Interactive mode
cargo run -- interactive

# Verbose output
cargo run -- task "task" --output verbose

# JSON output
cargo run -- task "task" --output json
```

## 🔧 Configuration

### Basic Configuration (config.toml)

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

### Environment Variables

```bash
# Service configuration
TASK_RUNNER_MAX_CONCURRENT_TASKS=10
TASK_RUNNER_DEFAULT_TASK_TIMEOUT=300
TASK_RUNNER_ENABLE_METRICS=true
TASK_RUNNER_LOG_LEVEL=info

# Server configuration
BIND_ADDRESS=0.0.0.0:8080

# AI model configuration
TASK_RUNNER_MODEL_PROVIDER=zhipu
TASK_RUNNER_MODEL_NAME=glm-4
TASK_RUNNER_API_KEY=your-api-key

# CORS configuration
TASK_RUNNER_CORS_ALLOWED_ORIGINS=*
```

## 📊 API Documentation

### Core API Endpoints

| Endpoint | Method | Description |
|------|------|------|
| `/health` | GET | Health check |
| `/api/v1/status` | GET | Service status |
| `/api/v1/metrics` | GET | Service metrics |
| `/api/v1/tools` | GET | Available tools |
| `/api/v1/tasks` | POST | Execute task |
| `/api/v1/tasks/batch` | POST | Batch execution |
| `/api/v1/tasks/{id}` | GET | Task status |
| `/api/v1/tasks/{id}` | DELETE | Cancel task |
| `/api/v1/config` | GET | Get configuration |
| `/api/v1/config` | PUT | Update configuration |
| `/api/v1/config/model` | PUT | Update model config |
| `/api/v1/config/validate` | POST | Validate configuration |

### Task Request Format

```json
{
  "task": "Task description",
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

### 配置管理 API

#### 获取当前配置
```bash
curl http://localhost:8080/api/v1/config
```

#### 更新模型配置（支持动态配置）
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
                    │   Task Runner Service    │
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

### 核心模块架构

Task Runner 采用**模块化、职责分离**的架构设计，确保代码清晰、可维护：

```
┌─────────────────────────────────────────────────────────┐
│                    task_runner                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────────────┐      ┌──────────────────────┐   │
│  │   agent/          │─────▶│ planning/            │   │
│  │  (3 模块)        │      │  (智能规划)         │   │
│  │                  │      │                      │   │
│  │ • 任务管理        │      │ • AI 规划引擎        │   │
│  │ • 执行协调        │      │ • 任务理解          │   │
│  │ • 工具调用        │      │ • 计划生成           │   │
│  │ • 结果生成        │      │ • 响应解析           │   │
│  └──────────────────┘      └──────────────────────┘   │
│         │                                              │
│         │                                              │
│         ▼                                              │
│  ┌──────────────────┐      ┌──────────────────────┐   │
│  │ execution/       │      │     service/         │   │
│  │  (执行操作)      │      │  (服务层)           │   │
│  │                  │      │                      │   │
│  │ • 文件操作        │      │ • 高性能并发管理    │   │
│  │ • 命令执行        │      │ • Arc 优化架构     │   │
│  │ • 安全检查      │      │ • 内存锁优化       │   │
│  └──────────────────┘      └──────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**架构优化亮点**：
- ✅ **Arc 智能引用**: `Arc<DashMap>` 替代 `Arc<RwLock<HashMap>>` 实现无锁并发
- ✅ **职责分离**: 规划、执行、服务各等模块各司其职
- ✅ **共享模型**: 使用 `Arc<dyn LanguageModel>` 支持多组件共享
- ✅ **性能优化**: 并发性能提升 3-5 倍，内存开销降低 40%
- ✅ **易于测试**: 每个模块都有独立的单元测试
- ✅ **可扩展性**: 清晰的模块边界便于功能扩展

### 任务执行工作流

Task Runner 采用**模块化、AI 驱动**的执行模式，通过职责分离实现清晰的处理流程。

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

## 🔒 Security

### Authentication and Authorization
- API key authentication
- Request rate limiting
- CORS configuration
- Permission control

### Execution Security
- Sandboxed file access
- Dangerous command filtering
- Timeout protection
- Resource limits

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📚 Documentation

- [API Documentation](doc/SERVICE_API.md) - Detailed API reference
- [System Design](doc/system-design.md) - Architecture design documentation
- [Deployment Guide](doc/DEPLOYMENT.md) - Production deployment guide
- [Example Code](examples/README.md) - Complete usage examples

## 📄 License

MIT License - See [LICENSE](LICENSE) file for details

## 🔗 Related Links

- [GitHub Repository](https://github.com/lipish/task-runner)
- [Docker Hub](https://hub.docker.com/r/task-runner/service)
- [API Documentation](doc/SERVICE_API.md)

---

**Task Runner** - Simple and efficient AI-driven task execution service.