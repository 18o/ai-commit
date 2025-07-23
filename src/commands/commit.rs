use crate::ai::AiClient;
use anyhow::{Ok, Result};
// use git2::Repository;

pub async fn handle_commit() -> Result<()> {
    let ai_client = AiClient::new();
    let message = std::env::var("COMMIT_MESSAGE").unwrap_or_else(|_| "No commit message provided".to_string());
    let enhanced_message = ai_client.generate_commit_message(&message);

    // Here you can add logic to integrate the enhanced message with the commit process
    enhanced_message.await.unwrap_or_else(|_| {
        eprintln!("Failed to generate commit message using AI service.");
        message.to_string()
    });

    Ok(())
}
