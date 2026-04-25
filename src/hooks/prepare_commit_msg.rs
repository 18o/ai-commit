use anyhow::{Context, Result};
use std::env;
use std::fs;

pub fn prepare_commit_msg() -> Result<()> {
    let commit_msg_file = env::args().nth(1).context("No commit message file provided")?;

    let mut commit_msg = fs::read_to_string(&commit_msg_file)
        .with_context(|| format!("Failed to read commit message file: {commit_msg_file}"))?;

    // Here you can add logic to modify the commit message, e.g., integrating AI suggestions
    // For now, we will just append a note to the commit message
    commit_msg.push_str("\n\n[AI-assisted commit message]");

    fs::write(&commit_msg_file, commit_msg)
        .with_context(|| format!("Failed to write commit message file: {commit_msg_file}"))?;

    Ok(())
}
