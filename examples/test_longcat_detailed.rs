use std::sync::Arc;
use agent_runner::planning::{PlanningEngine, PlanningConfig};
use agent_runner::models::LlmModel;
use agent_runner::config::{ModelConfig, ModelProvider};
use agent_runner::types::TaskComplexity;
use std::env;

/// 详细测试LongCat模型对三个复杂业务场景的解析能力
#[tokio::main]
async fn main() {
    // 从环境变量获取API key
    let api_key = env::var("LONGCAT_API_KEY")
        .expect("请设置环境变量 LONGCAT_API_KEY");
    
    println!("\n🚀 开始LongCat模型详细测试");
    println!("{}", "=".repeat(80));
    println!("模型: LongCat-Flash-Chat");
    println!("提供商: LongCat");
    println!("{}", "=".repeat(80));
    
    // 配置LongCat模型
    let model_config = ModelConfig {
        provider: ModelProvider::LongCat,
        model_name: "LongCat-Flash-Chat".to_string(),
        api_key: Some(api_key),
        endpoint: None, // 使用默认endpoint
        max_tokens: 4096,
        temperature: 0.7,
    };
    
    let model = Arc::new(LlmModel::from_config(model_config)
        .expect("创建LongCat模型失败"));
    
    let config = PlanningConfig {
        verbose: true,
        max_retries: 2,
        auto_infer_type: true,
    };
    
    let engine = PlanningEngine::with_config(model.clone(), config);
    
    // 测试场景1: 代理商License管理系统
    test_license_management(&engine).await;
    
    println!("\n{}", "=".repeat(80));
    println!("⏸️  暂停2秒后继续下一个测试...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // 测试场景2: 投资组合分析系统
    test_portfolio_management(&engine).await;
    
    println!("\n{}", "=".repeat(80));
    println!("⏸️  暂停2秒后继续下一个测试...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // 测试场景3: 会议室预定管理系统
    test_meeting_room_booking(&engine).await;
    
    println!("\n{}", "=".repeat(80));
    println!("⏸️  暂停2秒后继续下一个测试...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // 测试场景4: 简单任务对比
    test_simple_task(&engine).await;
    
    println!("\n{}", "=".repeat(80));
    println!("✅ 所有测试完成!");
    println!("{}", "=".repeat(80));
}

async fn test_license_management(engine: &PlanningEngine) {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                    测试场景1: 代理商License管理系统                          ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    
    let task_description = r#"
为一家软件公司设计和实现一个代理商License管理系统。该系统需要支持：
1. 多级代理商层次结构管理（总代理、区域代理、经销商三级体系）
2. License的生成、分配、激活和吊销功能
3. 不同类型产品的License管理（试用版30天、标准版永久、企业版按年订阅）
4. License使用情况的实时监控和报告（激活设备数、使用时长、功能模块使用情况）
5. 代理商权限和配额管理（每个代理商可分配的License数量限制）
6. 自动续费和到期提醒功能（邮件、短信、系统通知）
7. 安全的License验证机制（RSA加密、离线验证、防篡改）
8. 支持离线License验证（适用于无网络环境）
9. License批量导入导出功能
10. 完整的操作日志和审计跟踪
该系统需要具备高安全性、可扩展性，并支持REST API接口供第三方系统集成。
"#;

    print_task_description(task_description);
    
    match engine.analyze_task(task_description).await {
        Ok(plan) => {
            print_detailed_analysis(&plan, "License管理系统");
            validate_license_system_plan(&plan);
        }
        Err(e) => {
            println!("❌ 任务分析失败: {:?}", e);
        }
    }
}

async fn test_portfolio_management(engine: &PlanningEngine) {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                  测试场景2: 智能投资组合构建和分析系统                       ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    
    let task_description = r#"
开发一个智能投资组合构建和分析系统，需要实现以下功能：
1. 多资产类别支持（股票、债券、基金、ETF、期货、外汇、商品、REITs等）
2. 实时市场数据获取和处理（价格、成交量、财务指标、宏观经济数据）
3. 智能资产配置算法（现代投资组合理论MPT、风险平价、Black-Litterman模型、因子模型等）
4. 全面风险管理和评估（VaR、CVaR、最大回撤、夏普比率、索提诺比率、信息比率等）
5. 强大的回测引擎支持历史策略验证（支持多因子回测、滑点成本模拟）
6. 实时投资组合监控和智能预警系统（偏离目标配置、风险阈值突破等）
7. 个性化投资建议生成（基于用户风险偏好、投资目标、时间期限）
8. 税务优化和交易成本分析（税收损失收割、换手率优化）
9. ESG评分集成和可持续投资筛选
10. 机器学习驱动的市场预测模型（LSTM、Transformer、强化学习等）
11. 多语言报告生成（中文、英文、PDF、Excel格式）
12. 移动端和Web端界面支持，数据可视化（交互式图表、热力图、相关性矩阵）
13. 支持组合压力测试和情景分析
14. 与券商交易系统对接实现自动化交易
该系统需要处理大量实时数据流，支持高并发用户访问，具备良好的扩展性和容错性。
"#;

    print_task_description(task_description);
    
    match engine.analyze_task(task_description).await {
        Ok(plan) => {
            print_detailed_analysis(&plan, "投资组合分析系统");
            validate_portfolio_system_plan(&plan);
        }
        Err(e) => {
            println!("❌ 任务分析失败: {:?}", e);
        }
    }
}

async fn test_meeting_room_booking(engine: &PlanningEngine) {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                测试场景3: 多分支机构会议室预定管理系统                       ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    
    let task_description = r#"
为一个大型企业集团开发多分支机构会议室预定管理系统，需要支持：
1. 多城市分支机构管理（北京、上海、深圳、成都、广州、杭州等20+城市）
2. 不同类型会议室管理（小型讨论室2-4人、中型会议室5-10人、大型会议厅20-100人、视频会议室、董事会会议室）
3. 会议室设备管理（投影仪、音响系统、视频设备、电子白板、茶水服务、同传设备等）
4. 智能预定系统（时间冲突检测、自动推荐可用时段、循环预定、代理预定）
5. 多角色权限管理（普通员工、部门主管、分公司管理员、集团管理员、超级管理员）
6. 灵活的预定审批流程（根据会议室级别、使用时长、跨部门会议等设置不同审批规则）
7. 实时通知系统（邮件、短信、企业微信、钉钉、飞书等多渠道）
8. 会议室使用统计和分析报告（使用率、热门时段、部门使用情况、成本分析）
9. 移动端APP支持扫码签到、提前/延迟结束会议
10. 与企业日历系统集成（Outlook、Google Calendar、Exchange等）
11. 访客管理和临时预定功能（访客邀请、安保通知、访客通行证打印）
12. 智能会议室推荐（基于会议规模、设备需求、位置偏好、历史偏好学习）
13. 取消和变更管理（自动释放资源、通知相关人员、取消罚分机制）
14. 会议室维护管理（清洁时间、设备检修时间屏蔽、故障报修）
15. 能耗监控和绿色办公（会议室用电统计、无人自动关闭设备）
16. 会议记录和纪要管理（会议录音、会议纪要上传、参会人员确认）
系统需要支持高并发访问（峰值时段可能同时有1000+用户预定），具备良好的用户体验和7×24小时稳定性。
"#;

    print_task_description(task_description);
    
    match engine.analyze_task(task_description).await {
        Ok(plan) => {
            print_detailed_analysis(&plan, "会议室预定系统");
            validate_meeting_room_plan(&plan);
        }
        Err(e) => {
            println!("❌ 任务分析失败: {:?}", e);
        }
    }
}

async fn test_simple_task(engine: &PlanningEngine) {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                     测试场景4: 简单任务（对比测试）                          ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    
    let task_description = "读取项目根目录下的 Cargo.toml 文件并打印其中的依赖列表";

    print_task_description(task_description);
    
    match engine.analyze_task(task_description).await {
        Ok(plan) => {
            print_detailed_analysis(&plan, "简单文件读取");
            
            // 验证简单任务应该被识别为Simple复杂度
            if matches!(plan.complexity, TaskComplexity::Simple) {
                println!("✅ 复杂度评估正确：简单任务被识别为 Simple");
            } else {
                println!("⚠️  复杂度评估可能不准确：简单任务被识别为 {:?}", plan.complexity);
            }
        }
        Err(e) => {
            println!("❌ 任务分析失败: {:?}", e);
        }
    }
}

// 辅助函数：打印任务描述
fn print_task_description(description: &str) {
    println!("\n📋 任务描述:");
    println!("{}", "─".repeat(80));
    println!("{}", description.trim());
    println!("{}", "─".repeat(80));
    println!("\n🤖 正在调用LongCat API进行分析...\n");
}

// 辅助函数：打印详细分析结果
fn print_detailed_analysis(plan: &agent_runner::types::TaskPlan, scenario_name: &str) {
    println!("\n✅ {} 分析完成!", scenario_name);
    println!("{}", "═".repeat(80));
    
    // 基本信息摘要
    println!("\n📊 【解析结果摘要】");
    println!("{}", "─".repeat(80));
    println!("  • 复杂度评估:     {:?}", plan.complexity);
    println!("  • 预估步骤数:     {}", plan.estimated_steps.unwrap_or(0));
    println!("  • 需求条目数:     {}", plan.requirements.len());
    println!("  • 理解文本长度:   {} 字符", plan.understanding.len());
    println!("  • 方案文本长度:   {} 字符", plan.approach.len());
    
    // 任务理解部分
    println!("\n🧠 【任务理解】");
    println!("{}", "─".repeat(80));
    print_wrapped_text(&plan.understanding, 76);
    
    // 执行方案部分
    println!("\n🎯 【执行方案】");
    println!("{}", "─".repeat(80));
    print_wrapped_text(&plan.approach, 76);
    
    // 技术需求部分
    if !plan.requirements.is_empty() {
        println!("\n📋 【技术需求清单】");
        println!("{}", "─".repeat(80));
        for (i, req) in plan.requirements.iter().enumerate() {
            println!("  {}. {}", i + 1, req);
        }
    } else {
        println!("\n📋 【技术需求清单】");
        println!("{}", "─".repeat(80));
        println!("  (未识别到具体技术需求)");
    }
    
    // 结构化步骤（如果有）
    if let Some(steps) = &plan.structured_steps {
        if !steps.is_empty() {
            println!("\n📝 【结构化执行步骤】");
            println!("{}", "─".repeat(80));
            for step in steps {
                println!("  • {:?}", step);
            }
        }
    }
    
    println!("\n{}", "═".repeat(80));
}

// 辅助函数：打印换行文本
fn print_wrapped_text(text: &str, width: usize) {
    let mut current_line = String::new();
    let mut current_width = 0;
    
    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        
        if current_width + word_len + 1 > width && !current_line.is_empty() {
            println!("  {}", current_line);
            current_line.clear();
            current_width = 0;
        }
        
        if !current_line.is_empty() {
            current_line.push(' ');
            current_width += 1;
        }
        
        current_line.push_str(word);
        current_width += word_len;
    }
    
    if !current_line.is_empty() {
        println!("  {}", current_line);
    }
}

// 验证函数：License管理系统
fn validate_license_system_plan(plan: &agent_runner::types::TaskPlan) {
    println!("\n🔍 【验证分析质量】");
    println!("{}", "─".repeat(80));
    
    let combined_text = format!("{} {}", plan.understanding.to_lowercase(), plan.approach.to_lowercase());
    
    let key_concepts = vec![
        ("代理商", "代理商管理"),
        ("license", "License机制"),
        ("加密", "安全加密"),
        ("验证", "验证机制"),
        ("api", "API接口"),
    ];
    
    let mut found_count = 0;
    for (keyword, concept) in &key_concepts {
        if combined_text.contains(keyword) {
            println!("  ✓ 识别到关键概念: {}", concept);
            found_count += 1;
        } else {
            println!("  ✗ 未识别到: {}", concept);
        }
    }
    
    println!("\n  关键概念覆盖率: {}/{} ({:.0}%)", 
             found_count, key_concepts.len(), 
             (found_count as f32 / key_concepts.len() as f32) * 100.0);
    
    if found_count >= 3 {
        println!("  ✅ 分析质量: 良好");
    } else {
        println!("  ⚠️  分析质量: 需要改进");
    }
}

// 验证函数：投资组合系统
fn validate_portfolio_system_plan(plan: &agent_runner::types::TaskPlan) {
    println!("\n🔍 【验证分析质量】");
    println!("{}", "─".repeat(80));
    
    let combined_text = format!("{} {}", plan.understanding.to_lowercase(), plan.approach.to_lowercase());
    
    let key_concepts = vec![
        ("投资", "投资管理"),
        ("风险", "风险评估"),
        ("数据", "数据处理"),
        ("算法", "算法模型"),
        ("分析", "数据分析"),
    ];
    
    let mut found_count = 0;
    for (keyword, concept) in &key_concepts {
        if combined_text.contains(keyword) {
            println!("  ✓ 识别到关键概念: {}", concept);
            found_count += 1;
        } else {
            println!("  ✗ 未识别到: {}", concept);
        }
    }
    
    println!("\n  关键概念覆盖率: {}/{} ({:.0}%)", 
             found_count, key_concepts.len(), 
             (found_count as f32 / key_concepts.len() as f32) * 100.0);
    
    if found_count >= 3 {
        println!("  ✅ 分析质量: 良好");
    } else {
        println!("  ⚠️  分析质量: 需要改进");
    }
}

// 验证函数：会议室预定系统
fn validate_meeting_room_plan(plan: &agent_runner::types::TaskPlan) {
    println!("\n🔍 【验证分析质量】");
    println!("{}", "─".repeat(80));
    
    let combined_text = format!("{} {}", plan.understanding.to_lowercase(), plan.approach.to_lowercase());
    
    let key_concepts = vec![
        ("会议", "会议管理"),
        ("预定", "预定系统"),
        ("权限", "权限控制"),
        ("通知", "通知系统"),
        ("审批", "审批流程"),
    ];
    
    let mut found_count = 0;
    for (keyword, concept) in &key_concepts {
        if combined_text.contains(keyword) {
            println!("  ✓ 识别到关键概念: {}", concept);
            found_count += 1;
        } else {
            println!("  ✗ 未识别到: {}", concept);
        }
    }
    
    println!("\n  关键概念覆盖率: {}/{} ({:.0}%)", 
             found_count, key_concepts.len(), 
             (found_count as f32 / key_concepts.len() as f32) * 100.0);
    
    if found_count >= 3 {
        println!("  ✅ 分析质量: 良好");
    } else {
        println!("  ⚠️  分析质量: 需要改进");
    }
}
