// filepath: /ai-commit/ai-commit/src/git/diff.rs
use std::process::Command;

pub struct Diff {
    pub changes: String,
}

impl Diff {
    pub fn new() -> Self {
        Diff {
            changes: String::new(),
        }
    }

    pub fn get_diff(&mut self) -> Result<&str, String> {
        let output = Command::new("git")
            .arg("diff")
            .output()
            .map_err(|e| format!("Failed to execute git diff: {}", e))?;

        if output.status.success() {
            self.changes = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(&self.changes)
        } else {
            Err(format!(
                "Git diff command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}