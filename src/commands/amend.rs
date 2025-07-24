use anyhow::Result;
use colored::*;
use std::io::{self, Write};

use crate::ai::AiClient;
use crate::git::{execute_amend_with_cli, get_amend_diff, get_last_commit_message, get_staged_diff};

pub async fn handle_amend() -> Result<()> {
    let ai_client = AiClient::new();

    // 检查是否有staged changes或者需要amend
    let staged_diff = get_staged_diff()?;
    let amend_diff = get_amend_diff()?;
    let last_commit_msg = get_last_commit_message()?;

    let diff_content = if !staged_diff.is_empty() {
        println!("{}", "✅ Found staged changes to amend.".green());
        amend_diff
    } else {
        println!("{}", "⚠️  No staged changes found.".yellow());
        println!("{}", "Will generate new message for existing commit content.".yellow());
        amend_diff
    };

    if diff_content.is_empty() {
        println!("{}", "❌ No changes found to amend.".red());
        return Ok(());
    }

    println!("{}", "📝 Current commit message:".bright_blue().bold());
    println!("{}", "─────────────────────".bright_blue());
    println!("{}", last_commit_msg.trim().bright_yellow());
    println!("{}", "─────────────────────".bright_blue());

    println!("{}", "🤖 Generating new commit message using AI service...".cyan());

    match ai_client.generate_commit_message(&diff_content).await {
        Ok(message) => {
            if message.is_empty() {
                println!("{}", "❌ AI did not generate a commit message.".red());
                return Ok(());
            }

            println!("{}", "✨ Generated new commit message:".bright_cyan().bold());
            println!("{}", "─────────────────────".bright_blue());
            println!("{}", message.bright_green().bold());
            println!("{}", "─────────────────────".bright_blue());

            if !confirm_amend()? {
                println!("{}", "❌ Amend cancelled.".red());
                return Ok(());
            }

            execute_amend_with_cli(&message)?;
        }
        Err(e) => {
            println!("{}", format!("❌ Failed to generate commit message 1: {e}").red());
        }
    };

    Ok(())
}

pub async fn handle_amend_with_options(dry_run: bool) -> Result<()> {
    let ai_client = AiClient::new();

    let staged_diff = get_staged_diff()?;
    let amend_diff = get_amend_diff()?;
    let last_commit_msg = get_last_commit_message()?;

    let diff_content = if !staged_diff.is_empty() {
        println!("{}", "✅ Found staged changes to amend.".green());
        amend_diff
    } else {
        println!("{}", "⚠️  No staged changes found.".yellow());
        println!("{}", "Will generate new message for existing commit content.".yellow());
        amend_diff
    };

    if diff_content.is_empty() {
        println!("{}", "❌ No changes found to amend.".red());
        return Ok(());
    }

    println!("{}", "📝 Current commit message:".bright_blue().bold());
    println!("{}", "─────────────────────".bright_blue());
    println!("{}", last_commit_msg.trim().bright_yellow());
    println!("{}", "─────────────────────".bright_blue());

    println!("{}", "🤖 Generating new commit message using AI service...".cyan());

    match ai_client.generate_commit_message(&diff_content).await {
        Ok(message) => {
            if message.is_empty() {
                println!("{}", "❌ AI did not generate a commit message.".red());
                return Ok(());
            }

            println!("{}", "✨ Generated new commit message:".bright_cyan().bold());
            println!("{}", "─────────────────────".bright_blue());
            println!("{}", message.bright_green().bold());
            println!("{}", "─────────────────────".bright_blue());

            if dry_run {
                println!("{}", "(Dry run mode - no actual amend made)".yellow());
                println!("{}", "To amend: git commit --amend -m \"<message>\"".yellow());
            } else {
                if !confirm_amend()? {
                    println!("{}", "❌ Amend cancelled.".red());
                    return Ok(());
                }
                execute_amend_with_cli(&message)?;
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Failed to generate commit message 2: {e}").red());
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
