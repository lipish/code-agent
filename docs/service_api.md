# 服务 API（HTTP 接口）

## 概述
Axum 暴露 REST 端点用于提交任务、查询状态与获取指标。核心服务在 `src/service/{core.rs, api.rs, metrics_simple.rs}`，对外由 `src/server/main.rs` 启动。

## 端点
- `GET /health`：健康检查
- `GET /api/v1/status`：服务状态
- `GET /api/v1/metrics`：指标快照
- `GET /api/v1/tools`：可用工具列表
- `POST /api/v1/tasks`：执行单个任务
- `POST /api/v1/tasks/batch`：批量任务执行
- `GET /api/v1/tasks/:id`：查询任务状态与结果
- `DELETE /api/v1/tasks/:id`：取消任务

## 请求示例
```bash
curl -X POST http://localhost:8000/api/v1/tasks \
  -H 'Content-Type: application/json' \
  -d '{"task":{"request":"读取 Cargo.toml 并输出摘要"}}'
```

## 说明
- 工具执行通过 `ToolRegistry`，并由 `security.rs` 做命令/路径校验。
- 数据结构定义见 `src/service/types/`。
- 详细工作流参见 `doc/agent-workflow.md`。