use anyhow::Result;
use std::process::Command;

pub fn execute_commit_with_cli(message: &str) -> Result<()> {
    println!("🚀 Committing changes...");

    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", message]);

    if is_gpg_signing_enabled()? {
        println!("🔐 GPG signing is enabled, using git command for proper signing...");
    }

    let status = cmd.status()?;

    if status.success() {
        println!("✅ Commit successful!");
        show_commit_info()?;
    } else {
        return Err(anyhow::anyhow!("Commit failed"));
    }

    Ok(())
}

pub fn is_gpg_signing_enabled() -> Result<bool> {
    let output = Command::new("git").args(["config", "--get", "commit.gpgsign"]).output()?;

    if output.status.success() {
        let value = String::from_utf8_lossy(&output.stdout);
        Ok(value.trim() == "true")
    } else {
        let output = Command::new("git").args(["config", "--global", "--get", "commit.gpgsign"]).output()?;

        if output.status.success() {
            let value = String::from_utf8_lossy(&output.stdout);
            Ok(value.trim() == "true")
        } else {
            Ok(false)
        }
    }
}

pub fn show_commit_info() -> Result<()> {
    let output = Command::new("git").args(["log", "-1", "--oneline"]).output()?;

    if output.status.success() {
        let commit_info = String::from_utf8_lossy(&output.stdout);
        println!("Latest commit: {}", commit_info.trim());
    }

    Ok(())
}

pub fn execute_amend_with_cli(message: &str) -> Result<()> {
    println!("🔄 Amending last commit...");

    let mut cmd = Command::new("git");
    cmd.args(["commit", "--amend", "-m", message]);

    if is_gpg_signing_enabled()? {
        println!("🔐 GPG signing is enabled, using git command for proper signing...");
    }

    let status = cmd.status()?;

    if status.success() {
        println!("✅ Commit amended successfully!");
        show_commit_info()?;
    } else {
        return Err(anyhow::anyhow!("Amend failed"));
    }

    Ok(())
}
