use anyhow::{anyhow, Context, Result};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPlan {
    pub tool: String,
    pub args: serde_json::Value,
}

pub async fn plan_with_llm(model: &str, prompt: &str, tool_schema: &str) -> Result<ToolPlan> {
    let base_url =
        std::env::var("LLM_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let api_key = std::env::var("LLM_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .context("LLM_API_KEY or OPENAI_API_KEY must be set")?;

    let client = Client::new();

    let messages = json!([
        {
            "role": "system",
            "content": format!(
                "You are a strict planner. Given a goal and the tool schema, reply ONLY with a single JSON object that matches this format: {{\"tool\": string, \"args\": object}}.\n\nTools schema (JSON):\n{}\n\nRules:\n- The tool name MUST be one of the provided tools.\n- The args MUST be a valid JSON object for that tool.\n- Do NOT include any extra commentary or code fences.\n- If the goal cannot be achieved with available tools, pick the closest and set minimal args.",
                tool_schema
            )
        },
        {
            "role": "user",
            "content": prompt
        }
    ]);

    let payload = json!({
        "model": model,
        "messages": messages,
        "response_format": { "type": "json_object" },
        "temperature": 0.0
    });

    let response = client
        .post(format!(
            "{}/chat/completions",
            base_url.trim_end_matches('/')
        ))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .context("Failed to send request to LLM API")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow!("LLM API error: {}", error_text));
    }

    let response_json: Value = response
        .json()
        .await
        .context("Failed to parse LLM response")?;
    let content = response_json
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .ok_or_else(|| anyhow!("Invalid LLM response format"))?;

    // Try direct JSON parse first
    if let Ok(plan) = serde_json::from_str::<ToolPlan>(content.trim()) {
        return Ok(plan);
    }

    // Fallback: extract first ```json ... ``` block
    let re = Regex::new(r"```json\s*(?P<j>\{[\s\S]*?\})\s*```")?;
    if let Some(caps) = re.captures(content) {
        let j = &caps["j"];
        let plan: ToolPlan =
            serde_json::from_str(j).context("failed to parse JSON from fenced block")?;
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

    Err(anyhow!(
        "LLM did not return a valid ToolPlan JSON: {}",
        content
    ))
}
