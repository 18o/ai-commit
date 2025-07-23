use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn install_hook() -> Result<()> {
    let hook_path = Path::new(".git/hooks/prepare-commit-msg");
    let template_path = Path::new("templates/prepare-commit-msg");

    fs::copy(template_path, hook_path)?;

    println!("Git hook installed successfully.");
    Ok(())
}
