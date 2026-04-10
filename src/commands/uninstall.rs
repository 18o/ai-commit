use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn uninstall_hook() -> Result<()> {
    println!("Uninstalling AI-assisted Git hook...");

    let hooks_dir = Path::new(".git/hooks");

    let prepare_hook = hooks_dir.join("prepare-commit-msg");
    if prepare_hook.exists() {
        fs::remove_file(&prepare_hook).with_context(|| format!("Failed to remove {}", prepare_hook.display()))?;
        println!("Removed prepare-commit-msg hook.");
    } else {
        println!("No prepare-commit-msg hook found.");
    }

    let post_hook = hooks_dir.join("post-commit");
    if post_hook.exists() {
        fs::remove_file(&post_hook).with_context(|| format!("Failed to remove {}", post_hook.display()))?;
        println!("Removed post-commit hook.");
    } else {
        println!("No post-commit hook found.");
    }

    println!("Uninstallation complete.");
    Ok(())
}
