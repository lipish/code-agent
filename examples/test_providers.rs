//! 测试 llm-connector 0.2.3 与各提供商的连接

use llm_connector::{LlmClient, ChatRequest, Message};
use std::fs;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("🧪 测试 llm-connector 0.2.3");
    println!("{}", "=".repeat(80));
    println!();

    let keys = load_keys()?;
    println!("✅ 加载了 {} 个 API keys", keys.len());
    println!();

    let mut success_count = 0;
    let mut fail_count = 0;

    // 测试 DeepSeek
    if let Some(deepseek_key) = keys.get("deepseek") {
        println!("📝 测试 DeepSeek");
        println!("{}", "-".repeat(80));

        let client = LlmClient::openai_compatible(
            deepseek_key,
            "https://api.deepseek.com/v1"
        );

        let request = ChatRequest {
            model: "deepseek-chat".to_string(),
            messages: vec![Message::user("用一句话介绍 Rust 编程语言")],
            ..Default::default()
        };

        match client.chat(&request).await {
            Ok(response) => {
                println!("✅ DeepSeek 成功!");
                if let Some(choice) = response.choices.first() {
                    println!("   模型: deepseek-chat");
                    println!("   响应: {}", truncate(&choice.message.content, 150));
                    if let Some(usage) = &response.usage {
                        println!("   Token: {} 输入 + {} 输出 = {} 总计",
                                usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
                    }
                }
                success_count += 1;
            }
            Err(e) => {
                println!("❌ DeepSeek 失败: {}", e);
                fail_count += 1;
            }
        }
        println!();
    }

    // 测试 Zhipu
    if let Some(zhipu_key) = keys.get("zhipu") {
        println!("📝 测试 Zhipu GLM-4");
        println!("{}", "-".repeat(80));

        let client = LlmClient::openai_compatible(
            zhipu_key,
            "https://open.bigmodel.cn/api/paas/v4"
        );

        let request = ChatRequest {
            model: "glm-4-flash".to_string(),
            messages: vec![Message::user("用一句话介绍 Rust 编程语言")],
            ..Default::default()
        };

        match client.chat(&request).await {
            Ok(response) => {
                println!("✅ Zhipu 成功!");
                if let Some(choice) = response.choices.first() {
                    println!("   模型: glm-4-flash");
                    println!("   响应: {}", truncate(&choice.message.content, 150));
                    if let Some(usage) = &response.usage {
                        println!("   Token: {} 输入 + {} 输出 = {} 总计",
                                usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
                    }
                }
                success_count += 1;
            }
            Err(e) => {
                println!("❌ Zhipu 失败: {}", e);
                fail_count += 1;
            }
        }
        println!();
    }

    // 测试 Moonshot
    if let Some(moonshot_key) = keys.get("moonshot") {
        println!("📝 测试 Moonshot");
        println!("{}", "-".repeat(80));

        let client = LlmClient::openai_compatible(
            moonshot_key,
            "https://api.moonshot.cn/v1"
        );

        let request = ChatRequest {
            model: "moonshot-v1-8k".to_string(),
            messages: vec![Message::user("用一句话介绍 Rust 编程语言")],
            ..Default::default()
        };

        match client.chat(&request).await {
            Ok(response) => {
                println!("✅ Moonshot 成功!");
                if let Some(choice) = response.choices.first() {
                    println!("   模型: moonshot-v1-8k");
                    println!("   响应: {}", truncate(&choice.message.content, 150));
                    if let Some(usage) = &response.usage {
                        println!("   Token: {} 输入 + {} 输出 = {} 总计",
                                usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
                    }
                }
                success_count += 1;
            }
            Err(e) => {
                println!("❌ Moonshot 失败: {}", e);
                fail_count += 1;
            }
        }
        println!();
    }

    // 测试 Aliyun
    if let Some(aliyun_key) = keys.get("aliyun") {
        println!("📝 测试 Aliyun Qwen");
        println!("{}", "-".repeat(80));

        let client = LlmClient::aliyun(aliyun_key);

        let request = ChatRequest {
            model: "qwen-turbo".to_string(),
            messages: vec![Message::user("用一句话介绍 Rust 编程语言")],
            ..Default::default()
        };

        match client.chat(&request).await {
            Ok(response) => {
                println!("✅ Aliyun 成功!");
                if let Some(choice) = response.choices.first() {
                    println!("   模型: qwen-turbo");
                    println!("   响应: {}", truncate(&choice.message.content, 150));
                    if let Some(usage) = &response.usage {
                        println!("   Token: {} 输入 + {} 输出 = {} 总计",
                                usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
                    }
                }
                success_count += 1;
            }
            Err(e) => {
                println!("❌ Aliyun 失败: {}", e);
                fail_count += 1;
            }
        }
        println!();
    }

    // 测试 LongCat
    if let Some(longcat_key) = keys.get("longcat") {
        println!("📝 测试 LongCat");
        println!("{}", "-".repeat(80));

        let client = LlmClient::openai_compatible(
            longcat_key,
            "https://api.longcat.chat/openai"
        );

        let request = ChatRequest {
            model: "LongCat-Flash-Chat".to_string(),
            messages: vec![Message::user("用一句话介绍 Rust 编程语言")],
            ..Default::default()
        };

        match client.chat(&request).await {
            Ok(response) => {
                println!("✅ LongCat 成功!");
                if let Some(choice) = response.choices.first() {
                    println!("   模型: LongCat-Flash-Chat");
                    println!("   响应: {}", truncate(&choice.message.content, 150));
                    if let Some(usage) = &response.usage {
                        println!("   Token: {} 输入 + {} 输出 = {} 总计",
                                usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
                    }
                }
                success_count += 1;
            }
            Err(e) => {
                println!("❌ LongCat 失败: {}", e);
                fail_count += 1;
            }
        }
        println!();
    }

    // 总结
    println!("{}", "=".repeat(80));
    println!("📊 测试总结:");
    println!("   ✅ 成功: {}", success_count);
    println!("   ❌ 失败: {}", fail_count);
    if success_count + fail_count > 0 {
        println!("   📈 成功率: {:.1}%", (success_count as f64 / (success_count + fail_count) as f64) * 100.0);
    }
    println!();

    if fail_count > 0 {
        println!("⚠️  部分测试失败");
        std::process::exit(1);
    } else {
        println!("🎉 所有测试通过! llm-connector 0.2.3 工作正常!");
    }

    Ok(())
}

fn load_keys() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    use serde_yaml::Value;

    let content = fs::read_to_string("keys.yaml")?;
    let yaml: Value = serde_yaml::from_str(&content)?;

    let mut keys = HashMap::new();

    if let Some(providers) = yaml.get("providers").and_then(|v| v.as_mapping()) {
        for (provider_name, provider_config) in providers {
            if let (Some(name), Some(config)) = (provider_name.as_str(), provider_config.as_mapping()) {
                if let Some(api_key) = config.get(&Value::String("api_key".to_string())).and_then(|v| v.as_str()) {
                    keys.insert(name.to_string(), api_key.to_string());
                }
            }
        }
    }

    Ok(keys)
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len).collect();
        format!("{}...", truncated)
    }
}

