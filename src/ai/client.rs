// This file contains the logic for communicating with the AI service, handling requests and responses.

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AiRequest {
    prompt: String,
    max_tokens: usize,
}

#[derive(Serialize, Deserialize)]
struct AiResponse {
    text: String,
}

pub struct AiClient {
    client: Client,
    api_key: String,
    endpoint: String,
}

impl AiClient {
    pub fn new() -> Self {
        let client = Client::new();
        let api_key = std::env::var("AI_API_KEY").unwrap_or_default();
        let endpoint = std::env::var("AI_API_ENDPOINT").unwrap_or_default();
        AiClient { client, api_key, endpoint }
    }

    pub async fn send_request(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = AiRequest { prompt: prompt.to_string(), max_tokens: 100 };

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let ai_response: AiResponse = response.json().await?;
        Ok(ai_response.text)
    }

    pub async fn generate_commit_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!("Generate a commit message based on the following content:\n{}", message);
        self.send_request(&prompt).await
    }
}
