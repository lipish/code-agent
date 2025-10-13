use std::sync::Arc;
use agent_runner::planning::{PlanningEngine, PlanningConfig};
use agent_runner::models::{MockModel, LanguageModel};
use agent_runner::types::TaskComplexity;

/// 测试代理商License管理系统的任务拆解
#[tokio::test]
async fn test_license_management_system_decomposition() {
    println!("\n🎯 测试场景1: 代理商License管理系统");
    println!("{}", "=".repeat(60));
    
    let model = Arc::new(MockModel::new("代理商License管理测试".to_string()));
    let config = PlanningConfig {
        verbose: true,
        max_retries: 1,
        auto_infer_type: true,
    };
    let engine = PlanningEngine::with_config(model, config);
    
    let task_description = r#"
为一家软件公司设计和实现一个代理商License管理系统。该系统需要支持：
1. 多级代理商层次结构管理
2. License的生成、分配、激活和吊销
3. 不同类型产品的License管理（试用版、标准版、企业版）
4. License使用情况的实时监控和报告
5. 代理商权限和配额管理
6. 自动续费和到期提醒功能
7. 安全的License验证机制
8. 支持离线License验证
该系统需要具备高安全性、可扩展性，并支持REST API接口。
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
            assert!(matches!(plan.complexity, TaskComplexity::Complex), 
                   "License管理系统应被识别为复杂任务");
                   
            println!("\n🎉 场景1测试完成 - License管理系统拆解有效");
        }
        Err(e) => {
            println!("❌ 任务拆解失败: {:?}", e);
            panic!("License管理系统测试失败");
        }
    }
}

/// 验证MockModel的响应质量
#[tokio::test] 
async fn test_mock_model_license_response() {
    println!("\n🔍 验证MockModel对License管理的响应");
    
    let model = MockModel::new("License管理测试".to_string());
    let prompt = "分析License管理系统需求";
    
    let response = model.complete(prompt).await.unwrap();
    println!("📝 模拟响应内容:");
    println!("{}", response.content);
    
    assert!(!response.content.is_empty(), "响应内容不应为空");
    assert!(response.content.contains("UNDERSTANDING") || 
           response.content.contains("understanding"), 
           "响应应包含理解部分");
}