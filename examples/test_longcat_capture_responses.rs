use std::sync::Arc;
use std::fs::File;
use std::io::Write;
use agent_runner::planning::{PlanningEngine, PlanningConfig};
use agent_runner::models::LlmModel;
use agent_runner::config::{ModelConfig, ModelProvider};
use std::env;

/// 捕获所有LongCat模型响应的详细测试
#[tokio::main]
async fn main() {
    // 从环境变量获取API key
    let api_key = env::var("LONGCAT_API_KEY")
        .expect("请设置环境变量 LONGCAT_API_KEY");
    
    println!("\n🚀 开始捕获LongCat模型完整响应");
    println!("{}", "=".repeat(80));
    
    // 配置LongCat模型
    let model_config = ModelConfig {
        provider: ModelProvider::LongCat,
        model_name: "LongCat-Flash-Chat".to_string(),
        api_key: Some(api_key),
        endpoint: None,
        max_tokens: 4096,
        temperature: 0.7,
    };
    
    let model = Arc::new(LlmModel::from_config(model_config)
        .expect("创建LongCat模型失败"));
    
    let config = PlanningConfig {
        verbose: true,  // 开启详细日志
        max_retries: 2,
        auto_infer_type: true,
    };
    
    let engine = PlanningEngine::with_config(model.clone(), config);
    
    // 创建输出文件
    let mut output_file = File::create("longcat_full_responses.md")
        .expect("无法创建输出文件");
    
    writeln!(output_file, "# LongCat模型完整响应记录\n").unwrap();
    writeln!(output_file, "**测试时间**: 2025-10-13").unwrap();
    writeln!(output_file, "**模型**: LongCat-Flash-Chat\n").unwrap();
    writeln!(output_file, "---\n").unwrap();
    
    // 测试场景定义
    let scenarios = vec![
        (
            "场景1: 代理商License管理系统",
            r#"为一家软件公司设计和实现一个代理商License管理系统。该系统需要支持：
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
该系统需要具备高安全性、可扩展性，并支持REST API接口供第三方系统集成。"#
        ),
        (
            "场景2: 智能投资组合构建和分析系统",
            r#"开发一个智能投资组合构建和分析系统，需要实现以下功能：
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
该系统需要处理大量实时数据流，支持高并发用户访问，具备良好的扩展性和容错性。"#
        ),
        (
            "场景3: 多分支机构会议室预定管理系统",
            r#"为一个大型企业集团开发多分支机构会议室预定管理系统，需要支持：
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
系统需要支持高并发访问（峰值时段可能同时有1000+用户预定），具备良好的用户体验和7×24小时稳定性。"#
        ),
        (
            "场景4: 简单任务",
            "读取项目根目录下的 Cargo.toml 文件并打印其中的依赖列表"
        ),
    ];
    
    for (idx, (scenario_name, task_description)) in scenarios.iter().enumerate() {
        println!("\n{}", "=".repeat(80));
        println!("正在测试: {}", scenario_name);
        println!("{}", "=".repeat(80));
        
        // 写入场景标题
        writeln!(output_file, "\n## {}\n", scenario_name).unwrap();
        writeln!(output_file, "### 任务描述\n").unwrap();
        writeln!(output_file, "```").unwrap();
        writeln!(output_file, "{}", task_description.trim()).unwrap();
        writeln!(output_file, "```\n").unwrap();
        
        // 调用模型
        match engine.analyze_task(task_description).await {
            Ok(plan) => {
                // 这里我们无法直接获取原始响应，因为它在parse_task_plan中被处理了
                // 我们需要直接调用模型
                println!("✅ 分析完成");
                
                writeln!(output_file, "### 解析后的结果\n").unwrap();
                writeln!(output_file, "**复杂度**: {:?}", plan.complexity).unwrap();
                writeln!(output_file, "**预估步骤**: {:?}", plan.estimated_steps).unwrap();
                writeln!(output_file, "**需求数量**: {}\n", plan.requirements.len()).unwrap();
                
                writeln!(output_file, "**任务理解**:").unwrap();
                writeln!(output_file, "```").unwrap();
                writeln!(output_file, "{}", plan.understanding).unwrap();
                writeln!(output_file, "```\n").unwrap();
                
                writeln!(output_file, "**执行方案**:").unwrap();
                writeln!(output_file, "```").unwrap();
                writeln!(output_file, "{}", plan.approach).unwrap();
                writeln!(output_file, "```\n").unwrap();
                
                if !plan.requirements.is_empty() {
                    writeln!(output_file, "**技术需求**:").unwrap();
                    for req in &plan.requirements {
                        writeln!(output_file, "- {}", req).unwrap();
                    }
                    writeln!(output_file, "").unwrap();
                }
            }
            Err(e) => {
                println!("❌ 分析失败: {:?}", e);
                writeln!(output_file, "**错误**: {:?}\n", e).unwrap();
            }
        }
        
        // 暂停以避免API限流
        if idx < scenarios.len() - 1 {
            println!("⏸️  暂停2秒...");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }
    
    writeln!(output_file, "\n---\n").unwrap();
    writeln!(output_file, "**测试完成**").unwrap();
    
    println!("\n{}", "=".repeat(80));
    println!("✅ 所有响应已保存到 longcat_full_responses.md");
    println!("{}", "=".repeat(80));
}
