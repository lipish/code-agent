# Guardrails Quick Reference Card

## Risk Levels

| Level | Emoji | Description | Auto-Confirm |
|-------|-------|-------------|--------------|
| Safe | 🟢 | Read-only, no side effects | Yes |
| Low | 🟡 | Reversible operations | Yes (default) |
| Medium | 🟠 | Modifications with backup | No |
| High | 🔴 | Deletion, batch operations | No |
| Critical | 🚨 | System-level, irreversible | No |

## Operation Type → Risk Mapping

### File Operations
```rust
FileRead           → Safe
FileCreate         → Low
FileModify         → Medium
FileDelete         → High
FileRename         → High
FileMassModify     → High
```

### Directory Operations
```rust
DirectoryCreate    → Low
DirectoryDelete    → Critical  // Can delete many files
DirectoryRename    → High
```

### Command Execution
```rust
CommandRead        → Safe      // ls, cat
CommandWrite       → Medium    // echo >
CommandDelete      → Critical  // rm, rm -rf
CommandSystem      → Critical  // System commands
```

### Database Operations
```rust
DatabaseRead       → Safe
DatabaseInsert     → Low
DatabaseUpdate     → Medium
DatabaseDelete     → High
DatabaseDrop       → Critical
```

## Dangerous Patterns

| Pattern | Regex | Risk | Example |
|---------|-------|------|---------|
| rm_rf | `rm\s+-rf?` | 🚨 Critical | `rm -rf /tmp/data` |
| delete_all | `rm\s+\*` | 🔴 High | `rm *.log` |
| drop_database | `DROP\s+(DATABASE\|TABLE)` | 🚨 Critical | `DROP DATABASE prod` |
| sudo_command | `sudo\s+` | 🚨 Critical | `sudo systemctl stop` |
| chmod_777 | `chmod\s+777` | 🔴 High | `chmod 777 file` |
| curl_pipe_shell | `curl.*\|\s*sh` | 🚨 Critical | `curl url \| bash` |
| recursive_operation | `\*\*/\*` | 🔴 High | `**/*.rs` |

## Protected Paths

```
.git/
node_modules/
target/release/
.env
secrets/
credentials/
/etc/
/usr/
/System/
```

Operations on protected paths are automatically upgraded to at least High risk.

## Batch Thresholds

```rust
file_count: 10              // More than 10 files → High
line_count: 1000            // More than 1000 lines → High
total_size_bytes: 10 MB     // More than 10 MB → High
```

## Quick Setup

### Basic Usage

```rust
use agent_runner::execution::{GuardrailEngine, GuardrailConfig, OperationRiskLevel};

let config = GuardrailConfig {
    enabled: true,
    auto_confirm_threshold: OperationRiskLevel::Low,
    ..Default::default()
};
let engine = GuardrailEngine::new(config);
```

### With Sequential Executor

```rust
use agent_runner::execution::{SequentialExecutor, GuardrailEngine};

let executor = SequentialExecutor::new_with_guardrails(
    model,
    execution_config,
    GuardrailEngine::new(GuardrailConfig::default()),
);
```

### Check Operation

```rust
let guard = engine.check_operation(
    OperationType::FileDelete,
    "rm old_file.txt",
    vec![OperationTarget {
        resource_type: "file".to_string(),
        path: "old_file.txt".to_string(),
        is_protected: false,
        snapshot: None,
    }],
)?;

if guard.requires_confirmation {
    // Show warning and request confirmation
}
```

## Configuration Options

```rust
GuardrailConfig {
    enabled: bool,                           // Master switch
    auto_confirm_threshold: OperationRiskLevel,  // Auto-confirm up to this level
    show_operation_details: bool,            // Show detailed info
    enable_dry_run: bool,                    // Enable simulation
    confirmation_timeout_seconds: u64,       // Default: 120
    enable_operation_history: bool,          // Log operations
    protected_paths: Vec<String>,            // Custom protected paths
    forbidden_operations: Vec<OperationType>, // Completely forbidden
    custom_dangerous_patterns: Vec<DangerousPattern>,
    batch_operation_thresholds: BatchOperationThresholds,
}
```

## Confirmation Options

When an operation requires confirmation:

```rust
pub enum ConfirmationOption {
    Proceed,      // Execute the operation
    DryRunFirst,  // Simulate first, then decide
    Skip,         // Skip this operation
    Abort,        // Abort entire task
    Modify,       // Modify operation parameters
}
```

## Rollback Actions

```rust
pub enum RollbackAction {
    RestoreFile { path, snapshot_id },     // Restore from snapshot
    DeleteFile { path },                   // Delete created file
    ExecuteCommand { command },            // Run rollback command
    RestoreDatabase { backup_id },         // Restore DB backup
    RestoreConfig { path, snapshot_id },   // Restore config
}
```

## Common Patterns

### 1. Add Custom Protected Path

```rust
let mut config = GuardrailConfig::default();
config.protected_paths.push("my_important_dir/".to_string());
```

### 2. Add Custom Dangerous Pattern

```rust
use agent_runner::execution::DangerousPattern;

let pattern = DangerousPattern {
    name: "force_push".to_string(),
    description: "Git force push".to_string(),
    pattern: r"git\s+push\s+(-f|--force)".to_string(),
    risk_level: OperationRiskLevel::Critical,
    warning_message: "Force push detected!".to_string(),
    requires_confirmation: true,
};

config.custom_dangerous_patterns.push(pattern);
```

### 3. Forbid Specific Operations

```rust
config.forbidden_operations = vec![
    OperationType::DirectoryDelete,
    OperationType::DatabaseDrop,
];
```

### 4. Adjust Batch Thresholds

```rust
config.batch_operation_thresholds = BatchOperationThresholds {
    file_count: 5,           // More strict
    line_count: 500,
    total_size_bytes: 5 * 1024 * 1024,  // 5 MB
};
```

## Decision Flow

```
Operation Submitted
    ↓
Check forbidden operations → If forbidden: Error
    ↓
Get base risk level from operation type
    ↓
Detect dangerous patterns → Upgrade risk if detected
    ↓
Check protected paths → Upgrade to High if protected
    ↓
Check batch thresholds → Upgrade to High if exceeded
    ↓
Estimate impact (files, lines, reversibility)
    ↓
Create rollback plan (if possible)
    ↓
Determine if confirmation needed:
    - Risk > auto_confirm_threshold?
    - Dangerous pattern requires confirmation?
    - Irreversible & risk >= Medium?
    - Batch threshold exceeded?
    ↓
If confirmation needed:
    - Show operation details
    - Show detected patterns
    - Show impact estimation
    - Show rollback plan
    - Request user choice
    ↓
Execute or Skip based on user choice
```

## Examples

### Safe Operation (Auto-Confirmed)
```rust
// Reading a file → Safe → Auto-confirmed
FileRead + "config.json" → ✅ Execute
```

### Low Risk (Auto-Confirmed with Rollback)
```rust
// Creating a file → Low → Auto-confirmed, can rollback by deleting
FileCreate + "output.txt" → ✅ Execute (rollback: delete)
```

### Medium Risk (Requires Confirmation)
```rust
// Modifying a file → Medium → Requires confirmation
FileModify + "src/main.rs" → ⚠️ Confirm?
```

### High Risk (Requires Confirmation + Warning)
```rust
// Deleting a file → High → Requires confirmation
FileDelete + "data.db" → 🔴 Confirm! (irreversible)
```

### Critical Risk (Requires Confirmation + Multiple Warnings)
```rust
// Dangerous command → Critical → Requires confirmation
CommandDelete + "rm -rf /tmp/cache" → 🚨 DANGER! Confirm?
  ⚠️ Detected: rm_rf (critical)
  ⚠️ Recursive deletion
  ❌ No rollback possible
```

### Protected Path (Risk Upgraded)
```rust
// Modifying .env → Medium → Upgraded to High
ConfigModify + ".env" → 🔴 Confirm! (protected path)
```

### Batch Operation (Risk Upgraded)
```rust
// Modifying 15 files → High (batch threshold)
FileMassModify{15} → 🔴 Confirm! (15 files > threshold)
```

## Testing

Run the demo:
```bash
cargo run --example guardrails_demo
```

Run tests:
```bash
cargo test guardrails
```

## See Also

- [Design Document](EXECUTION_GUARDRAILS_DESIGN.md)
- [Implementation Summary](GUARDRAILS_IMPLEMENTATION_SUMMARY.md)
- [Completion Report](GUARDRAILS_COMPLETION_REPORT.md)
- [Sequential Execution Design](SEQUENTIAL_EXECUTION_DESIGN.md)
