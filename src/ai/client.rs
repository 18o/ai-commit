use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::config::ApiConfig;
use crate::config::prompt::{format_commit_prompt, format_commit_prompt_with_keywords, get_system_prompt};

#[derive(Serialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<usize>,
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
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
}

impl AiClient {
    pub fn new(config: ApiConfig) -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();
        AiClient { client, config }
    }

    pub async fn send_chat_request(&self, messages: Vec<Message>) -> anyhow::Result<String> {
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        let response = self
            .client
            .post(&self.config.endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            anyhow::bail!("API request failed ({status}): {error_text}");
        }

        let chat_response: ChatResponse = response.json().await?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from AI"))
    }

    pub async fn generate_commit_message(&self, diff: &str) -> anyhow::Result<String> {
        let system_message = Message { role: "system".to_string(), content: get_system_prompt() };
        let user_message = Message { role: "user".to_string(), content: format_commit_prompt(diff) };
        let messages = vec![system_message, user_message];
        debug!("Sending messages: {messages:?}");
        self.send_chat_request(messages).await
    }

    pub async fn generate_commit_message_with_keywords(&self, diff: &str, keywords: &str) -> anyhow::Result<String> {
        let system_message = Message { role: "system".to_string(), content: get_system_prompt() };
        let user_message =
            Message { role: "user".to_string(), content: format_commit_prompt_with_keywords(diff, keywords) };
        let messages = vec![system_message, user_message];
        debug!("Sending messages: {messages:?}");
        self.send_chat_request(messages).await
    }
}
