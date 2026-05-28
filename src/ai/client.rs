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
struct ThinkingConfig {
    r#type: String,
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<usize>,
    temperature: Option<f32>,
    /// Disable thinking mode for simple tasks (commit messages).
    /// DeepSeek V4 defaults to thinking mode which wastes tokens on reasoning.
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking: Option<ThinkingConfig>,
}

/// API 响应中的 message 对象。
/// - `content` uses `#[serde(default)]` for `null` compatibility.
/// - `reasoning_content` is populated by DeepSeek V4 thinking mode (separate from content).
#[derive(Deserialize, Debug)]
struct ChatMessage {
    #[serde(default)]
    content: String,
    #[serde(default)]
    reasoning_content: Option<String>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

/// 从推理模型响应中剥离 `<think` 思考内容。
///
/// DeepSeek R1 等旧模型会在 `content` 字段中嵌入 `<think` 思考过程，
/// 实际回答位于闭合标签 `</think` 之后。
///
/// DeepSeek V4+ 将推理内容放在单独的 `reasoning_content` 字段，
/// `content` 直接就是最终回答，无需剥离。
fn strip_thinking_content(content: &str) -> String {
    let trimmed = content.trim();
    let lower = trimmed.to_lowercase();

    // 查找最后一个 </think 闭合标签，取其后的内容
    if let Some(pos) = lower.rfind("</think")
        && let Some(gt_pos) = trimmed[pos..].find('>')
    {
        let after_tag = trimmed[pos + gt_pos + 1..].trim();
        if !after_tag.is_empty() {
            return after_tag.to_string();
        }
    }

    // 以 <think 开头但没有闭合标签 → 全部是思考过程
    if lower.starts_with("<think") {
        return String::new();
    }

    trimmed.to_string()
}

pub struct AiClient {
    client: Client,
    config: ApiConfig,
    system_prompt: String,
    user_prompt_template: String,
}

impl AiClient {
    pub fn new(config: ApiConfig, system_prompt: String, user_prompt_template: String) -> anyhow::Result<Self> {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(120))
            .build()
            .context("Failed to build HTTP client — TLS backend initialization error")?;
        Ok(AiClient { client, config, system_prompt, user_prompt_template })
    }

    pub async fn send_chat_request(&self, messages: Vec<Message>) -> anyhow::Result<String> {
        // Disable thinking mode — commit message generation doesn't need reasoning.
        // DeepSeek V4 defaults to thinking mode which can consume all max_tokens on
        // reasoning alone (finish_reason=length, 0 content tokens).
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            thinking: Some(ThinkingConfig { r#type: "disabled".to_string() }),
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
                anyhow::bail!("Failed to parse API response: {e}. Run with RUST_LOG=debug to see the raw response.");
            }
        };

        let choice = chat_response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("API returned empty choices — the request may have been filtered"))?;

        if choice.finish_reason.as_deref() == Some("length") {
            anyhow::bail!(
                "AI response was truncated (finish_reason=length). \
                 Consider increasing max_tokens (currently {:?}). \
                 Set env var AI_COMMIT_MAX_TOKENS to a higher value.",
                self.config.max_tokens
            );
        }

        // DeepSeek V4 thinking mode: reasoning is in reasoning_content, content is the answer.
        // Legacy models: reasoning is inline in content via <think/> tags.
        let content = choice.message.content;
        let message = strip_thinking_content(&content);

        if message.trim().is_empty() {
            if choice.message.reasoning_content.is_some() {
                anyhow::bail!(
                    "AI produced only reasoning (reasoning_content present, content empty). \
                     Thinking mode may not be fully disabled. \
                     Try setting AI_COMMIT_MAX_TOKENS to a higher value."
                );
            }
            anyhow::bail!(
                "AI returned an empty response after stripping thinking content. \
                 The model may have produced only reasoning without a final answer."
            );
        }

        Ok(message)
    }

    pub async fn generate_commit_message(&self, diff: &str) -> anyhow::Result<String> {
        let system_message = Message { role: "system".to_string(), content: self.system_prompt.clone() };
        let user_content = self.user_prompt_template.replace("{diff}", diff);
        let user_message = Message { role: "user".to_string(), content: user_content };
        let messages = vec![system_message, user_message];
        debug!("Sending messages: {messages:?}");
        self.send_chat_request(messages).await
    }

    pub async fn generate_commit_message_with_keywords(&self, diff: &str, keywords: &str) -> anyhow::Result<String> {
        let system_message = Message { role: "system".to_string(), content: self.system_prompt.clone() };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_thinking_with_tag() {
        let content = "<think >Some thinking\nMultiple lines\n</think >feat: add new feature";
        assert_eq!(strip_thinking_content(content), "feat: add new feature");
    }

    #[test]
    fn test_strip_thinking_unclosed_tag() {
        let content = "<think >Some thinking without closing tag";
        assert_eq!(strip_thinking_content(content), "");
    }

    #[test]
    fn test_strip_thinking_no_tag() {
        let content = "feat: add new feature";
        assert_eq!(strip_thinking_content(content), "feat: add new feature");
    }

    #[test]
    fn test_strip_thinking_empty_after_tag() {
        let content = "<think >thinking</think >";
        assert_eq!(strip_thinking_content(content), "");
    }

    #[test]
    fn test_strip_thinking_whitespace_after_tag() {
        let content = "<think >thinking</think >   \n  feat: fix bug  ";
        assert_eq!(strip_thinking_content(content), "feat: fix bug");
    }

    #[test]
    fn test_strip_thinking_case_insensitive() {
        let content = "<THINK >thinking</THINK >feat: add feature";
        assert_eq!(strip_thinking_content(content), "feat: add feature");
    }

    #[test]
    fn test_strip_thinking_multiline() {
        let content = "<think >\nLine 1\nLine 2\n</think >\n\nfeat: add feature\n";
        assert_eq!(strip_thinking_content(content), "feat: add feature");
    }
}
