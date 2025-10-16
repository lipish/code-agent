//! Execution Guardrails - 执行保护机制
//!
//! 提供操作风险评估、用户确认、危险模式检测等安全保护功能。

use crate::errors::AgentError;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Risk Assessment Types
// ============================================================================

/// 操作风险级别
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

impl OperationRiskLevel {
    /// 获取风险级别的emoji表示
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Safe => "🟢",
            Self::Low => "🟡",
            Self::Medium => "🟠",
            Self::High => "🔴",
            Self::Critical => "🚨",
        }
    }

    /// 获取风险级别的描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Safe => "安全操作",
            Self::Low => "低风险",
            Self::Medium => "中风险",
            Self::High => "高风险",
            Self::Critical => "极高风险",
        }
    }
}

/// 操作类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationType {
    // 文件操作
    FileRead,
    FileCreate,
    FileModify,
    FileDelete,
    FileRename,
    FileMassModify { count: usize },

    // 目录操作
    DirectoryCreate,
    DirectoryDelete,
    DirectoryRename,

    // 命令执行
    CommandRead,
    CommandWrite,
    CommandDelete,
    CommandSystem,

    // 网络操作
    NetworkRead,
    NetworkWrite,
    NetworkDelete,

    // 数据库操作
    DatabaseRead,
    DatabaseInsert,
    DatabaseUpdate,
    DatabaseDelete,
    DatabaseDrop,

    // 配置操作
    ConfigRead,
    ConfigModify,

    // 部署操作
    DeployStart,
    DeployStop,
    DeployRestart,
    DeployRollback,

    // 其他
    Unknown,
}

impl OperationType {
    /// 获取操作类型的默认风险级别
    pub fn default_risk_level(&self) -> OperationRiskLevel {
        match self {
            Self::FileRead | Self::CommandRead | Self::DatabaseRead | Self::ConfigRead => {
                OperationRiskLevel::Safe
            }
            Self::FileCreate
            | Self::DirectoryCreate
            | Self::NetworkRead
            | Self::DatabaseInsert => OperationRiskLevel::Low,
            Self::FileModify
            | Self::CommandWrite
            | Self::NetworkWrite
            | Self::DatabaseUpdate
            | Self::DeployStart
            | Self::DeployRestart => OperationRiskLevel::Medium,
            Self::FileDelete
            | Self::FileRename
            | Self::FileMassModify { .. }
            | Self::DirectoryRename
            | Self::NetworkDelete
            | Self::DatabaseDelete
            | Self::ConfigModify
            | Self::DeployStop
            | Self::DeployRollback => OperationRiskLevel::High,
            Self::DirectoryDelete | Self::CommandDelete | Self::CommandSystem | Self::DatabaseDrop => {
                OperationRiskLevel::Critical
            }
            Self::Unknown => OperationRiskLevel::High,
        }
    }
}

// ============================================================================
// Dangerous Pattern Detection
// ============================================================================

/// 危险模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DangerousPattern {
    /// 模式名称
    pub name: String,
    /// 模式描述
    pub description: String,
    /// 检测规则（正则表达式字符串）
    pub pattern: String,
    /// 风险级别
    pub risk_level: OperationRiskLevel,
    /// 警告信息
    pub warning_message: String,
    /// 是否必须确认
    pub requires_confirmation: bool,
}

impl DangerousPattern {
    /// 创建正则表达式
    pub fn regex(&self) -> Result<Regex, AgentError> {
        Regex::new(&self.pattern)
            .map_err(|e| AgentError::ExecutionError(format!("Invalid regex pattern: {}", e)))
    }
}

/// 危险模式检测器
pub struct DangerousPatternDetector {
    patterns: Vec<DangerousPattern>,
}

impl DangerousPatternDetector {
    /// 创建新的检测器
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
        }
    }

    /// 添加自定义模式
    pub fn add_pattern(&mut self, pattern: DangerousPattern) {
        self.patterns.push(pattern);
    }

    /// 检测文本中的危险模式
    pub fn detect(&self, text: &str) -> Result<Vec<DangerousPattern>, AgentError> {
        let mut detected = Vec::new();

        for pattern in &self.patterns {
            let regex = pattern.regex()?;
            if regex.is_match(text) {
                detected.push(pattern.clone());
            }
        }

        Ok(detected)
    }

    /// 默认的危险模式
    fn default_patterns() -> Vec<DangerousPattern> {
        vec![
            // 删除操作
            DangerousPattern {
                name: "rm_rf".to_string(),
                description: "强制递归删除".to_string(),
                pattern: r"rm\s+-rf?|rm\s+-fr?".to_string(),
                risk_level: OperationRiskLevel::Critical,
                warning_message: "检测到 rm -rf 命令，这将递归删除目录且无法恢复".to_string(),
                requires_confirmation: true,
            },
            DangerousPattern {
                name: "delete_all".to_string(),
                description: "删除所有文件".to_string(),
                pattern: r"rm\s+\*|del\s+\*|DELETE\s+FROM\s+\w+\s*;".to_string(),
                risk_level: OperationRiskLevel::High,
                warning_message: "检测到批量删除操作".to_string(),
                requires_confirmation: true,
            },
            DangerousPattern {
                name: "drop_database".to_string(),
                description: "删除数据库".to_string(),
                pattern: r"DROP\s+(DATABASE|TABLE|SCHEMA)".to_string(),
                risk_level: OperationRiskLevel::Critical,
                warning_message: "检测到数据库删除操作".to_string(),
                requires_confirmation: true,
            },
            // 系统命令
            DangerousPattern {
                name: "sudo_command".to_string(),
                description: "超级用户权限".to_string(),
                pattern: r"sudo\s+".to_string(),
                risk_level: OperationRiskLevel::Critical,
                warning_message: "检测到 sudo 命令，将以管理员权限执行".to_string(),
                requires_confirmation: true,
            },
            DangerousPattern {
                name: "chmod_777".to_string(),
                description: "不安全的文件权限".to_string(),
                pattern: r"chmod\s+777".to_string(),
                risk_level: OperationRiskLevel::High,
                warning_message: "检测到 chmod 777，这会使文件对所有用户可读写执行".to_string(),
                requires_confirmation: true,
            },
            // 网络危险操作
            DangerousPattern {
                name: "curl_pipe_shell".to_string(),
                description: "下载并执行脚本".to_string(),
                pattern: r"curl.*\|\s*(sh|bash)|wget.*\|\s*(sh|bash)".to_string(),
                risk_level: OperationRiskLevel::Critical,
                warning_message: "检测到从网络下载并直接执行脚本，存在安全风险".to_string(),
                requires_confirmation: true,
            },
            // 批量修改
            DangerousPattern {
                name: "recursive_operation".to_string(),
                description: "递归操作".to_string(),
                pattern: r"\*\*/\*|\*\.\*".to_string(),
                risk_level: OperationRiskLevel::High,
                warning_message: "检测到递归通配符操作".to_string(),
                requires_confirmation: true,
            },
        ]
    }
}

impl Default for DangerousPatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Operation Guard
// ============================================================================

/// 操作守卫
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// 操作目标资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationTarget {
    /// 资源类型
    pub resource_type: String,
    /// 资源路径/名称
    pub path: String,
    /// 是否为受保护资源
    pub is_protected: bool,
    /// 当前状态快照（用于回滚）
    pub snapshot: Option<ResourceSnapshot>,
}

/// 资源快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    /// 快照ID
    pub id: String,
    /// 快照时间
    pub created_at: DateTime<Utc>,
    /// 资源类型
    pub resource_type: String,
    /// 资源路径
    pub path: String,
    /// 快照数据（根据资源类型存储不同的数据）
    pub data: serde_json::Value,
}

/// 操作影响
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationImpact {
    /// 影响的文件数量
    pub affected_files: usize,
    /// 影响的代码行数
    pub affected_lines: usize,
    /// 影响的目录数量
    pub affected_directories: usize,
    /// 是否可逆
    pub reversible: bool,
    /// 预计执行时间（秒）
    pub estimated_duration: u64,
    /// 影响范围描述
    pub scope_description: String,
    /// 级联影响
    pub cascade_impact: Option<CascadeImpact>,
}

/// 级联影响
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeImpact {
    /// 直接影响的资源
    pub directly_affected: Vec<String>,
    /// 间接影响的资源
    pub indirectly_affected: Vec<String>,
    /// 可能导致构建失败
    pub broken_builds: Vec<String>,
    /// 可能导致测试失败
    pub broken_tests: Vec<String>,
}

// ============================================================================
// Rollback Plan
// ============================================================================

/// 回滚计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackPlan {
    /// 回滚计划ID
    pub id: String,
    /// 回滚步骤
    pub steps: Vec<RollbackStep>,
    /// 是否自动回滚
    pub auto_rollback: bool,
    /// 回滚时间窗口（秒）
    pub rollback_window_seconds: u64,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

impl RollbackPlan {
    /// 创建新的回滚计划
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            steps: Vec::new(),
            auto_rollback: true,
            rollback_window_seconds: 300, // 5 minutes
            created_at: Utc::now(),
        }
    }

    /// 添加回滚步骤
    pub fn add_step(&mut self, step: RollbackStep) {
        self.steps.push(step);
    }

    /// 按倒序执行回滚
    pub fn steps_reversed(&self) -> Vec<&RollbackStep> {
        self.steps.iter().rev().collect()
    }
}

impl Default for RollbackPlan {
    fn default() -> Self {
        Self::new()
    }
}

/// 回滚步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackStep {
    /// 步骤ID
    pub id: String,
    /// 步骤描述
    pub description: String,
    /// 回滚操作
    pub action: RollbackAction,
    /// 执行顺序
    pub sequence: usize,
}

/// 回滚操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollbackAction {
    /// 恢复文件
    RestoreFile { path: String, snapshot_id: String },
    /// 删除文件
    DeleteFile { path: String },
    /// 执行命令
    ExecuteCommand { command: String },
    /// 恢复数据库
    RestoreDatabase { backup_id: String },
    /// 撤销配置修改
    RestoreConfig { path: String, snapshot_id: String },
}

// ============================================================================
// Confirmation
// ============================================================================

/// 确认选项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// 确认请求
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl ConfirmationRequest {
    /// 格式化为用户友好的提示信息
    pub fn format_prompt(&self) -> String {
        let guard = &self.operation_guard;
        let mut prompt = String::new();

        prompt.push_str("⚠️  危险操作需要确认\n\n");
        prompt.push_str(&format!(
            "操作类型: {}\n",
            format!("{:?}", guard.operation_type)
        ));
        prompt.push_str(&format!(
            "风险级别: {} {}\n",
            guard.risk_level.emoji(),
            guard.risk_level.description()
        ));
        prompt.push_str(&format!("操作描述: {}\n\n", guard.description));

        // 影响范围
        let impact = &guard.expected_impact;
        prompt.push_str("影响范围:\n");
        if impact.affected_files > 0 {
            prompt.push_str(&format!("  • 影响文件: {} 个\n", impact.affected_files));
        }
        if impact.affected_directories > 0 {
            prompt.push_str(&format!("  • 影响目录: {} 个\n", impact.affected_directories));
        }
        if impact.affected_lines > 0 {
            prompt.push_str(&format!("  • 影响代码行: {} 行\n", impact.affected_lines));
        }
        prompt.push_str(&format!(
            "  • 可逆性: {}\n",
            if impact.reversible { "✅ 可逆" } else { "❌ 不可逆" }
        ));
        prompt.push_str(&format!(
            "  • 预计时间: {} 秒\n\n",
            impact.estimated_duration
        ));

        // 目标资源
        if !guard.targets.is_empty() {
            prompt.push_str("目标资源:\n");
            for (i, target) in guard.targets.iter().take(5).enumerate() {
                let protected = if target.is_protected { "🔒" } else { "" };
                prompt.push_str(&format!("  {}. {} {}\n", i + 1, target.path, protected));
            }
            if guard.targets.len() > 5 {
                prompt.push_str(&format!("  ... 还有 {} 个资源\n", guard.targets.len() - 5));
            }
            prompt.push('\n');
        }

        // 检测到的危险模式
        if !guard.detected_patterns.is_empty() {
            prompt.push_str("检测到的危险模式:\n");
            for pattern in &guard.detected_patterns {
                prompt.push_str(&format!("  ⚠️  {}: {}\n", pattern.name, pattern.warning_message));
            }
            prompt.push('\n');
        }

        // 回滚计划
        if let Some(rollback) = &guard.rollback_plan {
            prompt.push_str(&format!(
                "回滚计划: {} ({} 个步骤)\n\n",
                if rollback.auto_rollback {
                    "✅ 自动回滚"
                } else {
                    "⚠️  手动回滚"
                },
                rollback.steps.len()
            ));
        } else {
            prompt.push_str("回滚计划: ❌ 无法回滚\n\n");
        }

        // 选项
        prompt.push_str("请选择操作:\n");
        prompt.push_str("  [P] Proceed - 继续执行\n");
        prompt.push_str("  [D] Dry Run - 先模拟执行\n");
        prompt.push_str("  [S] Skip - 跳过此步骤\n");
        prompt.push_str("  [A] Abort - 中止整个任务\n\n");
        prompt.push_str("选择 (P/D/S/A): ");

        prompt
    }
}

/// 确认响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationResponse {
    /// 响应ID
    pub id: String,
    /// 关联的请求ID
    pub request_id: String,
    /// 用户选择
    pub choice: ConfirmationOption,
    /// 响应时间
    pub responded_at: DateTime<Utc>,
    /// 用户备注
    pub user_notes: Option<String>,
    /// 如果选择 Modify，修改后的参数
    pub modified_params: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Guardrail Configuration
// ============================================================================

/// 保护配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailConfig {
    /// 是否启用保护机制
    pub enabled: bool,
    /// 自动确认的最高风险级别
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
    /// 批量操作阈值
    pub batch_operation_thresholds: BatchOperationThresholds,
}

impl Default for GuardrailConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_confirm_threshold: OperationRiskLevel::Low,
            show_operation_details: true,
            enable_dry_run: true,
            confirmation_timeout_seconds: 120,
            enable_operation_history: true,
            protected_paths: DEFAULT_PROTECTED_PATHS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            forbidden_operations: vec![],
            custom_dangerous_patterns: vec![],
            batch_operation_thresholds: BatchOperationThresholds::default(),
        }
    }
}

/// 批量操作阈值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationThresholds {
    /// 文件数量阈值
    pub file_count: usize,
    /// 代码行数阈值
    pub line_count: usize,
    /// 文件大小阈值（字节）
    pub total_size_bytes: u64,
}

impl Default for BatchOperationThresholds {
    fn default() -> Self {
        Self {
            file_count: 10,
            line_count: 1000,
            total_size_bytes: 10 * 1024 * 1024, // 10 MB
        }
    }
}

/// 默认受保护路径
pub const DEFAULT_PROTECTED_PATHS: &[&str] = &[
    ".git/",
    "node_modules/",
    "target/release/",
    ".env",
    "secrets/",
    "credentials/",
    "/etc/",
    "/usr/",
    "/System/", // macOS
];

// ============================================================================
// Dry Run Result
// ============================================================================

/// Dry-run 模拟结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunResult {
    /// 成功
    pub success: bool,
    /// 将要执行的操作
    pub planned_actions: Vec<PlannedAction>,
    /// 预计影响
    pub estimated_impact: OperationImpact,
    /// 警告信息
    pub warnings: Vec<String>,
    /// 错误信息
    pub errors: Vec<String>,
}

/// 计划的操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedAction {
    /// 操作类型
    pub action_type: String,
    /// 操作描述
    pub description: String,
    /// 目标资源
    pub target: String,
    /// 是否成功
    pub would_succeed: bool,
    /// 失败原因（如果有）
    pub failure_reason: Option<String>,
}

// ============================================================================
// Guardrail Engine
// ============================================================================

/// 保护引擎
pub struct GuardrailEngine {
    config: GuardrailConfig,
    pattern_detector: DangerousPatternDetector,
}

impl GuardrailEngine {
    /// 创建新的保护引擎
    pub fn new(config: GuardrailConfig) -> Self {
        let mut pattern_detector = DangerousPatternDetector::new();
        
        // 添加自定义危险模式
        for pattern in &config.custom_dangerous_patterns {
            pattern_detector.add_pattern(pattern.clone());
        }
        
        Self {
            config,
            pattern_detector,
        }
    }

    /// 检查操作是否安全
    pub fn check_operation(
        &self,
        operation_type: OperationType,
        description: &str,
        targets: Vec<OperationTarget>,
    ) -> Result<OperationGuard, AgentError> {
        if !self.config.enabled {
            // 保护机制未启用，创建一个不需要确认的守卫
            return Ok(OperationGuard {
                id: uuid::Uuid::new_v4().to_string(),
                operation_type: operation_type.clone(),
                risk_level: operation_type.default_risk_level(),
                description: description.to_string(),
                targets,
                detected_patterns: vec![],
                requires_confirmation: false,
                confirmation_prompt: String::new(),
                expected_impact: OperationImpact::default(),
                rollback_plan: None,
            });
        }

        // 1. 检查是否为禁止的操作
        if self.config.forbidden_operations.contains(&operation_type) {
            return Err(AgentError::ExecutionError(format!(
                "操作被禁止: {:?}",
                operation_type
            )));
        }

        // 2. 评估基础风险级别
        let mut risk_level = operation_type.default_risk_level();

        // 3. 检测危险模式
        let detected_patterns = self.pattern_detector.detect(description)?;
        if !detected_patterns.is_empty() {
            // 如果检测到危险模式，提升风险级别
            for pattern in &detected_patterns {
                if pattern.risk_level > risk_level {
                    risk_level = pattern.risk_level;
                }
            }
        }

        // 4. 检查受保护路径
        for target in &targets {
            if self.is_protected_path(&target.path) {
                // 对受保护路径的操作，至少为 High 风险
                if risk_level < OperationRiskLevel::High {
                    risk_level = OperationRiskLevel::High;
                }
            }
        }

        // 5. 评估批量操作
        risk_level = self.evaluate_batch_operation_risk(risk_level, &targets);

        // 6. 评估操作影响
        let expected_impact = self.estimate_impact(&targets, &operation_type)?;

        // 7. 创建回滚计划
        let rollback_plan = self.create_rollback_plan(&operation_type, &targets)?;

        // 8. 判断是否需要确认
        let requires_confirmation = self.should_require_confirmation(
            &risk_level,
            &detected_patterns,
            &expected_impact,
        );

        // 9. 构建确认提示
        let confirmation_prompt = if requires_confirmation {
            self.build_confirmation_prompt(
                &operation_type,
                &risk_level,
                description,
                &detected_patterns,
            )
        } else {
            String::new()
        };

        Ok(OperationGuard {
            id: uuid::Uuid::new_v4().to_string(),
            operation_type,
            risk_level,
            description: description.to_string(),
            targets,
            detected_patterns,
            requires_confirmation,
            confirmation_prompt,
            expected_impact,
            rollback_plan,
        })
    }

    /// 检查路径是否受保护
    fn is_protected_path(&self, path: &str) -> bool {
        for protected in &self.config.protected_paths {
            if path.starts_with(protected) || path.contains(protected) {
                return true;
            }
        }
        false
    }

    /// 评估批量操作的风险
    fn evaluate_batch_operation_risk(
        &self,
        current_risk: OperationRiskLevel,
        targets: &[OperationTarget],
    ) -> OperationRiskLevel {
        let thresholds = &self.config.batch_operation_thresholds;
        
        // 如果目标数量超过阈值，提升风险级别
        if targets.len() > thresholds.file_count {
            if current_risk < OperationRiskLevel::High {
                return OperationRiskLevel::High;
            }
        }
        
        current_risk
    }

    /// 评估操作影响
    fn estimate_impact(
        &self,
        targets: &[OperationTarget],
        operation_type: &OperationType,
    ) -> Result<OperationImpact, AgentError> {
        let affected_files = targets
            .iter()
            .filter(|t| t.resource_type == "file")
            .count();
        
        let affected_directories = targets
            .iter()
            .filter(|t| t.resource_type == "directory")
            .count();

        // 简单估算代码行数（实际实现可以读取文件）
        let affected_lines = affected_files * 100; // 假设平均每个文件 100 行

        // 判断是否可逆
        let reversible = matches!(
            operation_type,
            OperationType::FileCreate
                | OperationType::FileModify
                | OperationType::DirectoryCreate
        );

        // 估算执行时间
        let estimated_duration = (affected_files + affected_directories * 10) as u64;

        let scope_description = format!(
            "影响 {} 个文件和 {} 个目录",
            affected_files, affected_directories
        );

        Ok(OperationImpact {
            affected_files,
            affected_lines,
            affected_directories,
            reversible,
            estimated_duration,
            scope_description,
            cascade_impact: None,
        })
    }

    /// 创建回滚计划
    fn create_rollback_plan(
        &self,
        operation_type: &OperationType,
        targets: &[OperationTarget],
    ) -> Result<Option<RollbackPlan>, AgentError> {
        // 对于某些操作类型，无法创建回滚计划
        if matches!(
            operation_type,
            OperationType::FileDelete
                | OperationType::DirectoryDelete
                | OperationType::DatabaseDrop
        ) {
            // 删除操作无法回滚（除非有快照）
            if targets.iter().any(|t| t.snapshot.is_some()) {
                let mut plan = RollbackPlan::new();
                
                for (i, target) in targets.iter().enumerate() {
                    if let Some(snapshot) = &target.snapshot {
                        plan.add_step(RollbackStep {
                            id: uuid::Uuid::new_v4().to_string(),
                            description: format!("恢复 {}", target.path),
                            action: RollbackAction::RestoreFile {
                                path: target.path.clone(),
                                snapshot_id: snapshot.id.clone(),
                            },
                            sequence: i,
                        });
                    }
                }
                
                return Ok(Some(plan));
            }
            return Ok(None);
        }

        // 对于修改操作，可以创建回滚计划
        if matches!(operation_type, OperationType::FileModify | OperationType::ConfigModify) {
            let mut plan = RollbackPlan::new();
            
            for (i, target) in targets.iter().enumerate() {
                if target.snapshot.is_some() {
                    plan.add_step(RollbackStep {
                        id: uuid::Uuid::new_v4().to_string(),
                        description: format!("恢复 {} 的修改", target.path),
                        action: RollbackAction::RestoreFile {
                            path: target.path.clone(),
                            snapshot_id: format!("snapshot_{}", i),
                        },
                        sequence: i,
                    });
                }
            }
            
            return Ok(Some(plan));
        }

        // 对于创建操作，回滚就是删除
        if matches!(operation_type, OperationType::FileCreate | OperationType::DirectoryCreate) {
            let mut plan = RollbackPlan::new();
            
            for (i, target) in targets.iter().enumerate() {
                plan.add_step(RollbackStep {
                    id: uuid::Uuid::new_v4().to_string(),
                    description: format!("删除 {}", target.path),
                    action: RollbackAction::DeleteFile {
                        path: target.path.clone(),
                    },
                    sequence: i,
                });
            }
            
            return Ok(Some(plan));
        }

        Ok(None)
    }

    /// 判断是否需要确认
    fn should_require_confirmation(
        &self,
        risk_level: &OperationRiskLevel,
        detected_patterns: &[DangerousPattern],
        impact: &OperationImpact,
    ) -> bool {
        // 1. 检查是否超过自动确认阈值
        if *risk_level > self.config.auto_confirm_threshold {
            return true;
        }

        // 2. 检查是否检测到需要确认的危险模式
        if detected_patterns.iter().any(|p| p.requires_confirmation) {
            return true;
        }

        // 3. 检查是否为不可逆操作
        if !impact.reversible && *risk_level >= OperationRiskLevel::Medium {
            return true;
        }

        // 4. 检查批量操作
        let thresholds = &self.config.batch_operation_thresholds;
        if impact.affected_files > thresholds.file_count
            || impact.affected_lines > thresholds.line_count
        {
            return true;
        }

        false
    }

    /// 构建确认提示
    fn build_confirmation_prompt(
        &self,
        operation_type: &OperationType,
        risk_level: &OperationRiskLevel,
        description: &str,
        detected_patterns: &[DangerousPattern],
    ) -> String {
        let mut prompt = format!(
            "{}操作: {:?}\n风险级别: {} {}\n描述: {}\n",
            if *risk_level >= OperationRiskLevel::High {
                "⚠️  "
            } else {
                ""
            },
            operation_type,
            risk_level.emoji(),
            risk_level.description(),
            description
        );

        if !detected_patterns.is_empty() {
            prompt.push_str("\n检测到的危险模式:\n");
            for pattern in detected_patterns {
                prompt.push_str(&format!("  • {}: {}\n", pattern.name, pattern.warning_message));
            }
        }

        prompt
    }
}

impl Default for GuardrailEngine {
    fn default() -> Self {
        Self::new(GuardrailConfig::default())
    }
}

impl Default for OperationImpact {
    fn default() -> Self {
        Self {
            affected_files: 0,
            affected_lines: 0,
            affected_directories: 0,
            reversible: true,
            estimated_duration: 0,
            scope_description: String::new(),
            cascade_impact: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_risk_levels() {
        assert!(OperationRiskLevel::Safe < OperationRiskLevel::Low);
        assert!(OperationRiskLevel::High < OperationRiskLevel::Critical);
    }

    #[test]
    fn test_operation_type_risk_mapping() {
        assert_eq!(OperationType::FileRead.default_risk_level(), OperationRiskLevel::Safe);
        assert_eq!(OperationType::FileDelete.default_risk_level(), OperationRiskLevel::High);
        assert_eq!(OperationType::DirectoryDelete.default_risk_level(), OperationRiskLevel::Critical);
    }

    #[test]
    fn test_dangerous_pattern_detector() {
        let detector = DangerousPatternDetector::new();
        
        // Test rm -rf detection
        let result = detector.detect("rm -rf /tmp/test").unwrap();
        assert!(!result.is_empty());
        assert_eq!(result[0].name, "rm_rf");
        
        // Test safe command
        let result = detector.detect("ls -la").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_rollback_plan() {
        let mut plan = RollbackPlan::new();
        assert!(plan.steps.is_empty());
        
        plan.add_step(RollbackStep {
            id: "step1".to_string(),
            description: "Restore file".to_string(),
            action: RollbackAction::RestoreFile {
                path: "/path/to/file".to_string(),
                snapshot_id: "snap1".to_string(),
            },
            sequence: 1,
        });
        
        assert_eq!(plan.steps.len(), 1);
        assert_eq!(plan.steps_reversed().len(), 1);
    }

    #[test]
    fn test_guardrail_engine_safe_operation() {
        let engine = GuardrailEngine::default();
        
        let guard = engine.check_operation(
            OperationType::FileRead,
            "读取配置文件",
            vec![OperationTarget {
                resource_type: "file".to_string(),
                path: "config.json".to_string(),
                is_protected: false,
                snapshot: None,
            }],
        ).unwrap();
        
        assert_eq!(guard.risk_level, OperationRiskLevel::Safe);
        assert!(!guard.requires_confirmation);
    }

    #[test]
    fn test_guardrail_engine_dangerous_operation() {
        let engine = GuardrailEngine::default();
        
        let guard = engine.check_operation(
            OperationType::FileDelete,
            "rm -rf /tmp/test",
            vec![OperationTarget {
                resource_type: "file".to_string(),
                path: "/tmp/test".to_string(),
                is_protected: false,
                snapshot: None,
            }],
        ).unwrap();
        
        assert!(guard.risk_level >= OperationRiskLevel::High);
        assert!(guard.requires_confirmation);
        assert!(!guard.detected_patterns.is_empty());
    }

    #[test]
    fn test_guardrail_engine_protected_path() {
        let engine = GuardrailEngine::default();
        
        let guard = engine.check_operation(
            OperationType::FileModify,
            "修改 .env 文件",
            vec![OperationTarget {
                resource_type: "file".to_string(),
                path: ".env".to_string(),
                is_protected: true,
                snapshot: None,
            }],
        ).unwrap();
        
        assert!(guard.risk_level >= OperationRiskLevel::High);
        assert!(guard.requires_confirmation);
    }
}
