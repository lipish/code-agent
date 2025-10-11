//! 综合领域任务分解能力测试
//! 测试不同类型和复杂程度的任务，包括金融投资、市场分析、商业策略等

mod common;

use tokio;
use task_runner::planning::PlanningEngine;
use task_runner::models::{LlmModel};
use std::sync::Arc;
use tracing_subscriber;

#[tokio::test]
async fn test_financial_investment_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("💰 金融投资分析任务测试");
    println!("======================");
    
    let engine = create_test_engine().await?;
    
    let financial_prompts = vec![
        (
            "初级投资组合",
            "我有10万元人民币，想要进行投资理财，风险承受能力中等，投资期限3-5年，请帮我制定一个投资组合策略"
        ),
        (
            "高级量化策略", 
            "基于机器学习和大数据分析，设计一个A股市场的量化交易策略，要求包含因子挖掘、风险控制、回测验证和实盘部署方案"
        ),
        (
            "企业估值分析",
            "对一家新能源汽车公司进行全面的企业估值分析，需要考虑DCF模型、相对估值法、实物期权价值，并给出投资建议"
        )
    ];
    
    for (task_name, prompt) in financial_prompts {
        println!("\n📊 金融任务: {}", task_name);
        test_task_analysis(&engine, prompt, task_name).await;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_market_business_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 市场商业分析任务测试");
    println!("========================");
    
    let engine = create_test_engine().await?;
    
    let business_prompts = vec![
        (
            "市场调研报告",
            "针对中国咖啡市场进行深度调研，分析市场规模、竞争格局、消费者行为、发展趋势，并提出市场进入策略"
        ),
        (
            "商业模式设计",
            "为一个面向Z世代的社交电商平台设计商业模式，包括用户获取、变现方式、运营策略、竞争壁垒构建"
        ),
        (
            "危机公关处理",
            "某知名品牌出现产品质量问题，需要制定一套完整的危机公关应对方案，包括媒体沟通、用户安抚、品牌修复"
        )
    ];
    
    for (task_name, prompt) in business_prompts {
        println!("\n🏢 商业任务: {}", task_name);
        test_task_analysis(&engine, prompt, task_name).await;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_creative_strategic_tasks() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 创意策略任务测试");
    println!("===================");
    
    let engine = create_test_engine().await?;
    
    let creative_prompts = vec![
        (
            "品牌营销策划",
            "为一个新推出的智能家居品牌策划一场跨平台整合营销活动，目标是在6个月内提升品牌知名度并实现销售突破"
        ),
        (
            "组织变革管理",
            "一家传统制造企业要进行数字化转型，需要制定详细的组织变革管理方案，包括文化重塑、人才培养、流程再造"
        ),
        (
            "可持续发展规划",
            "为一座中等规模城市制定碳中和目标的可持续发展规划，涵盖能源转型、交通优化、产业升级、生态建设"
        )
    ];
    
    for (task_name, prompt) in creative_prompts {
        println!("\n💡 创意任务: {}", task_name);
        test_task_analysis(&engine, prompt, task_name).await;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_scientific_research_tasks() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔬 科研学术任务测试");
    println!("===================");
    
    let engine = create_test_engine().await?;
    
    let research_prompts = vec![
        (
            "学术论文写作",
            "撰写一篇关于人工智能在医疗诊断中应用的综述论文，需要包含文献调研、现状分析、技术对比、未来展望"
        ),
        (
            "实验设计方案",
            "设计一个关于植物在不同光照条件下生长差异的对照实验，包括假设提出、变量控制、数据收集、结果分析"
        ),
        (
            "政策影响评估",
            "评估新能源汽车补贴政策对汽车产业发展的影响，需要建立评估模型、收集数据、进行定量分析"
        )
    ];
    
    for (task_name, prompt) in research_prompts {
        println!("\n📚 科研任务: {}", task_name);
        test_task_analysis(&engine, prompt, task_name).await;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_complexity_variations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚖️ 复杂度梯度测试");
    println!("==================");
    
    let engine = create_test_engine().await?;
    
    let complexity_prompts = vec![
        (
            "简单任务",
            "帮我写一份周末聚餐的购物清单"
        ),
        (
            "中等任务", 
            "制定一个为期3个月的个人健身计划，包括饮食搭配和运动安排"
        ),
        (
            "复杂任务",
            "为一家跨国公司设计全球供应链优化方案，需要考虑成本控制、风险管理、可持续发展、地缘政治影响"
        ),
        (
            "超复杂任务",
            "构建一个智慧城市的综合解决方案，涵盖交通系统、能源管理、环境监控、公共安全、政务服务、产业发展，并制定10年实施路线图"
        )
    ];
    
    for (task_name, prompt) in complexity_prompts {
        println!("\n📊 {} 测试:", task_name);
        test_task_analysis(&engine, prompt, task_name).await;
    }
    
    Ok(())
}

/// 创建测试引擎
async fn create_test_engine() -> Result<PlanningEngine, Box<dyn std::error::Error>> {
    // 初始化tracing日志系统
    let _ = tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
    
    let model_config = common::get_test_zhipu_config();
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    // 根据项目规范，使用verbose模式以便观察LLM行为
    let mut planning_config = task_runner::planning::PlanningConfig::default();
    planning_config.verbose = true;
    
    Ok(task_runner::planning::PlanningEngine::with_config(Arc::new(model), planning_config))
}

/// 执行任务分析并输出详细结果
async fn test_task_analysis(engine: &PlanningEngine, prompt: &str, task_name: &str) {
    println!("🎯 任务描述: {}", prompt);
    println!("{}", "-".repeat(80));
    
    match engine.analyze_task(prompt).await {
        Ok(plan) => {
            println!("✅ 任务分析完成!");
            println!();
            
            // 输出解析结果
            println!("📋 解析结果摘要:");
            println!("  📝 理解深度: {} 字符", plan.understanding.len());
            println!("  🛠️ 方法详细度: {} 字符", plan.approach.len());
            println!("  ⚖️ 复杂度评估: {:?}", plan.complexity);
            println!("  📊 预估步骤: {} 步", plan.estimated_steps.unwrap_or(0));
            println!("  📋 需求识别: {} 项", plan.requirements.len());
            
            // 领域特定分析
            analyze_domain_specific_concepts(&plan, task_name);
            
            // 内容质量分析
            analyze_content_quality(&plan);
            
        }
        Err(e) => {
            println!("❌ 任务分析失败: {}", e);
        }
    }
    
    println!("{}", "=".repeat(80));
}

/// 领域特定概念分析
fn analyze_domain_specific_concepts(plan: &task_runner::types::TaskPlan, task_type: &str) {
    let understanding = &plan.understanding.to_lowercase();
    let approach = &plan.approach.to_lowercase();
    let requirements_text = plan.requirements.join(" ").to_lowercase();
    let combined_text = format!("{} {} {}", understanding, approach, requirements_text);
    
    let detected_concepts = match task_type {
        name if name.contains("投资") || name.contains("金融") || name.contains("估值") => {
            vec![
                ("风险管理", combined_text.contains("风险") || combined_text.contains("risk")),
                ("投资分析", combined_text.contains("投资") || combined_text.contains("investment") || combined_text.contains("valuation") || combined_text.contains("估值")),
                ("财务模型", combined_text.contains("dcf") || combined_text.contains("模型") || combined_text.contains("model") || combined_text.contains("财务")),
                ("市场评估", combined_text.contains("市场") || combined_text.contains("market") || combined_text.contains("analysis") || combined_text.contains("分析")),
            ]
        },
        name if name.contains("市场") || name.contains("商业") || name.contains("危机") => {
            vec![
                ("策略规划", combined_text.contains("策略") || combined_text.contains("strategy") || combined_text.contains("plan")),
                ("品牌管理", combined_text.contains("品牌") || combined_text.contains("brand") || combined_text.contains("公关") || combined_text.contains("communication")),
                ("市场分析", combined_text.contains("市场") || combined_text.contains("market") || combined_text.contains("竞争") || combined_text.contains("competition")),
                ("客户关系", combined_text.contains("客户") || combined_text.contains("customer") || combined_text.contains("用户") || combined_text.contains("support")),
            ]
        },
        name if name.contains("创意") || name.contains("营销") || name.contains("变革") => {
            vec![
                ("创新设计", combined_text.contains("创意") || combined_text.contains("creative") || combined_text.contains("设计") || combined_text.contains("innovation")),
                ("数字化转型", combined_text.contains("数字化") || combined_text.contains("digital") || combined_text.contains("转型") || combined_text.contains("transformation")),
                ("组织管理", combined_text.contains("组织") || combined_text.contains("organization") || combined_text.contains("管理") || combined_text.contains("management")),
                ("可持续发展", combined_text.contains("可持续") || combined_text.contains("sustainable") || combined_text.contains("碳中和") || combined_text.contains("环境")),
            ]
        },
        name if name.contains("科研") || name.contains("学术") || name.contains("实验") => {
            vec![
                ("研究方法", combined_text.contains("研究") || combined_text.contains("research") || combined_text.contains("方法") || combined_text.contains("methodology")),
                ("数据分析", combined_text.contains("数据") || combined_text.contains("data") || combined_text.contains("分析") || combined_text.contains("analysis")),
                ("实验设计", combined_text.contains("实验") || combined_text.contains("experiment") || combined_text.contains("设计") || combined_text.contains("design")),
                ("学术写作", combined_text.contains("论文") || combined_text.contains("paper") || combined_text.contains("文献") || combined_text.contains("literature")),
            ]
        },
        _ => {
            vec![
                ("问题识别", combined_text.contains("问题") || combined_text.contains("problem") || combined_text.contains("issue")),
                ("解决方案", combined_text.contains("解决") || combined_text.contains("solution") || combined_text.contains("方案")),
                ("执行计划", combined_text.contains("计划") || combined_text.contains("plan") || combined_text.contains("步骤") || combined_text.contains("execution")),
                ("效果评估", combined_text.contains("评估") || combined_text.contains("evaluation") || combined_text.contains("assessment")),
            ]
        }
    };
    
    let detected_count = detected_concepts.iter().filter(|(_, detected)| *detected).count();
    let total_concepts = detected_concepts.len();
    
    println!("  🎯 领域概念识别: {}/{}", detected_count, total_concepts);
    for (concept, detected) in detected_concepts {
        println!("    {}: {}", concept, if detected { "✅" } else { "❌" });
    }
    
    // 添加泛化性评估
    let generalization_score = if detected_count as f32 / total_concepts as f32 >= 0.75 {
        "🎉 泛化性优秀"
    } else if detected_count >= 2 {
        "🔄 泛化性良好"
    } else {
        "⚠️ 泛化性需要改进"
    };
    
    println!("    {}", generalization_score);
}

/// 内容质量分析
fn analyze_content_quality(plan: &task_runner::types::TaskPlan) {
    let understanding_quality = match plan.understanding.len() {
        0..=50 => "简单",
        51..=150 => "中等", 
        151..=300 => "详细",
        _ => "非常详细"
    };
    
    let approach_quality = match plan.approach.len() {
        0..=50 => "简单",
        51..=150 => "中等",
        151..=300 => "详细", 
        _ => "非常详细"
    };
    
    let requirements_quality = match plan.requirements.len() {
        0..=2 => "基础",
        3..=6 => "充分",
        7..=12 => "详细",
        _ => "全面"
    };
    
    println!("  📊 内容质量评估:");
    println!("    理解深度: {} ({}字符)", understanding_quality, plan.understanding.len());
    println!("    方法详细度: {} ({}字符)", approach_quality, plan.approach.len());
    println!("    需求完整性: {} ({}项)", requirements_quality, plan.requirements.len());
    
    // 综合质量评分
    let quality_score = calculate_quality_score(plan);
    let quality_rating = match quality_score {
        0..=3 => "🔴 需要改进",
        4..=6 => "🟡 基本满足",
        7..=9 => "🟢 质量良好",
        _ => "🟢 质量优秀"
    };
    
    println!("    综合质量: {} (评分: {}/12)", quality_rating, quality_score);
}

/// 计算质量评分
fn calculate_quality_score(plan: &task_runner::types::TaskPlan) -> i32 {
    let mut score = 0;
    
    // 理解深度评分 (0-4分)
    score += match plan.understanding.len() {
        0..=50 => 1,
        51..=150 => 2,
        151..=300 => 3,
        _ => 4
    };
    
    // 方法详细度评分 (0-4分)
    score += match plan.approach.len() {
        0..=50 => 1,
        51..=150 => 2,
        151..=300 => 3,
        _ => 4
    };
    
    // 需求完整性评分 (0-4分)
    score += match plan.requirements.len() {
        0..=2 => 1,
        3..=6 => 2,
        7..=12 => 3,
        _ => 4
    };
    
    score
}