# 可执行任务规划系统

## 问题背景

在原始的 `TaskPlan` 中，`approach` 字段是一个非结构化的字符串，这使得它难以被程序化执行。

**原始 approach 的问题**：
- 非结构化，难以程序化执行
- 缺乏详细的执行参数
- 没有依赖关系信息
- 缺乏验证和回滚机制

## 解决方案

我们通过扩展现有的 `TaskPlan` 结构，添加了可选的结构化执行步骤，避免了类型重复和DRY原则违反。

### 改进后的 TaskPlan 结构

```rust
pub struct TaskPlan {
    pub understanding: String,
    pub approach: String,                           // 保持向后兼容
    pub complexity: TaskComplexity,
    pub estimated_steps: Option<u32>,
    pub requirements: Vec<String>,
    
    // ✅ 新增：可选的结构化执行步骤
    pub structured_steps: Option<Vec<StructuredExecutionStep>>,
    // ✅ 新增：可选的步骤依赖关系
    pub step_dependencies: Option<Vec<StepDependency>>,
}
```

这种设计的优势：
- **向后兼容**：现有代码继续工作
- **避免重复**：没有创建单独的 `ExecutableTaskPlan` 类型
- **渐进式增强**：可以选择性地添加结构化步骤
- **类型统一**：所有任务计划使用同一个类型

### 2. 执行步骤 (`ExecutionStep`)

每个步骤包含详细的执行信息：

```rust
pub struct ExecutionStep {
    pub id: String,                          // 唯一标识
    pub name: String,                        // 步骤名称
    pub description: String,                 // 步骤描述
    pub step_type: ExecutionStepType,        // ✅ 步骤类型
    pub estimated_duration: Option<u32>,     // 预估时间
    pub preconditions: Vec<String>,          // ✅ 前置条件
    pub expected_outputs: Vec<String>,       // ✅ 预期输出
    pub validation_criteria: Vec<String>,    // ✅ 验证标准
    pub rollback_actions: Vec<String>,       // ✅ 回滚操作
}
```

### 3. 步骤类型 (`ExecutionStepType`)

支持多种类型的操作：

- **文件操作** (`FileOperation`): 创建、读取、更新、删除文件
- **命令执行** (`CommandExecution`): 运行系统命令
- **代码生成** (`CodeGeneration`): 生成各种语言的代码
- **数据分析** (`DataAnalysis`): 数据处理和分析
- **系统配置** (`SystemConfiguration`): 系统设置和配置
- **测试执行** (`TestExecution`): 运行测试用例
- **工具调用** (`ToolInvocation`): 调用自定义工具
- **人工确认** (`ManualConfirmation`): 需要人工确认的步骤

### 4. 依赖关系管理

支持多种依赖类型：
- **严格依赖** (`StrictDependency`): 前置步骤必须成功
- **弱依赖** (`WeakDependency`): 前置步骤完成即可
- **条件依赖** (`ConditionalDependency`): 基于条件的依赖
- **数据依赖** (`DataDependency`): 需要前置步骤的输出

## 使用方式

### 1. 增强现有的 TaskPlan

```rust
use task_runner::planning::ApproachParser;

let parser = ApproachParser::new();
let enhanced_plan = parser.enhance_task_plan(&task_plan);
```

### 2. 检查是否有结构化步骤

```rust
if enhanced_plan.has_structured_steps() {
    // 使用结构化步骤
    let next_steps = enhanced_plan.get_next_executable_steps(&completed_steps);
} else {
    // 使用传统 approach 字符串
    println!("传统方法: {}", enhanced_plan.approach);
}
```

### 3. 执行流程管理

```rust
// 获取可执行的下一个步骤
let next_steps = enhanced_plan.get_next_executable_steps(&completed_steps);

// 验证计划完整性
enhanced_plan.validate_structured_plan()?;

// 估算总执行时间
let total_duration = enhanced_plan.estimate_total_duration();

// 生成执行摘要
let summary = enhanced_plan.generate_execution_summary();
```

## 演示效果

通过运行测试可以看到转换效果：

```bash
cargo test test_approach_to_executable_steps_demo -- --nocapture
```

**转换前** (原始 approach 字符串):
```
"1. Set up a Node.js project with Express and MongoDB. 2. Design RESTful endpoints for CRUD operations on to-do items. 3. Implement user authentication using JWT."
```

**转换后** (结构化步骤):
```
🔸 步骤 1 [ID: step_5fa77d39]
   名称: 步骤 1
   描述: Set up a Node.js project with Express and MongoDB...
   类型: 代码生成 (javascript)
   预计耗时: 30 分钟
   验证标准:
     - Step completed successfully
```

## 优势对比

| 特性 | 原始 approach (字符串) | ExecutableTaskPlan |
|------|----------------------|-------------------|
| 结构化 | ❌ 非结构化 | ✅ 完全结构化 |
| 程序化执行 | ❌ 无法执行 | ✅ 可程序化执行 |
| 执行参数 | ❌ 缺乏详细参数 | ✅ 详细的执行参数 |
| 依赖关系 | ❌ 无依赖信息 | ✅ 明确的依赖关系 |
| 验证机制 | ❌ 无验证标准 | ✅ 验证标准和回滚 |
| 进度跟踪 | ❌ 无法跟踪 | ✅ 详细的进度监控 |
| 错误处理 | ❌ 无错误处理 | ✅ 完善的错误处理 |

## 智能解析功能

`ApproachParser` 具备以下智能解析能力：

1. **自动步骤提取**: 识别编号列表、项目符号等格式
2. **步骤类型推断**: 根据关键词自动推断操作类型
3. **前置条件识别**: 提取步骤间的依赖关系
4. **验证标准生成**: 自动生成验证标准
5. **时间估算**: 基于任务复杂度估算执行时间

## 扩展性

系统设计具有良好的扩展性：

1. **新增步骤类型**: 通过扩展 `ExecutionStepType` 枚举
2. **自定义解析规则**: 通过修改 `ApproachParser` 的模式匹配
3. **执行引擎集成**: 可以轻松集成到现有的执行引擎中
4. **监控和日志**: 支持详细的执行监控和日志记录

这个系统将非结构化的任务描述转换为完全可执行的结构化计划，为AI驱动的任务自动化提供了强大的基础设施。