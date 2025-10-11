//! 复杂License管理项目的任务拆解测试

use tokio;
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};
use task_runner::planning::PlanningEngine;
use std::sync::Arc;

#[tokio::test]
async fn test_complex_license_project_planning() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️ 复杂License管理项目任务拆解测试");
    println!("=====================================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 4000,
        temperature: 0.6,
    };
    
    let complex_task = "创建一个 rust 项目，使用 Postgresql 数据库，有用户管理模型，可以登录注册 API 方式对外暴露接口。包括用户登录注册的接口 主要内容是一个 license 的管理，针对代理商，请完成这个项目";
    
    println!("🎯 复杂任务:");
    println!("{}", complex_task);
    println!();
    
    println!("🧠 开始任务分析...");
    
    // 首先直接调用LLM，观察原始的提示和响应
    println!("📤 发送给LLM的原始提示：");
    println!("{}", "═".repeat(80));
    
    // 获取Planning Engine实际使用的提示词
    let detailed_prompt = format!(
        "分析以下任务并提供详细的执行计划：\n\n任务：{}\n\n请按照以下格式回答：\nUNDERSTANDING: [你对任务的理解]\nAPPROACH: [解决这个任务的方法]\nCOMPLEXITY: [SIMPLE, MODERATE, 或 COMPLEX]\nREQUIREMENTS: [列出具体需求，每行一个]\n\n请用中文回答，并提供技术细节。", 
        complex_task
    );
    
    println!("{}", detailed_prompt);
    println!("{}", "═".repeat(80));
    println!();
    
    let start_time = std::time::Instant::now();
    
    // 直接调用LLM获取原始响应
    let llm_model = LlmModel::from_config(model_config.clone())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    println!("🚀 直接调用LLM...");
    match llm_model.complete(&detailed_prompt).await {
        Ok(response) => {
            println!("📥 LLM原始响应内容：");
            println!("{}", "═".repeat(80));
            println!("{}", response.content);
            println!("{}", "═".repeat(80));
            
            if let Some(usage) = &response.usage {
                println!("📈 Token使用情况：");
                println!("  输入token: {}", usage.prompt_tokens);
                println!("  输出token: {}", usage.completion_tokens);
                println!("  总token: {}", usage.total_tokens);
            }
            println!();
        }
        Err(e) => {
            println!("❌ LLM直接调用失败: {}", e);
        }
    }
    
    // 然后再用Planning Engine进行分析
    println!("🔍 现在用Planning Engine进行分析...");
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let planning_engine = PlanningEngine::new(Arc::new(model));
    
    match planning_engine.analyze_task(complex_task).await {
        Ok(plan) => {
            let duration = start_time.elapsed();
            
            println!("✅ 任务分析完成！耗时: {:?}", duration);
            println!();
            
            // 详细分析TaskPlan
            analyze_complex_task_plan(&plan);
            
            // 检查拆解的稳定性
            check_planning_stability(&plan);
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 任务分析失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[tokio::test]
async fn test_planning_consistency() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 任务拆解一致性测试");
    println!("=====================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 3000,
        temperature: 0.5, // 降低temperature以提高一致性
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let planning_engine = PlanningEngine::new(Arc::new(model));
    
    let task = "创建一个 rust 项目，使用 Postgresql 数据库，有用户管理模型，可以登录注册 API 方式对外暴露接口。包括用户登录注册的接口 主要内容是一个 license 的管理，针对代理商，请完成这个项目";
    
    println!("🔁 连续3次分析同一任务，检查一致性...");
    
    let mut plans = Vec::new();
    
    for i in 1..=3 {
        println!("📋 第{}次分析:", i);
        
        match planning_engine.analyze_task(task).await {
            Ok(plan) => {
                println!("  ✅ 分析成功");
                println!("  📊 复杂度: {:?}", plan.complexity);
                println!("  📝 步骤数: {:?}", plan.estimated_steps);
                println!("  📋 需求数: {}", plan.requirements.len());
                
                plans.push(plan);
            }
            Err(e) => {
                println!("  ❌ 分析失败: {}", e);
            }
        }
        println!();
    }
    
    if plans.len() >= 2 {
        compare_planning_consistency(&plans);
    }
    
    Ok(())
}

/// 分析复杂任务计划的质量
fn analyze_complex_task_plan(plan: &task_runner::types::TaskPlan) {
    println!("📊 TaskPlan详细分析:");
    println!("{}", "═".repeat(60));
    
    println!("🎯 任务理解:");
    println!("{}", plan.understanding);
    println!();
    
    println!("🛠️ 解决方法:");
    println!("{}", plan.approach);
    println!();
    
    println!("⚖️ 复杂度评估: {:?}", plan.complexity);
    
    if let Some(steps) = plan.estimated_steps {
        println!("📊 预估步骤数: {}", steps);
    }
    
    println!("📝 识别的需求数量: {}", plan.requirements.len());
    if !plan.requirements.is_empty() {
        println!("具体需求:");
        for (i, req) in plan.requirements.iter().enumerate() {
            println!("  {}. {}", i + 1, req);
        }
    }
    println!();
    
    // 分析内容质量
    analyze_content_quality(plan);
}

/// 分析内容质量
fn analyze_content_quality(plan: &task_runner::types::TaskPlan) {
    println!("🔍 内容质量分析:");
    println!("{}", "─".repeat(40));
    
    let understanding = &plan.understanding;
    let approach = &plan.approach;
    
    // 检查关键技术要素
    let has_rust = understanding.to_lowercase().contains("rust") || approach.to_lowercase().contains("rust");
    let has_postgresql = understanding.to_lowercase().contains("postgresql") || approach.to_lowercase().contains("postgresql");
    let has_api = understanding.to_lowercase().contains("api") || approach.to_lowercase().contains("api");
    let has_user_mgmt = understanding.contains("用户") || approach.contains("用户") || 
                       understanding.to_lowercase().contains("user") || approach.to_lowercase().contains("user");
    let has_license = understanding.contains("license") || approach.contains("license") ||
                     understanding.contains("许可") || approach.contains("许可");
    let has_agent = understanding.contains("代理商") || approach.contains("代理商") ||
                   understanding.to_lowercase().contains("agent") || approach.to_lowercase().contains("agent");
    
    println!("技术要素覆盖:");
    println!("  Rust项目: {}", if has_rust { "✅" } else { "❌" });
    println!("  PostgreSQL: {}", if has_postgresql { "✅" } else { "❌" });
    println!("  API接口: {}", if has_api { "✅" } else { "❌" });
    println!("  用户管理: {}", if has_user_mgmt { "✅" } else { "❌" });
    println!("  License管理: {}", if has_license { "✅" } else { "❌" });
    println!("  代理商功能: {}", if has_agent { "✅" } else { "❌" });
    
    let coverage_score = [has_rust, has_postgresql, has_api, has_user_mgmt, has_license, has_agent]
        .iter().filter(|&&x| x).count();
    
    println!();
    println!("📈 技术要素覆盖率: {}/6 ({:.1}%)", coverage_score, (coverage_score as f64 / 6.0) * 100.0);
    
    // 分析理解深度
    let understanding_depth = if understanding.len() > 200 { "深入" } 
                             else if understanding.len() > 100 { "中等" } 
                             else { "简单" };
    
    let approach_detail = if approach.len() > 200 { "详细" } 
                         else if approach.len() > 100 { "中等" } 
                         else { "简单" };
    
    println!("📝 理解深度: {} ({} 字符)", understanding_depth, understanding.len());
    println!("🛠️ 方法详细度: {} ({} 字符)", approach_detail, approach.len());
}

/// 检查规划稳定性
fn check_planning_stability(plan: &task_runner::types::TaskPlan) {
    println!("🔒 规划稳定性检查:");
    println!("{}", "─".repeat(30));
    
    // 检查是否有明确的复杂度评估
    let has_clear_complexity = matches!(plan.complexity, task_runner::types::TaskComplexity::Complex);
    println!("复杂度评估合理性: {}", if has_clear_complexity { "✅ 识别为复杂任务" } else { "⚠️ 可能低估复杂度" });
    
    // 检查步骤数是否合理
    let reasonable_steps = plan.estimated_steps.map_or(false, |steps| steps >= 8 && steps <= 20);
    println!("步骤数合理性: {}", if reasonable_steps { 
        "✅ 步骤数合理" 
    } else { 
        "⚠️ 步骤数可能不合理"
    });
    
    // 检查是否识别出足够的需求
    let sufficient_requirements = plan.requirements.len() >= 3;
    println!("需求识别充分性: {}", if sufficient_requirements { 
        "✅ 识别出多个需求" 
    } else { 
        "⚠️ 需求识别不足" 
    });
    
    // 综合稳定性评分
    let stability_score = [has_clear_complexity, reasonable_steps, sufficient_requirements]
        .iter().filter(|&&x| x).count();
    
    println!();
    println!("🎯 整体稳定性评分: {}/3", stability_score);
    
    match stability_score {
        3 => println!("🟢 规划质量优秀，拆解稳定"),
        2 => println!("🟡 规划质量良好，基本稳定"),
        1 => println!("🟠 规划质量一般，稳定性待改进"),
        0 => println!("🔴 规划质量较差，稳定性不足"),
        _ => {}
    }
}

/// 比较多次规划的一致性
fn compare_planning_consistency(plans: &[task_runner::types::TaskPlan]) {
    println!("📊 多次规划一致性分析:");
    println!("{}", "─".repeat(40));
    
    if plans.len() < 2 {
        println!("⚠️ 数据不足，无法进行一致性分析");
        return;
    }
    
    // 比较复杂度一致性
    let complexities: Vec<_> = plans.iter().map(|p| &p.complexity).collect();
    let complexity_consistent = complexities.windows(2).all(|w| w[0] == w[1]);
    println!("复杂度评估一致性: {}", if complexity_consistent { "✅" } else { "❌" });
    
    // 比较步骤数一致性
    let steps: Vec<_> = plans.iter().map(|p| p.estimated_steps).collect();
    let steps_variance = if steps.iter().all(|s| s.is_some()) {
        let step_values: Vec<u32> = steps.iter().filter_map(|&s| s).collect();
        let max_step = *step_values.iter().max().unwrap();
        let min_step = *step_values.iter().min().unwrap();
        max_step - min_step
    } else {
        u32::MAX
    };
    
    println!("步骤数一致性: {}", if steps_variance <= 2 { "✅" } else { "❌" });
    
    // 比较需求数量一致性
    let req_counts: Vec<_> = plans.iter().map(|p| p.requirements.len()).collect();
    let req_variance = req_counts.iter().max().unwrap() - req_counts.iter().min().unwrap();
    println!("需求识别一致性: {}", if req_variance <= 2 { "✅" } else { "❌" });
    
    // 计算整体一致性得分
    let consistency_score = [complexity_consistent, steps_variance <= 2, req_variance <= 2]
        .iter().filter(|&&x| x).count();
    
    println!();
    println!("🎯 整体一致性评分: {}/3", consistency_score);
    
    match consistency_score {
        3 => println!("🟢 规划高度一致，系统稳定"),
        2 => println!("🟡 规划基本一致，稳定性良好"),
        1 => println!("🟠 规划存在差异，稳定性一般"),
        0 => println!("🔴 规划差异较大，稳定性不足"),
        _ => {}
    }
}