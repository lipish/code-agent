//! Guardrails System Demonstration
//! 
//! This example demonstrates the execution guardrails and safety mechanisms:
//! - Risk level assessment
//! - Dangerous pattern detection
//! - User confirmation requests
//! - Protected paths
//! - Rollback plans

use agent_runner::execution::{
    GuardrailEngine, GuardrailConfig, OperationType, OperationTarget, OperationRiskLevel,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️  Execution Guardrails Demonstration");
    println!("================================================================================\n");

    // Create a guardrail engine with default configuration
    let mut config = GuardrailConfig::default();
    config.auto_confirm_threshold = OperationRiskLevel::Low;
    config.show_operation_details = true;
    
    let engine = GuardrailEngine::new(config);

    // Demo 1: Safe operation - file read
    demo_safe_operation(&engine)?;
    
    // Demo 2: Low risk - file creation
    demo_low_risk_operation(&engine)?;
    
    // Demo 3: Medium risk - file modification
    demo_medium_risk_operation(&engine)?;
    
    // Demo 4: High risk - file deletion
    demo_high_risk_operation(&engine)?;
    
    // Demo 5: Critical risk - dangerous command
    demo_critical_risk_operation(&engine)?;
    
    // Demo 6: Protected path
    demo_protected_path(&engine)?;
    
    // Demo 7: Batch operation
    demo_batch_operation(&engine)?;

    Ok(())
}

fn demo_safe_operation(engine: &GuardrailEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Demo 1: Safe Operation - File Read");
    println!("────────────────────────────────────────");
    
    let guard = engine.check_operation(
        OperationType::FileRead,
        "读取配置文件 config.json",
        vec![OperationTarget {
            resource_type: "file".to_string(),
            path: "config.json".to_string(),
            is_protected: false,
            snapshot: None,
        }],
    )?;
    
    print_operation_guard(&guard);
    println!();
    Ok(())
}

fn demo_low_risk_operation(engine: &GuardrailEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Demo 2: Low Risk - File Creation");
    println!("────────────────────────────────────────");
    
    let guard = engine.check_operation(
        OperationType::FileCreate,
        "创建新文件 output.txt",
        vec![OperationTarget {
            resource_type: "file".to_string(),
            path: "output.txt".to_string(),
            is_protected: false,
            snapshot: None,
        }],
    )?;
    
    print_operation_guard(&guard);
    println!();
    Ok(())
}

fn demo_medium_risk_operation(engine: &GuardrailEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("✏️  Demo 3: Medium Risk - File Modification");
    println!("────────────────────────────────────────");
    
    let guard = engine.check_operation(
        OperationType::FileModify,
        "修改源文件 main.rs",
        vec![OperationTarget {
            resource_type: "file".to_string(),
            path: "src/main.rs".to_string(),
            is_protected: false,
            snapshot: None,
        }],
    )?;
    
    print_operation_guard(&guard);
    println!();
    Ok(())
}

fn demo_high_risk_operation(engine: &GuardrailEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("🗑️  Demo 4: High Risk - File Deletion");
    println!("────────────────────────────────────────");
    
    let guard = engine.check_operation(
        OperationType::FileDelete,
        "删除临时文件",
        vec![OperationTarget {
            resource_type: "file".to_string(),
            path: "/tmp/test_file.txt".to_string(),
            is_protected: false,
            snapshot: None,
        }],
    )?;
    
    print_operation_guard(&guard);
    println!();
    Ok(())
}

fn demo_critical_risk_operation(engine: &GuardrailEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚨 Demo 5: Critical Risk - Dangerous Command");
    println!("────────────────────────────────────────");
    
    let guard = engine.check_operation(
        OperationType::CommandDelete,
        "rm -rf /tmp/build_cache",
        vec![OperationTarget {
            resource_type: "directory".to_string(),
            path: "/tmp/build_cache".to_string(),
            is_protected: false,
            snapshot: None,
        }],
    )?;
    
    print_operation_guard(&guard);
    println!();
    Ok(())
}

fn demo_protected_path(engine: &GuardrailEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Demo 6: Protected Path");
    println!("────────────────────────────────────────");
    
    let guard = engine.check_operation(
        OperationType::ConfigModify,
        "修改环境变量文件",
        vec![OperationTarget {
            resource_type: "file".to_string(),
            path: ".env".to_string(),
            is_protected: true,
            snapshot: None,
        }],
    )?;
    
    print_operation_guard(&guard);
    println!();
    Ok(())
}

fn demo_batch_operation(engine: &GuardrailEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Demo 7: Batch Operation");
    println!("────────────────────────────────────────");
    
    // Create 15 file targets to exceed batch threshold
    let targets: Vec<OperationTarget> = (1..=15)
        .map(|i| OperationTarget {
            resource_type: "file".to_string(),
            path: format!("file_{}.rs", i),
            is_protected: false,
            snapshot: None,
        })
        .collect();
    
    let guard = engine.check_operation(
        OperationType::FileMassModify { count: 15 },
        "批量修改 15 个 Rust 源文件",
        targets,
    )?;
    
    print_operation_guard(&guard);
    println!();
    Ok(())
}

fn print_operation_guard(guard: &agent_runner::execution::OperationGuard) {
    println!("  操作ID: {}", guard.id);
    println!("  操作类型: {:?}", guard.operation_type);
    println!(
        "  风险级别: {} {}",
        guard.risk_level.emoji(),
        guard.risk_level.description()
    );
    println!("  操作描述: {}", guard.description);
    println!("  需要确认: {}", if guard.requires_confirmation { "✅ 是" } else { "❌ 否" });
    
    println!("\n  影响范围:");
    println!("    • 文件数量: {}", guard.expected_impact.affected_files);
    println!("    • 目录数量: {}", guard.expected_impact.affected_directories);
    println!("    • 代码行数: {}", guard.expected_impact.affected_lines);
    println!(
        "    • 可逆性: {}",
        if guard.expected_impact.reversible { "✅ 可逆" } else { "❌ 不可逆" }
    );
    println!("    • 预计时间: {} 秒", guard.expected_impact.estimated_duration);
    
    if !guard.targets.is_empty() {
        println!("\n  目标资源:");
        for (i, target) in guard.targets.iter().take(3).enumerate() {
            let protected = if target.is_protected { " 🔒" } else { "" };
            println!("    {}. {}{}", i + 1, target.path, protected);
        }
        if guard.targets.len() > 3 {
            println!("    ... 还有 {} 个资源", guard.targets.len() - 3);
        }
    }
    
    if !guard.detected_patterns.is_empty() {
        println!("\n  ⚠️  检测到的危险模式:");
        for pattern in &guard.detected_patterns {
            println!("    • {}: {}", pattern.name, pattern.warning_message);
        }
    }
    
    if let Some(rollback) = &guard.rollback_plan {
        println!("\n  回滚计划:");
        println!(
            "    • 自动回滚: {}",
            if rollback.auto_rollback { "✅ 启用" } else { "❌ 禁用" }
        );
        println!("    • 回滚步骤: {} 个", rollback.steps.len());
        println!("    • 回滚窗口: {} 秒", rollback.rollback_window_seconds);
    } else {
        println!("\n  回滚计划: ❌ 无法回滚（无快照）");
    }
    
    if guard.requires_confirmation {
        println!("\n  ⚠️  此操作需要用户确认才能执行");
    }
}
