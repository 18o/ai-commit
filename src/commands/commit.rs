use anyhow::Result;
use colored::*;
use std::io::{self, Write};

use crate::ai::AiClient;
use crate::git::{execute_commit_with_cli, get_staged_diff, get_unstaged_diff};

pub async fn handle_commit() -> Result<()> {
    let ai_client = AiClient::new();

    let staged_diff = get_staged_diff()?;
    let unstaged_diff = get_unstaged_diff()?;

    let (diff_content, is_dry_run) = if !staged_diff.is_empty() {
        println!("{}", "✅ Staged changes found. Generating commit message...".green());
        (staged_diff, false)
    } else if !unstaged_diff.is_empty() {
        println!("{}", "⚠️ No staged changes found, but found unstaged changes.".yellow());
        println!("{}", "Running in dry-run mode (no actual commit will be made).".yellow());
        println!("{}", "To commit these changes, please stage them first with 'git add'.".yellow());
        (unstaged_diff, true)
    } else {
        println!("{}", "❌ No changes found to commit.".red());
        return Ok(());
    };

    println!("{}", "🤖 Generating commit message using AI service...".cyan());

    match ai_client.generate_commit_message(&diff_content).await {
        Ok(message) => {
            if message.is_empty() {
                println!("{}", "❌ AI did not generate a commit message.".red());
                return Ok(());
            }

            println!("{}", "✨ Generated commit message:".bright_cyan().bold());
            println!("{}", "─────────────────────".bright_blue());
            println!("{}", message.bright_green().bold());
            println!("{}", "─────────────────────".bright_blue());

            if is_dry_run {
                println!("{}", "(Dry run mode - no actual commit made)".yellow());
                println!("{}", "To commit these changes:".yellow());
                println!("{}", "1. Stage your changes: git add <files>".yellow());
                println!("{}", "2. Run ai-commit again".yellow());
            } else {
                if !confirm_commit()? {
                    println!("{}", "❌ Commit cancelled.".red());
                    return Ok(());
                }
                execute_commit_with_cli(&message)?;
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Failed to generate commit message 3: {e}").red());
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
