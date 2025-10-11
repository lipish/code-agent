# Agent工作流（通用化与配置驱动）

## 目的
统一说明“单一通用 Agent + 配置驱动”下的工作流：理解 → 规划 → 执行 → 结果；覆盖模板、工具、安全三大要素。

## 核心组件
- `TaskAgent`（`src/agent/mod.rs`）：编排模型、规划引擎、执行器与工具注册。
- `PlanningEngine`（`src/planning/engine.rs`）：基于模板与场景进行任务分析（`analyze_task`）。
- `PromptTemplate`（`src/prompts/mod.rs` + `prompts/*.yaml`）：全局/项目/场景分层提示词，外置配置。
- `TaskExecutor`（`src/agent/executor.rs`）：根据理解结果委派执行，调用工具函数或注册工具。
- `ToolRegistry`（`src/tools.rs`）：线程安全工具注册与执行（读写文件、列目录、运行命令等）。
- 安全校验（`src/security.rs`）：命令与路径验证（白/黑名单、路径遍历防护）。

## 配置驱动
- 全局：系统角色、输出格式、通用约束（`global.*`）。
- 项目：技术栈、编码规范、上下文与架构（`project.*`）。
- 场景：任务类型的特定指令与输出结构（`scenarios.*`）。

## 端到端流程
1. 输入自然语言任务到 `TaskAgent`。
2. `PlanningEngine.analyze_task`：根据模板与场景推断任务类型，生成理解与方案。
3. `TaskExecutor`：解析计划，确定是否需要工具；构造 `ToolCall` 并通过 `ToolRegistry` 执行。
4. 安全校验：`CommandValidator`/`PathValidator` 在工具执行前防止危险操作。
5. 汇总结果：返回结构化的 `TaskResult` 或具体输出。

## 示例（简化）
```rust
let template = PromptTemplate::from_file("prompts/optimized-template.yaml")?;
let engine = PlanningEngine::with_template(Arc::new(MockModel::new("gpt-4".into())), template);
let plan = engine.analyze_task("读取 Cargo.toml 并打印摘要").await?;

let registry = ToolRegistry::new();
registry.register(ReadFileTool).await;
let call = ToolCall { name: "read_file".into(), args: ToolArgs::from_kv("path", "Cargo.toml") };
let result = registry.execute(&call).await?;
```

## 设计原则
- 统一的通用 Agent，行为外置到配置（减少固定类型）。
- 模块清晰、职责分离（规划/执行/工具/安全）。
- 保持可扩展性（模板与工具可插拔）。