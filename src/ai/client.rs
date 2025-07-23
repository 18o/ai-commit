use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    ai::{format_commit_prompt, SYSTEM_PROMPT},
    config::AppConfig,
};

#[derive(Serialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    model: String,
    thinking: Thinking,
    messages: Vec<Message>,
    max_tokens: Option<usize>,
    temperature: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Thinking {
    r#type: String,
}

impl Default for Thinking {
    fn default() -> Self {
        Thinking { r#type: "disabled".to_string() }
    }
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

#[derive(Default)]
pub struct AiClient {
    client: Client,
    api_key: String,
    endpoint: String,
    model: String,
}

impl AiClient {
    pub fn new() -> Self {
        let config = AppConfig::load().unwrap_or_default();
        let client = Client::new();

        let api_key = AppConfig::get_api_key().unwrap_or_else(|e| {
            eprintln!("Warning: {e}");
            String::new()
        });

        AiClient { client, api_key, endpoint: config.api.endpoint, model: config.api.model }
    }

    pub async fn send_chat_request(&self, messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error>> {
        let config = AppConfig::load().unwrap_or_default();

        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            max_tokens: config.api.max_tokens,
            temperature: config.api.temperature,
            thinking: Thinking::default(),
        };
        let response = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {error_text}").into());
        }

        let chat_response: ChatResponse = response.json().await?;

        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No response from AI".into())
        }
    }

    pub async fn generate_commit_message(&self, diff: &str) -> Result<String, Box<dyn std::error::Error>> {
        let system_message = Message { role: "system".to_string(), content: SYSTEM_PROMPT.to_string() };

        let user_message = Message { role: "user".to_string(), content: format_commit_prompt(diff) };

        let messages = vec![system_message, user_message];
        self.send_chat_request(messages).await
    }
}
