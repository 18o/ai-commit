// src/ai/prompts.rs
pub const COMMIT_MESSAGE_PROMPT: &str = "Please provide a brief description of the changes made:";

pub const COMMIT_TEMPLATE: &str = r#"### Commit Message
- **Type:** [feat|fix|docs|style|refactor|perf|test|chore]
- **Scope:** (optional)
- **Description:** 
- **Related Issues:** (optional)
"#;

pub fn generate_commit_prompt() -> String {
    format!("{}\n{}", COMMIT_MESSAGE_PROMPT, COMMIT_TEMPLATE)
}