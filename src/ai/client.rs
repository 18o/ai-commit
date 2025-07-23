use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::ai::{format_commit_prompt, SYSTEM_PROMPT};

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
    role: String,
}

#[derive(Deserialize)]
struct Choice {
    finish_reason: String,
    index: u32,
    message: ChatMessage,
}

#[derive(Deserialize, Debug)]
struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    created: u64,
    id: String,
    model: String,
    object: String,
    usage: Usage,
}

#[derive(Default)]
pub struct AiClient {
    client: Client,
    api_key: String,
    endpoint: String,
    model: String,
}

const API_KEY: &str = "";
const MODEL: &str = "doubao-seed-1.6-250615";
const DEFAULT_API_ENDPOINT: &str = "https://ark.cn-beijing.volces.com/api/v3/chat/completions";

impl AiClient {
    pub fn new() -> Self {
        let client = Client::new();
        let api_key = std::env::var("AI_COMMIT_ARK_API_KEY").unwrap_or_else(|_| API_KEY.to_string());
        let endpoint = std::env::var("AI_COMMIT_ARK_ENDPOINT").unwrap_or_else(|_| DEFAULT_API_ENDPOINT.to_string());
        let model = std::env::var("AI_COMMIT_ARK_MODEL").unwrap_or_else(|_| MODEL.to_string());

        AiClient { client, api_key, endpoint, model }
    }

    pub async fn send_chat_request(&self, messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error>> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            max_tokens: Some(1000),
            temperature: Some(0.7),
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
