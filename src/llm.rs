use anyhow::{anyhow, Context, Result};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ResponseFormat,
    },
    Client,
};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPlan {
    pub tool: String,
    pub args: serde_json::Value,
}

pub async fn plan_with_llm(model: &str, prompt: &str, tool_schema: &str) -> Result<ToolPlan> {
    // Prepare client; read optional custom base URL from env
    let client = if let Ok(base) = std::env::var("OPENAI_BASE_URL").or_else(|_| std::env::var("OPENAI_API_BASE")) {
        let cfg = OpenAIConfig::default().with_api_base(base);
        Client::with_config(cfg)
    } else {
        Client::new()
    };

    // System instructions: force strict JSON matching our schema
    let sys = ChatCompletionRequestSystemMessageArgs::default()
        .content(format!(
            "You are a strict planner. Given a goal and the tool schema, reply ONLY with a single JSON object that matches this format: {{\"tool\": string, \"args\": object}}.\n\nTools schema (JSON):\n{}\n\nRules:\n- The tool name MUST be one of the provided tools.\n- The args MUST be a valid JSON object for that tool.\n- Do NOT include any extra commentary or code fences.\n- If the goal cannot be achieved with available tools, pick the closest and set minimal args.",
            tool_schema
        ))
        .build()?;

    let user = ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()?;

    let req = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(vec![sys.into(), user.into()])
        .response_format(ResponseFormat::JsonObject)
        .temperature(0.0)
        .build()?;

    let resp = client
        .chat()
        .create(req)
        .await
        .context("chat.create failed")?;

    let content = resp
        .choices
        .get(0)
        .and_then(|c| c.message.content.clone())
        .ok_or_else(|| anyhow!("no content in response"))?;

    // Try direct JSON parse first
    if let Ok(plan) = serde_json::from_str::<ToolPlan>(content.trim()) {
        return Ok(plan);
    }

    // Fallback: extract first ```json ... ``` block
    let re = Regex::new(r"```json\s*(?P<j>\{[\s\S]*?\})\s*```")?;
    if let Some(caps) = re.captures(&content) {
        let j = &caps["j"];
        let plan: ToolPlan = serde_json::from_str(j).context("failed to parse JSON from fenced block")?;
        return Ok(plan);
    }

    // As a last resort, attempt to find the first balanced-looking JSON object
    if let Some(start) = content.find('{') {
        if let Some(end) = content.rfind('}') {
            let slice = &content[start..=end];
            if let Ok(plan) = serde_json::from_str::<ToolPlan>(slice) {
                return Ok(plan);
            }
        }
    }

    Err(anyhow!("LLM did not return a valid ToolPlan JSON: {}", content))
}
