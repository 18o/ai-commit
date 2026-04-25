use anyhow::Context;
use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::config::ApiConfig;

#[derive(Serialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<usize>,
    temperature: Option<f32>,
}

/// API 响应中的 message 对象。
/// `content` 使用 `#[serde(default)]` 以兼容 `null`（某些 API 在无内容时返回 null）。
/// `reasoning_content` 用于 DeepSeek R1 等推理模型，回答可能放在此字段。
#[derive(Deserialize, Debug)]
struct ChatMessage {
    #[serde(default)]
    content: String,
    reasoning_content: Option<String>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

pub struct AiClient {
    client: Client,
    config: ApiConfig,
    system_prompt: String,
    user_prompt_template: String,
}

impl AiClient {
    pub fn new(
        config: ApiConfig,
        system_prompt: String,
        user_prompt_template: String,
    ) -> anyhow::Result<Self> {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .build()
            .context("Failed to build HTTP client — TLS backend initialization error")?;
        Ok(AiClient { client, config, system_prompt, user_prompt_template })
    }

    pub async fn send_chat_request(&self, messages: Vec<Message>) -> anyhow::Result<String> {
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        debug!("Sending chat request to {}", self.config.endpoint);

        let response = self
            .client
            .post(&self.config.endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            debug!("API error response ({status}): {error_text}");
            anyhow::bail!("API request failed ({status}): {error_text}");
        }

        // 先获取原始响应文本，记录日志后再解析
        let response_text = response.text().await?;
        debug!("API raw response: {response_text}");

        let chat_response: ChatResponse = match serde_json::from_str(&response_text) {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to parse API response: {e}. Raw response: {response_text}");
                anyhow::bail!(
                    "Failed to parse API response: {e}. Run with RUST_LOG=debug to see the raw response."
                );
            }
        };

        let choice = chat_response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("API returned empty choices — the request may have been filtered"))?;

        let content = choice.message.content;
        let message = if content.trim().is_empty() {
            // DeepSeek R1 等推理模型可能把回答放在 reasoning_content，content 为空
            choice.message.reasoning_content.unwrap_or(content)
        } else {
            content
        };

        if message.trim().is_empty() {
            anyhow::bail!(
                "AI returned an empty response. This can happen with reasoning models or content filters."
            );
        }

        Ok(message)
    }

    pub async fn generate_commit_message(&self, diff: &str) -> anyhow::Result<String> {
        let system_message =
            Message { role: "system".to_string(), content: self.system_prompt.clone() };
        let user_content = self.user_prompt_template.replace("{diff}", diff);
        let user_message = Message { role: "user".to_string(), content: user_content };
        let messages = vec![system_message, user_message];
        debug!("Sending messages: {messages:?}");
        self.send_chat_request(messages).await
    }

    pub async fn generate_commit_message_with_keywords(
        &self,
        diff: &str,
        keywords: &str,
    ) -> anyhow::Result<String> {
        let system_message =
            Message { role: "system".to_string(), content: self.system_prompt.clone() };
        let user_content = format!(
            "Based on the following git diff, generate a commit message.\n\n\
             User provided keywords/context: {keywords}\n\n\
             Git diff:\n\
             ```diff\n\
             {diff}\n\
             ```\n\n\
             Please focus on the user's keywords/context when generating the commit message. \
             Provide only the commit message, no explanations or additional text."
        );
        let user_message = Message { role: "user".to_string(), content: user_content };
        let messages = vec![system_message, user_message];
        debug!("Sending messages: {messages:?}");
        self.send_chat_request(messages).await
    }
}
