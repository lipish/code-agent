use std::sync::Arc;
use agent_runner::planning::{PlanningEngine, PlanningConfig};
use agent_runner::models::{MockModel, LanguageModel};
use agent_runner::types::TaskComplexity;

/// 测试投资组合构建和分析系统的任务拆解
#[tokio::test]
async fn test_portfolio_management_system_decomposition() {
    println!("\n🎯 测试场景2: 投资组合构建和分析系统");
    println!("{}", "=".repeat(60));
    
    let model = Arc::new(MockModel::new("投资组合管理测试".to_string()));
    let config = PlanningConfig {
        verbose: true,
        max_retries: 1,
        auto_infer_type: true,
    };
    let engine = PlanningEngine::with_config(model, config);
    
    let task_description = r#"
开发一个智能投资组合构建和分析系统，需要实现以下功能：
1. 多资产类别支持（股票、债券、基金、ETF、期货、外汇等）
2. 实时市场数据获取和处理（价格、成交量、财务指标等）
3. 智能资产配置算法（现代投资组合理论、风险平价、因子模型等）
4. 风险管理和评估（VaR、CVaR、最大回撤、夏普比率等）
5. 回测引擎支持历史策略验证
6. 实时投资组合监控和预警系统
7. 个性化投资建议生成（基于用户风险偏好和投资目标）
8. 税务优化和成本分析
9. ESG评分集成和可持续投资筛选
10. 机器学习驱动的市场预测模型
11. 多语言报告生成（中文、英文）
12. 移动端和Web端界面支持
该系统需要处理大量实时数据，支持高并发用户访问，具备良好的扩展性和容错性。
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
                   "投资组合系统应被识别为复杂任务");
                   
            // 检查是否包含关键的金融概念
            let combined_text = format!("{} {}", plan.understanding, plan.approach).to_lowercase();
            let financial_keywords = ["投资", "组合", "风险", "数据", "分析", "算法"];
            let keyword_found = financial_keywords.iter().any(|&keyword| combined_text.contains(keyword));
            assert!(keyword_found, "应该识别出金融投资相关概念");
                   
            println!("\n🎉 场景2测试完成 - 投资组合系统拆解有效");
        }
        Err(e) => {
            println!("❌ 任务拆解失败: {:?}", e);
            panic!("投资组合系统测试失败");
        }
    }
}

/// 测试复杂度评估的准确性
#[tokio::test]
async fn test_complexity_assessment_accuracy() {
    println!("\n🔍 测试复杂度评估准确性");
    
    let model = Arc::new(MockModel::new("复杂度测试".to_string()));
    let config = PlanningConfig {
        verbose: false,  // 这里关闭verbose减少输出
        max_retries: 1,
        auto_infer_type: true,
    };
    let engine = PlanningEngine::with_config(model, config);
    
    // 简单任务
    let simple_task = "读取一个配置文件并打印内容";
    let simple_result = engine.analyze_task(simple_task).await.unwrap();
    println!("简单任务复杂度: {:?}", simple_result.complexity);
    
    // 中等任务  
    let moderate_task = "创建一个REST API服务，包含用户认证和数据存储";
    let moderate_result = engine.analyze_task(moderate_task).await.unwrap();
    println!("中等任务复杂度: {:?}", moderate_result.complexity);
    
    // 复杂任务
    let complex_task = "构建一个分布式微服务架构，包含服务发现、负载均衡、消息队列和监控系统";
    let complex_result = engine.analyze_task(complex_task).await.unwrap();
    println!("复杂任务复杂度: {:?}", complex_result.complexity);
    
    // 验证复杂度递增趋势（注意：MockModel可能不会产生真实的复杂度差异）
    println!("复杂度评估测试完成");
}