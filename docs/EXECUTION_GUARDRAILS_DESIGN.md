# Execution Guardrails and Safety Mechanisms - Design Document

## 概述

本文档设计了一套完整的执行边界保护（Guardrails）机制，确保危险操作必须经过用户确认，防止意外的破坏性操作。

## 核心安全理念

### 1. **分级操作风险评估**

操作按照风险级别分为五个等级：

```rust
pub enum OperationRiskLevel {
    /// 安全操作 - 只读操作，无副作用
    Safe,           
    
    /// 低风险 - 可逆操作，如创建新文件
    Low,            
    
    /// 中风险 - 修改现有文件，但有备份
    Medium,         
    
    /// 高风险 - 删除、重命名、大规模修改
    High,           
    
    /// 极高风险 - 系统级操作、不可逆操作
    Critical,       
}
```

### 2. **操作分类**

```rust
pub enum OperationType {
    // 文件操作
    FileRead,           // Safe
    FileCreate,         // Low
    FileModify,         // Medium
    FileDelete,         // High
    FileRename,         // High
    FileMassModify,     // High - 修改多个文件
    
    // 目录操作
    DirectoryCreate,    // Low
    DirectoryDelete,    // Critical - 可能删除多个文件
    DirectoryRename,    // High
    
    // 命令执行
    CommandRead,        // Safe - 如 ls, cat
    CommandWrite,       // Medium - 如 echo >
    CommandDelete,      // Critical - 如 rm, rm -rf
    CommandSystem,      // Critical - 系统命令
    
    // 网络操作
    NetworkRead,        // Low - 如 GET 请求
    NetworkWrite,       // Medium - 如 POST 请求
    NetworkDelete,      // High - 如 DELETE 请求
    
    // 数据库操作
    DatabaseRead,       // Safe
    DatabaseInsert,     // Low
    DatabaseUpdate,     // Medium
    DatabaseDelete,     // High
    DatabaseDrop,       // Critical
    
    // 配置操作
    ConfigRead,         // Safe
    ConfigModify,       // High - 配置错误可能导致系统不可用
    
    // 部署操作
    DeployStart,        // Medium
    DeployStop,         // High
    DeployRestart,      // Medium
    DeployRollback,     // High
}
```

### 3. **危险模式检测**

系统会自动检测以下危险模式：

```rust
pub struct DangerousPatternDetector {
    patterns: Vec<DangerousPattern>,
}

pub struct DangerousPattern {
    /// 模式名称
    name: String,
    
    /// 检测规则（正则表达式）
    regex: Regex,
    
    /// 风险级别
    risk_level: OperationRiskLevel,
    
    /// 警告信息
    warning_message: String,
    
    /// 是否必须确认
    requires_confirmation: bool,
}
```

**预定义的危险模式：**

1. **删除操作**
   - `rm -rf` - Critical
   - `rm *.rs` - High
   - `DELETE FROM` - High
   - `DROP TABLE/DATABASE` - Critical

2. **批量操作**
   - `*.* ` 匹配所有文件 - High
   - `**/*` 递归匹配 - High
   - 修改超过 10 个文件 - High

3. **系统命令**
   - `sudo` - Critical
   - `chmod 777` - High
   - `chown` - High
   - `systemctl stop` - Critical

4. **网络危险操作**
   - `curl | sh` - Critical
   - `wget | bash` - Critical
   - 下载并执行脚本 - Critical

5. **配置文件修改**
   - `.env` 文件 - High
   - `config.yml/json` - High
   - `docker-compose.yml` - High
   - `Cargo.toml` 依赖修改 - Medium

## 核心类型设计

### 1. GuardrailConfig - 保护配置

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailConfig {
    /// 是否启用保护机制
    pub enabled: bool,
    
    /// 自动确认的最高风险级别
    /// 例如：Low 表示 Low 及以下自动执行，Medium+ 需要确认
    pub auto_confirm_threshold: OperationRiskLevel,
    
    /// 是否在执行前显示操作详情
    pub show_operation_details: bool,
    
    /// 是否启用操作模拟（dry-run）
    pub enable_dry_run: bool,
    
    /// 危险操作的确认超时时间（秒）
    pub confirmation_timeout_seconds: u64,
    
    /// 是否启用操作历史记录
    pub enable_operation_history: bool,
    
    /// 受保护的路径/文件列表
    pub protected_paths: Vec<String>,
    
    /// 禁止的操作类型
    pub forbidden_operations: Vec<OperationType>,
    
    /// 自定义危险模式
    pub custom_dangerous_patterns: Vec<DangerousPattern>,
}
```

### 2. OperationGuard - 操作守卫

```rust
pub struct OperationGuard {
    /// 操作ID
    pub id: String,
    
    /// 操作类型
    pub operation_type: OperationType,
    
    /// 风险级别
    pub risk_level: OperationRiskLevel,
    
    /// 操作描述
    pub description: String,
    
    /// 目标资源
    pub targets: Vec<OperationTarget>,
    
    /// 检测到的危险模式
    pub detected_patterns: Vec<DangerousPattern>,
    
    /// 是否需要确认
    pub requires_confirmation: bool,
    
    /// 确认提示信息
    pub confirmation_prompt: String,
    
    /// 预期影响
    pub expected_impact: OperationImpact,
    
    /// 回滚计划
    pub rollback_plan: Option<RollbackPlan>,
}

pub struct OperationTarget {
    /// 资源类型（文件、目录、数据库等）
    pub resource_type: String,
    
    /// 资源路径/名称
    pub path: String,
    
    /// 是否为受保护资源
    pub is_protected: bool,
    
    /// 当前状态快照（用于回滚）
    pub snapshot: Option<ResourceSnapshot>,
}

pub struct OperationImpact {
    /// 影响的文件数量
    pub affected_files: usize,
    
    /// 影响的代码行数
    pub affected_lines: usize,
    
    /// 是否可逆
    pub reversible: bool,
    
    /// 预计执行时间（秒）
    pub estimated_duration: u64,
    
    /// 影响范围描述
    pub scope_description: String,
}
```

### 3. ConfirmationRequest - 确认请求

```rust
pub struct ConfirmationRequest {
    /// 请求ID
    pub id: String,
    
    /// 关联的操作守卫
    pub operation_guard: OperationGuard,
    
    /// 请求时间
    pub requested_at: DateTime<Utc>,
    
    /// 超时时间
    pub timeout_at: DateTime<Utc>,
    
    /// 确认选项
    pub options: Vec<ConfirmationOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfirmationOption {
    /// 执行操作
    Proceed,
    
    /// 执行操作前先模拟（dry-run）
    DryRunFirst,
    
    /// 跳过此操作
    Skip,
    
    /// 中止整个执行计划
    Abort,
    
    /// 修改操作参数
    Modify,
}

pub struct ConfirmationResponse {
    /// 响应ID
    pub id: String,
    
    /// 用户选择
    pub choice: ConfirmationOption,
    
    /// 响应时间
    pub responded_at: DateTime<Utc>,
    
    /// 用户备注
    pub user_notes: Option<String>,
    
    /// 如果选择 Modify，修改后的参数
    pub modified_params: Option<HashMap<String, serde_json::Value>>,
}
```

### 4. RollbackPlan - 回滚计划

```rust
pub struct RollbackPlan {
    /// 回滚计划ID
    pub id: String,
    
    /// 回滚步骤
    pub steps: Vec<RollbackStep>,
    
    /// 是否自动回滚
    pub auto_rollback: bool,
    
    /// 回滚时间窗口（秒）
    pub rollback_window_seconds: u64,
}

pub struct RollbackStep {
    /// 步骤描述
    pub description: String,
    
    /// 回滚操作
    pub action: RollbackAction,
    
    /// 执行顺序（倒序执行）
    pub sequence: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollbackAction {
    /// 恢复文件
    RestoreFile {
        path: String,
        snapshot_id: String,
    },
    
    /// 删除文件
    DeleteFile {
        path: String,
    },
    
    /// 执行命令
    ExecuteCommand {
        command: String,
    },
    
    /// 恢复数据库
    RestoreDatabase {
        backup_id: String,
    },
}
```

## 执行流程集成

### 1. 在 ExecutionStep 中添加守卫

```rust
pub struct ExecutionStep {
    // ... 现有字段 ...
    
    /// 操作守卫
    pub operation_guard: Option<OperationGuard>,
    
    /// 是否需要在执行前创建快照
    pub create_snapshot_before: bool,
    
    /// 快照ID（执行后填充）
    pub snapshot_id: Option<String>,
}
```

### 2. 执行前检查流程

```
1. 分析操作
   ↓
2. 评估风险等级
   ↓
3. 检测危险模式
   ↓
4. 判断是否需要确认
   ↓
5a. 不需要确认 → 直接执行
5b. 需要确认 → 创建确认请求
   ↓
6. 等待用户响应
   ↓
7. 根据响应执行对应操作
```

### 3. GuardrailEngine - 保护引擎

```rust
pub struct GuardrailEngine {
    config: GuardrailConfig,
    pattern_detector: DangerousPatternDetector,
    confirmation_handler: ConfirmationHandler,
    snapshot_manager: SnapshotManager,
}

impl GuardrailEngine {
    /// 检查操作是否安全
    pub async fn check_operation(
        &self,
        operation: &ExecutionStep,
    ) -> Result<OperationGuard, AgentError> {
        // 1. 分析操作类型
        let op_type = self.analyze_operation_type(operation)?;
        
        // 2. 评估风险级别
        let risk_level = self.evaluate_risk_level(&op_type, operation)?;
        
        // 3. 检测危险模式
        let patterns = self.pattern_detector.detect(operation)?;
        
        // 4. 检查受保护路径
        let protected = self.check_protected_paths(operation)?;
        
        // 5. 创建操作守卫
        let guard = OperationGuard {
            id: uuid::Uuid::new_v4().to_string(),
            operation_type: op_type,
            risk_level,
            description: operation.description.clone(),
            targets: self.extract_targets(operation)?,
            detected_patterns: patterns,
            requires_confirmation: self.should_require_confirmation(&risk_level),
            confirmation_prompt: self.build_confirmation_prompt(operation, &risk_level)?,
            expected_impact: self.estimate_impact(operation)?,
            rollback_plan: self.create_rollback_plan(operation)?,
        };
        
        Ok(guard)
    }
    
    /// 请求用户确认
    pub async fn request_confirmation(
        &self,
        guard: &OperationGuard,
    ) -> Result<ConfirmationResponse, AgentError> {
        let request = ConfirmationRequest {
            id: uuid::Uuid::new_v4().to_string(),
            operation_guard: guard.clone(),
            requested_at: Utc::now(),
            timeout_at: Utc::now() + chrono::Duration::seconds(
                self.config.confirmation_timeout_seconds as i64
            ),
            options: vec![
                ConfirmationOption::Proceed,
                ConfirmationOption::DryRunFirst,
                ConfirmationOption::Skip,
                ConfirmationOption::Abort,
            ],
        };
        
        self.confirmation_handler.request(request).await
    }
    
    /// 执行 dry-run 模拟
    pub async fn dry_run(
        &self,
        operation: &ExecutionStep,
    ) -> Result<DryRunResult, AgentError> {
        // 模拟执行，不实际修改任何资源
        todo!()
    }
}
```

## 用户交互设计

### 1. 确认提示格式

```
⚠️  危险操作需要确认

操作类型: 文件删除
风险级别: 🔴 HIGH
操作描述: 删除临时文件目录

影响范围:
  • 删除文件: 23 个
  • 删除目录: 5 个
  • 总大小: 1.5 MB
  • 可逆性: ❌ 不可逆

目标资源:
  1. /tmp/build_cache/
  2. /tmp/test_data/
  3. ...

检测到的危险模式:
  ⚠️  批量删除操作 (rm -rf)
  ⚠️  递归目录删除

回滚计划: ❌ 无法回滚（未创建备份）

请选择操作:
  [P] Proceed - 继续执行
  [D] Dry Run - 先模拟执行
  [S] Skip - 跳过此步骤
  [A] Abort - 中止整个任务
  
选择 (P/D/S/A): _
```

### 2. Dry-Run 结果展示

```
🔍 模拟执行结果

将要删除的文件:
  ✓ /tmp/build_cache/file1.rs (120 KB)
  ✓ /tmp/build_cache/file2.rs (85 KB)
  ... (显示前 10 个)
  
将要删除的目录:
  ✓ /tmp/build_cache/
  ✓ /tmp/test_data/
  
预计执行时间: < 1 秒

确认执行？ (Y/n): _
```

## 特殊场景处理

### 1. 受保护路径

某些路径应该始终受到保护，即使风险级别较低：

```rust
pub const DEFAULT_PROTECTED_PATHS: &[&str] = &[
    ".git/",
    "node_modules/",
    "target/release/",
    ".env",
    "secrets/",
    "credentials/",
    "/etc/",
    "/usr/",
    "/System/",  // macOS
];
```

### 2. 批量操作阈值

当操作影响的资源数量超过阈值时，自动提升风险级别：

```rust
pub struct BatchOperationThresholds {
    /// 文件数量阈值
    pub file_count: usize,        // 默认: 10
    
    /// 代码行数阈值
    pub line_count: usize,        // 默认: 1000
    
    /// 文件大小阈值（字节）
    pub total_size_bytes: u64,    // 默认: 10 MB
}
```

### 3. 级联影响检测

检测操作可能产生的级联影响：

```rust
pub struct CascadeImpactAnalyzer {
    /// 依赖关系图
    dependency_graph: DependencyGraph,
}

impl CascadeImpactAnalyzer {
    /// 分析删除文件的级联影响
    pub fn analyze_file_deletion(
        &self,
        file_path: &str,
    ) -> Result<CascadeImpact, AgentError> {
        // 检查有哪些文件依赖这个文件
        let dependents = self.dependency_graph.find_dependents(file_path)?;
        
        CascadeImpact {
            directly_affected: vec![file_path.to_string()],
            indirectly_affected: dependents,
            broken_builds: self.check_broken_builds(&dependents)?,
            broken_tests: self.check_broken_tests(&dependents)?,
        }
    }
}
```

## 实现优先级

### Phase 1: 核心保护机制（本阶段实现）
- [x] 风险级别枚举
- [x] 操作类型枚举
- [x] OperationGuard 核心类型
- [x] GuardrailConfig 配置
- [x] 基本的风险评估逻辑

### Phase 2: 危险模式检测
- [ ] DangerousPatternDetector 实现
- [ ] 预定义危险模式库
- [ ] 正则表达式匹配引擎

### Phase 3: 用户确认机制
- [ ] ConfirmationHandler 实现
- [ ] 用户交互界面
- [ ] 超时处理

### Phase 4: 快照和回滚
- [ ] SnapshotManager 实现
- [ ] RollbackPlan 执行器
- [ ] 自动回滚机制

### Phase 5: 高级功能
- [ ] Dry-run 模拟执行
- [ ] 级联影响分析
- [ ] 操作历史记录
- [ ] 批量操作优化

## 使用示例

```rust
use agent_runner::execution::{SequentialExecutor, GuardrailEngine, GuardrailConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 配置保护机制
    let guardrail_config = GuardrailConfig {
        enabled: true,
        auto_confirm_threshold: OperationRiskLevel::Low,
        show_operation_details: true,
        enable_dry_run: true,
        confirmation_timeout_seconds: 120,
        enable_operation_history: true,
        protected_paths: vec![
            ".git/".to_string(),
            ".env".to_string(),
        ],
        forbidden_operations: vec![
            OperationType::DirectoryDelete,  // 禁止删除目录
        ],
        custom_dangerous_patterns: vec![],
    };
    
    // 2. 创建保护引擎
    let guardrail_engine = GuardrailEngine::new(guardrail_config);
    
    // 3. 配置执行器
    let execution_config = ExecutionConfig {
        max_retries_per_phase: 3,
        require_confirmation: true,  // 启用确认机制
        min_confidence_threshold: 0.7,
        enable_auto_rollback: true,
        verbose_logging: true,
    };
    
    // 4. 创建执行器，注入保护引擎
    let executor = SequentialExecutor::new_with_guardrails(
        model,
        execution_config,
        guardrail_engine,
    );
    
    // 5. 执行任务
    let plan = executor.execute_task("清理临时文件").await?;
    
    Ok(())
}
```

## 总结

这套保护机制提供了：

1. **多层次风险评估** - 从操作类型、影响范围、目标资源多个维度评估风险
2. **智能确认** - 根据风险级别自动决定是否需要用户确认
3. **透明度** - 清晰展示操作将产生的影响
4. **安全网** - 提供 dry-run、快照、回滚等多重保护
5. **灵活配置** - 支持自定义保护规则和危险模式
6. **用户友好** - 提供清晰的提示和多种操作选项

这确保了系统在执行危险操作前，用户能够充分了解风险并做出明智的决策。
