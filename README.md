# Task Runner

一个简单高效的 AI 驱动任务运行服务，提供 Rust API 和 HTTP REST 接口，可集成到任何应用中。

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

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- 配置的AI模型API密钥（Zhipu GLM-4、OpenAI GPT-4等）

### 方式一：命令行工具

```bash
# 克隆项目
git clone https://github.com/lipish/task-runner.git
cd task-runner

# 配置API密钥
cp .env.example .env
# 编辑 .env 文件，添加你的API密钥

# 运行CLI
cargo run -- task "分析这个项目并创建摘要"
```

### 方式二：HTTP服务

```bash
# 启动HTTP服务
cargo run --bin task-runner-server

# 在另一个终端测试
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"task": "Hello, Task Runner!"}'
```


## 📋 使用方式

### 1. Rust API 集成

```rust
use task_runner::{
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

### 3. 命令行工具

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
TASK_RUNNER_MAX_CONCURRENT_TASKS=10
TASK_RUNNER_DEFAULT_TASK_TIMEOUT=300
TASK_RUNNER_ENABLE_METRICS=true
TASK_RUNNER_LOG_LEVEL=info

# 服务器配置
BIND_ADDRESS=0.0.0.0:8080

# AI模型配置
TASK_RUNNER_MODEL_PROVIDER=zhipu
TASK_RUNNER_MODEL_NAME=glm-4
TASK_RUNNER_API_KEY=your-api-key

# CORS配置
TASK_RUNNER_CORS_ALLOWED_ORIGINS=*
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
          ┌──────────────────────┼──────────────────────┘
          │                      │                      │
    ┌─────┴─────┐        ┌──────┴───────┐        ┌──────┴─────┐
    │  Models   │        │   Tools     │        │  Metrics   │
    │ (Zhipu,   │        │ (File Ops,  │        │ (Prometheus│
    │ OpenAI,   │        │ Commands,  │        │  Export)   │
    │ etc.)     │        │ etc.)       │        │            │
    └───────────┘        └─────────────┘        └────────────┘
```

### 任务执行模式：单次计划生成

Task Runner 采用**"一次性计划生成"**的执行模式，这是一种简单高效的任务处理方式。

#### 执行流程

```
┌─────────────────────────────────────────────────────────────┐
│                     任务请求                                 │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  阶段 1: 理解阶段 (understand_task)                          │
│  ─────────────────────────────────────────────────────      │
│  • AI 分析任务需求                                           │
│  • 生成 TaskPlan:                                           │
│    - understanding: 对任务的理解                             │
│    - approach: 解决方法                                      │
│    - complexity: 复杂度评估 (Simple/Moderate/Complex)       │
│    - estimated_steps: 预估步骤数                            │
│    - requirements: 依赖和要求                               │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  阶段 2: 执行阶段 (execute_task_real)                        │
│  ─────────────────────────────────────────────────────      │
│  • 基于生成的计划执行任务                                    │
│  • 模式匹配识别任务类型:                                     │
│    - 文件读取 (read file)                                   │
│    - 文件列表 (list files)                                  │
│    - 命令执行 (run command)                                 │
│  • 执行相应操作                                             │
│  • 计划在执行过程中不会动态调整                              │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  阶段 3: 结果生成                                            │
│  ─────────────────────────────────────────────────────      │
│  • 返回 ExecutionResult                                     │
│  • 包含执行摘要、详情、耗时等信息                            │
└─────────────────────────────────────────────────────────────┘
```

#### 核心特点

- **计划一次生成**: 在理解阶段生成完整计划，执行过程中不再调整
- **简单高效**: 流程清晰，适合明确的单一任务
- **模式匹配执行**: 通过识别任务类型选择相应的执行策略

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

- [GitHub仓库](https://github.com/lipish/task-runner)
- [Docker Hub](https://hub.docker.com/r/task-runner/service)
- [API文档](doc/SERVICE_API.md)

---

**Task Runner** - 简单高效的 AI 驱动任务运行服务。