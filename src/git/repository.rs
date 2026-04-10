use anyhow::Result;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn execute_commit_with_cli(message: &str) -> Result<()> {
    println!("Committing changes...");

    let mut cmd = Command::new("git");
    cmd.args(["commit", "--file", "-"]).stdin(Stdio::piped());

    setup_gpg_signing(&mut cmd)?;

    let mut child = cmd.spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(message.as_bytes())?;
    }
    let status = child.wait()?;

    if status.success() {
        println!("Commit successful!");
        show_commit_info()?;
    } else {
        return Err(anyhow::anyhow!("Commit failed"));
    }

    Ok(())
}

pub fn execute_amend_with_cli(message: &str) -> Result<()> {
    println!("Amending last commit...");

    let mut cmd = Command::new("git");
    cmd.args(["commit", "--amend", "--file", "-"]).stdin(Stdio::piped());

    setup_gpg_signing(&mut cmd)?;

    let mut child = cmd.spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(message.as_bytes())?;
    }
    let status = child.wait()?;

    if status.success() {
        println!("Commit amended successfully!");
        show_commit_info()?;
    } else {
        return Err(anyhow::anyhow!("Amend failed"));
    }

    Ok(())
}

fn setup_gpg_signing(cmd: &mut Command) -> Result<()> {
    if is_gpg_signing_enabled()? {
        println!("GPG signing is enabled...");
        if let Ok(tty) = Command::new("tty").output()
            && tty.status.success()
        {
            let tty_path = String::from_utf8_lossy(&tty.stdout).trim().to_string();
            cmd.env("GPG_TTY", tty_path);
        }
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
