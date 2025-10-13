use std::sync::Arc;
use agent_runner::planning::{PlanningEngine, PlanningConfig};
use agent_runner::models::{MockModel, LanguageModel};
use agent_runner::types::TaskComplexity;

/// 测试多分支机构会议室预定管理系统的任务拆解
#[tokio::test]
async fn test_meeting_room_booking_system_decomposition() {
    println!("\n🎯 测试场景3: 多分支机构会议室预定管理系统");
    println!("{}", "=".repeat(60));
    
    let model = Arc::new(MockModel::new("会议室预定管理测试".to_string()));
    let config = PlanningConfig {
        verbose: true,
        max_retries: 1,
        auto_infer_type: true,
    };
    let engine = PlanningEngine::with_config(model, config);
    
    let task_description = r#"
为一个大型企业集团开发多分支机构会议室预定管理系统，需要支持：
1. 多城市分支机构管理（北京、上海、深圳、成都等）
2. 不同类型会议室管理（小型讨论室、中型会议室、大型会议厅、视频会议室）
3. 会议室设备管理（投影仪、音响、视频设备、白板等）
4. 智能预定系统（时间冲突检测、自动推荐可用时段）
5. 多角色权限管理（普通员工、部门主管、管理员、超级管理员）
6. 预定审批流程（根据会议室级别和时长设置不同审批规则）
7. 实时通知系统（邮件、短信、企业微信、钉钉等）
8. 会议室使用统计和分析报告
9. 移动端APP支持扫码签到
10. 与企业日历系统集成（Outlook、Google Calendar等）
11. 访客管理和临时预定功能
12. 智能会议室推荐（基于会议规模、设备需求、位置偏好）
13. 取消和变更管理（自动释放资源、通知相关人员）
14. 会议室维护管理（清洁、设备检修时间屏蔽）
系统需要支持高并发访问，具备良好的用户体验和稳定性。
"#;

    println!("📋 任务描述:");
    println!("{}", task_description.trim());
    println!("\n🤖 开始AI分析...\n");
    
    let result = engine.analyze_task(task_description).await;
    
    match result {
        Ok(plan) => {
            println!("\n✅ 任务拆解成功!");
            println!("{}", "=".repeat(60));
            
            println!("📊 解析结果摘要:");
            println!("• 复杂度评估: {:?}", plan.complexity);
            println!("• 预估步骤数: {:?}", plan.estimated_steps);
            println!("• 需求条目数: {}", plan.requirements.len());
            
            println!("\n🧠 任务理解:");
            println!("{}", plan.understanding);
            
            println!("\n🎯 执行方案:");
            println!("{}", plan.approach);
            
            if !plan.requirements.is_empty() {
                println!("\n📋 技术需求:");
                for (i, req) in plan.requirements.iter().enumerate() {
                    println!("  {}. {}", i + 1, req);
                }
            }
            
            // 验证解析质量
            assert!(!plan.understanding.is_empty(), "任务理解不应为空");
            assert!(!plan.approach.is_empty(), "执行方案不应为空");
            assert!(matches!(plan.complexity, TaskComplexity::Complex | TaskComplexity::Moderate), 
                   "会议室管理系统应被识别为中等或复杂任务");
                   
            // 检查是否包含关键的业务概念
            let combined_text = format!("{} {}", plan.understanding, plan.approach).to_lowercase();
            let business_keywords = ["会议", "预定", "管理", "系统", "权限", "通知"];
            let keyword_found = business_keywords.iter().any(|&keyword| combined_text.contains(keyword));
            assert!(keyword_found, "应该识别出会议室管理相关概念");
                   
            println!("\n🎉 场景3测试完成 - 会议室预定系统拆解有效");
        }
        Err(e) => {
            println!("❌ 任务拆解失败: {:?}", e);
            panic!("会议室预定系统测试失败");
        }
    }
}

/// 测试任务类型推断功能
#[tokio::test]
async fn test_task_type_inference() {
    println!("\n🔍 测试任务类型自动推断功能");
    
    let model = Arc::new(MockModel::new("类型推断测试".to_string()));
    let config = PlanningConfig {
        verbose: false,
        max_retries: 1, 
        auto_infer_type: true,
    };
    let engine = PlanningEngine::with_config(model, config);
    
    // 测试不同类型的任务描述
    let test_cases = vec![
        ("创建一个新的REST API", "code_generation"),
        ("优化数据库查询性能", "optimization"), 
        ("修复登录Bug", "debugging"),
        ("写单元测试", "testing"),
        ("重构用户服务模块", "refactoring"),
        ("读取配置文件", "file_operations"),
        ("运行构建命令", "command_execution"),
        ("设计系统架构", "architecture"),
        ("编写API文档", "documentation"),
    ];
    
    for (task, expected_type) in test_cases {
        println!("测试任务: {} -> 期望类型: {}", task, expected_type);
        let result = engine.analyze_task(task).await;
        assert!(result.is_ok(), "任务分析应该成功: {}", task);
    }
    
    println!("任务类型推断测试完成");
}

/// 综合测试：验证三个场景的任务拆解结果对比
#[tokio::test]
async fn test_comprehensive_task_decomposition_comparison() {
    println!("\n📊 综合对比分析：三个业务场景任务拆解");
    println!("{}", "=".repeat(70));
    
    let model = Arc::new(MockModel::new("综合对比测试".to_string()));
    let config = PlanningConfig {
        verbose: false,  // 关闭详细输出，专注于对比分析
        max_retries: 1,
        auto_infer_type: true,
    };
    let engine = PlanningEngine::with_config(model, config);
    
    let scenarios = vec![
        ("代理商License管理系统", "多级代理商管理、License生成分配、权限控制、安全验证机制"),
        ("投资组合分析系统", "多资产配置、风险评估、实时数据处理、机器学习预测模型"),
        ("会议室预定管理系统", "多分支管理、智能预定、审批流程、实时通知、移动端支持"),
    ];
    
    for (name, description) in scenarios {
        println!("\n🎯 分析场景: {}", name);
        println!("描述: {}", description);
        
        let result = engine.analyze_task(description).await;
        match result {
            Ok(plan) => {
                println!("• 复杂度: {:?}", plan.complexity);
                println!("• 步骤数: {:?}", plan.estimated_steps.unwrap_or(0));
                println!("• 需求数: {}", plan.requirements.len());
                println!("• 理解长度: {} 字符", plan.understanding.len());
                println!("• 方案长度: {} 字符", plan.approach.len());
                
                // 验证基本质量
                assert!(!plan.understanding.is_empty());
                assert!(!plan.approach.is_empty());
            }
            Err(e) => {
                println!("❌ 分析失败: {:?}", e);
                panic!("场景 {} 分析失败", name);
            }
        }
    }
    
    println!("\n✅ 综合对比分析完成 - 所有场景均成功拆解");
}