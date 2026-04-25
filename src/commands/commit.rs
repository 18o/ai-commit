use anyhow::Result;
use colored::*;
use std::io::{self, Write};

use crate::ai::AiClient;
use crate::config::{ApiConfig, AppConfig};
use crate::git::{execute_commit_with_cli, get_staged_diff, get_truncated_diff, get_unstaged_diff};

pub async fn handle_commit(
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
    let unstaged_diff = get_unstaged_diff(Some(&app_config.commit))?;

    let limit = context_limit.unwrap_or(app_config.commit.context_limit);

    let (diff_content, auto_dry_run) = if !staged_diff.is_empty() {
        println!("{}", "Staged changes found. Generating commit message...".green());
        (staged_diff, false)
    } else if !unstaged_diff.is_empty() {
        println!("{}", "No staged changes found, but found unstaged changes.".yellow());
        println!("{}", "Running in dry-run mode (no actual commit will be made).".yellow());
        println!("{}", "To commit these changes, please stage them first with 'git add'.".yellow());
        (unstaged_diff, true)
    } else {
        println!("{}", "No changes found to commit.".red());
        return Ok(());
    };

    let is_dry_run = dry_run || auto_dry_run;
    let diff_content = get_truncated_diff(&diff_content, limit);

    if let Some(kw) = keywords {
        println!("{}", format!("Using keywords: {kw}").cyan());
    }
    println!("{}", "Generating commit message using AI service...".cyan());

    let result = if let Some(kw) = keywords {
        ai_client.generate_commit_message_with_keywords(&diff_content, kw).await
    } else {
        ai_client.generate_commit_message(&diff_content).await
    };

    match result {
        Ok(message) => {
            println!("{}", "Generated commit message:".bright_cyan().bold());
            println!("{}", "─────────────────────".bright_blue());
            println!("{}", message.bright_green().bold());
            println!("{}", "─────────────────────".bright_blue());

            if is_dry_run {
                println!("{}", "(Dry run mode - no actual commit made)".yellow());
            } else if app_config.commit.auto_confirm || confirm_commit()? {
                execute_commit_with_cli(&message)?;
            } else {
                println!("{}", "Commit cancelled.".red());
            }
        }
        Err(e) => {
            eprintln!("{} {e}", "Failed to generate commit message:".red());
            return Err(e);
        }
    };

    Ok(())
}

fn confirm_commit() -> Result<bool> {
    print!("Do you want to commit with this message? (y/N): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}
