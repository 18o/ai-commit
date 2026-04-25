use anyhow::Result;
use colored::*;
use std::io::{self, Write};

use crate::ai::AiClient;
use crate::config::{ApiConfig, AppConfig};
use crate::git::{
    execute_amend_with_cli, get_amend_diff, get_last_commit_message, get_staged_diff, get_truncated_diff,
};

pub async fn handle_amend(
    keywords: Option<&str>,
    dry_run: bool,
    context_limit: Option<usize>,
) -> Result<()> {
    let app_config = AppConfig::load_or_create()?;
    let api_config = ApiConfig::from_env(&app_config.env)?;
    let ai_client = AiClient::new(
        api_config,
        app_config.prompts.system_prompt.clone(),
        app_config.prompts.user_prompt_template.clone(),
    )?;

    let staged_diff = get_staged_diff(Some(&app_config.commit))?;
    let amend_diff = get_amend_diff(Some(&app_config.commit))?;
    let last_commit_msg = get_last_commit_message()?;

    let diff_content = if !staged_diff.is_empty() {
        println!("{}", "Found staged changes to amend.".green());
        amend_diff
    } else {
        println!("{}", "No staged changes found.".yellow());
        println!("{}", "Will generate new message for existing commit content.".yellow());
        amend_diff
    };

    if diff_content.is_empty() {
        println!("{}", "No changes found to amend.".red());
        return Ok(());
    }

    let limit = context_limit.unwrap_or(app_config.commit.context_limit);
    let diff_content = get_truncated_diff(&diff_content, limit);

    println!("{}", "Current commit message:".bright_blue().bold());
    println!("{}", "─────────────────────".bright_blue());
    println!("{}", last_commit_msg.trim().bright_yellow());
    println!("{}", "─────────────────────".bright_blue());

    if let Some(kw) = keywords {
        println!("{}", format!("Using keywords: {kw}").cyan());
    }
    println!("{}", "Generating new commit message using AI service...".cyan());

    let result = if let Some(kw) = keywords {
        ai_client.generate_commit_message_with_keywords(&diff_content, kw).await
    } else {
        ai_client.generate_commit_message(&diff_content).await
    };

    match result {
        Ok(message) => {
            println!("{}", "Generated new commit message:".bright_cyan().bold());
            println!("{}", "─────────────────────".bright_blue());
            println!("{}", message.bright_green().bold());
            println!("{}", "─────────────────────".bright_blue());

            if dry_run {
                println!("{}", "(Dry run mode - no actual amend made)".yellow());
            } else if app_config.commit.auto_confirm || confirm_amend()? {
                execute_amend_with_cli(&message)?;
            } else {
                println!("{}", "Amend cancelled.".red());
            }
        }
        Err(e) => {
            eprintln!("{} {e}", "Failed to generate commit message:".red());
            return Err(e);
        }
    };

    Ok(())
}

fn confirm_amend() -> Result<bool> {
    print!("Do you want to amend the commit with this message? (y/N): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}
