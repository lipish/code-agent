# Agent Runner Documentation

This directory contains detailed documentation and test reports for the Agent Runner project.

## Core Documentation

### Architecture and Design

- **[SEQUENTIAL_EXECUTION_DESIGN.md](SEQUENTIAL_EXECUTION_DESIGN.md)** - 顺序执行系统详细设计
  - 基于 OpenAI Codex 的设计理念
  - Understanding → Approach → Plan → Execution 的完整流程
  - 每个阶段的验证和纠错机制
  - 类型系统和API设计

- **[SEQUENTIAL_EXECUTION_SUMMARY.md](SEQUENTIAL_EXECUTION_SUMMARY.md)** - 实现总结
  - 已完成功能概览
  - 核心组件说明
  - 使用示例和测试结果
  - 下一步实施计划

- **[SEQUENTIAL_EXECUTION_LLM_INTEGRATION.md](SEQUENTIAL_EXECUTION_LLM_INTEGRATION.md)** - LLM 集成实现
  - Phase 2 完成：真实 LLM 调用
  - Understanding/Approach/Planning 阶段
  - 重试机制和错误处理
  - 多格式响应解析

- **[SEQUENTIAL_EXECUTION_ENHANCED.md](SEQUENTIAL_EXECUTION_ENHANCED.md)** - 增强实现文档
  - Phase 4 完成：增强步骤解析和真实执行
  - 详细步骤信息解析（NAME/DESCRIPTION/TYPE/DURATION等）
  - 真实文件操作（create/read/modify）
  - 真实命令执行与安全检查
  - LLM 驱动的代码生成
  - 测试集成和配置管理

- **[EXECUTION_GUARDRAILS_DESIGN.md](EXECUTION_GUARDRAILS_DESIGN.md)** - 执行保护机制详细设计
  - 多层次风险评估系统
  - 危险模式检测机制
  - 用户确认和边界保护
  - 回滚计划和快照管理

- **[GUARDRAILS_IMPLEMENTATION_SUMMARY.md](GUARDRAILS_IMPLEMENTATION_SUMMARY.md)** - 保护机制实现总结
  - 核心功能实现说明
  - 测试结果和验证
  - 使用指南和最佳实践
  - 下一步计划

- **[LLM_CONNECTOR_UPGRADE.md](LLM_CONNECTOR_UPGRADE.md)** - llm-connector 0.3.8 升级文档
  - 版本升级说明（0.3.1 → 0.3.8）
  - 新功能：模型发现、Ollama 管理、流式增强
  - API 变更：Zhipu/Aliyun 专用构造函数
  - 兼容性和迁移指南

- **[EXECUTABLE_PLANNING.md](EXECUTABLE_PLANNING.md)** - 可执行计划设计
  - TaskPlan 的结构化步骤扩展
  - 步骤依赖关系管理
  - 执行进度跟踪

## Test Reports

### LongCat Model Testing

- **[LONGCAT_TEST_REPORT.md](LONGCAT_TEST_REPORT.md)** - LongCat模型综合测试报告
  - 4个测试场景的完整分析
  - 每个场景的完整原始响应（13,488字符）
  - Token使用统计和质量分析
  - 模型优势和改进建议

- **[longcat_raw_responses.md](longcat_raw_responses.md)** - 原始响应存档
  - 未经处理的完整LLM输出
  - 详细的响应统计

## Quick Start

### 1. 运行顺序执行演示

```bash
cargo run --example sequential_execution_demo
```

### 2. 运行 LLM 集成演示（完整功能）

```bash
# 使用 Mock 模型（快速测试）
cargo run --example sequential_llm_demo

# 使用真实 LLM（完整体验）
export OPENAI_API_KEY="sk-..."
# 或
export DEEPSEEK_API_KEY="sk-..."
# 或
export LONGCAT_API_KEY="sk-..."
cargo run --example sequential_llm_demo
```

演示功能：
- ✅ Phase 1-3: 真实 LLM 理解、方案设计、计划制定
- ✅ Phase 4: 增强步骤解析（支持详细步骤信息）
- ✅ Phase 4: 真实执行（文件操作、命令执行、代码生成）
- ✅ 保护机制集成（风险评估、确认请求）

### 3. 运行执行保护机制演示

```bash
cargo run --example guardrails_demo
```

### 4. 测试 LongCat 模型

```bash
# 设置 API key
export LONGCAT_API_KEY="your-api-key-here"

# 运行详细测试
cargo run --example test_longcat_detailed

# 捕获原始响应
cargo run --example capture_longcat_raw
```

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     User Request                            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              Sequential Executor + Guardrails               │
│  ┌────────────────────────────────────────────────────┐    │
│  │  Phase 1: Understanding                            │    │
│  │    • Parse task requirements                       │    │
│  │    • Identify complexity                           │    │
│  │    • Validate understanding (confidence: 0.9)      │    │
│  └────────────────────┬───────────────────────────────┘    │
│                       ▼                                      │
│  ┌────────────────────────────────────────────────────┐    │
│  │  Phase 2: Approach                                 │    │
│  │    • Design solution approach                      │    │
│  │    • Select tech stack                             │    │
│  │    • Validate approach (confidence: 0.85)          │    │
│  └────────────────────┬───────────────────────────────┘    │
│                       ▼                                      │
│  ┌────────────────────────────────────────────────────┐    │
│  │  Phase 3: Planning                                 │    │
│  │    • Create detailed execution plan                │    │
│  │    • Define step dependencies                      │    │
│  │    • Validate plan (confidence: 0.8)               │    │
│  └────────────────────┬───────────────────────────────┘    │
│                       ▼                                      │
│  ┌────────────────────────────────────────────────────┐    │
│  │  Phase 4: Execution (Sequential Steps)             │    │
│  │    ┌──────────────────────────────────────┐        │    │
│  │    │ Guardrail Engine (Safety Check)      │        │    │
│  │    │  • Risk assessment (5 levels)        │        │    │
│  │    │  • Dangerous pattern detection        │        │    │
│  │    │  • Protected path check               │        │    │
│  │    │  • Batch operation threshold          │        │    │
│  │    │  • User confirmation (if needed)      │        │    │
│  │    └──────────────────────────────────────┘        │    │
│  │    Step 1 → Validate → Step 2 → Validate → ...    │    │
│  │    • Check dependencies                            │    │
│  │    • Execute with retry                            │    │
│  │    • Create snapshot (if needed)                   │    │
│  │    • Rollback on failure                           │    │
│  └────────────────────┬───────────────────────────────┘    │
│                       ▼                                      │
│  ┌────────────────────────────────────────────────────┐    │
│  │  Phase 5: Validation                               │    │
│  │    • Verify all outputs                            │    │
│  │    • Generate final report                         │    │
│  │    • Overall score: 0.9                            │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              Execution Result                               │
│  • Complete execution history                               │
│  • Performance metrics                                      │
│  • Confidence scores for each phase                         │
│  • Safety checks and confirmations                          │
│  • Rollback plans (if applicable)                           │
│  • Detailed diagnostics                                     │
└─────────────────────────────────────────────────────────────┘
```

## Key Features

### ✅ Phased Execution
- Understanding → Approach → Planning → Execution → Validation
- Each phase is independently validated
- Support for human confirmation at critical points

### ✅ Safety Guardrails
- 5-level risk assessment (Safe → Low → Medium → High → Critical)
- Dangerous pattern detection (rm -rf, sudo, etc.)
- Protected path enforcement (.env, .git, etc.)
- Batch operation thresholds
- Automatic user confirmation for risky operations
- Rollback plans with snapshot support

### ✅ Error Correction
- Automatic retry with configurable attempts
- Confidence-based quality gates
- Intelligent rollback on failures
- Detailed error diagnostics

### ✅ Observability
- Complete execution history
- Confidence scoring for each phase
- Performance metrics and timing
- Comprehensive logging

### ✅ Flexibility
- Configurable behavior (retries, thresholds, etc.)
- Support for human intervention
- Partial execution resumption (planned)
- Conditional branching (planned)

## Configuration Options

```rust
pub struct ExecutionConfig {
    pub max_retries_per_phase: u32,      // Default: 3
    pub require_confirmation: bool,       // Default: false
    pub min_confidence_threshold: f32,    // Default: 0.7
    pub enable_auto_rollback: bool,       // Default: true
    pub verbose_logging: bool,            // Default: false
}
```

## LongCat Model Integration

### Test Results Summary

| Metric | Result |
|--------|--------|
| **Overall Rating** | ⭐⭐⭐⭐⭐ (5/5) |
| **Understanding Accuracy** | 5/5 |
| **Technical Quality** | 5/5 |
| **Actionability** | 5/5 |
| **Response Completeness** | 5/5 |
| **Domain Expertise** | 5/5 |

**Key Findings**:
- ✅ Accurate understanding of complex business requirements
- ✅ Professional technical solutions and architecture
- ✅ Executable code and detailed steps
- ✅ Deep domain knowledge demonstration
- ⚠️ Parser compatibility issue (uses `**FIELD**` format without colons)

### Recommended Usage

LongCat-Flash-Chat excels at:
- Complex system architecture design
- Technical solution evaluation
- Project planning and estimation
- Code generation and implementation guidance

## Implementation Status

### Phase 1: ✅ Completed
- Core type definitions
- Sequential execution framework
- Basic phase implementations
- Error handling
- Demo application

### Phase 2: ✅ Completed
- **LLM Integration** (Understanding, Approach, Planning)
- **Response Parsing** (supports multiple formats)
- **Retry Logic** with exponential backoff
- **Validation System** with confidence scoring
- **Real LLM Demo** with multiple provider support

### Phase 3: 🚧 In Progress
- Step execution engine with guardrails
- Safety checks before execution
- Snapshot and rollback implementation

### Phase 4: 📋 Planned
- Enhanced parsing for decisions and alternatives
- Dependency resolution
- Milestone tracking
- Streaming support
- Human intervention points
- Confidence assessment

### Phase 4: 📋 Planned
- Advanced features (conditional branching, parallel execution)
- Monitoring and diagnostics
- Performance optimization
- Complete test coverage

## Related Files

### Source Code
- `/src/execution/sequential.rs` - Sequential execution system
- `/src/planning/engine.rs` - Planning engine (needs parser update)
- `/src/models.rs` - LLM model abstraction
- `/src/types.rs` - Core type definitions

### Examples
- `/examples/sequential_execution_demo.rs` - Basic usage demo
- `/examples/test_longcat_detailed.rs` - LongCat testing
- `/examples/capture_longcat_raw.rs` - Response capture

### Tests
- `/tests/test_license_management_decomposition.rs`
- `/tests/test_meeting_room_booking_decomposition.rs`
- `/tests/test_portfolio_management_decomposition.rs`

## Next Steps

1. **LLM Integration** (High Priority)
   - Implement actual LLM calls in each phase
   - Add response parsing for multiple formats
   - Support LongCat's markdown format

2. **Execution Engine** (High Priority)
   - Implement step dependency checking
   - Add different step type executors
   - Implement rollback mechanism

3. **Validation System** (Medium Priority)
   - Add validation logic for each phase
   - Implement confidence scoring
   - Add quality gates

4. **State Management** (Medium Priority)
   - Add state persistence
   - Support checkpoint/resume
   - Implement state recovery

5. **Advanced Features** (Low Priority)
   - Conditional branching
   - Parallel execution
   - Dynamic plan adjustment
   - Automated diagnostics

## Contributing

When contributing to this project, please:
1. Follow the phased execution design principles
2. Add validation for all new phases/steps
3. Include confidence scoring
4. Provide detailed error messages
5. Update documentation

## References

- [OpenAI Codex](https://github.com/openai/codex) - Inspiration for phased execution
- [LongCat API Docs](https://longcat.chat/platform/docs/) - LongCat model documentation

---

**Last Updated**: 2025-10-15  
**Maintained By**: Agent Runner Development Team
