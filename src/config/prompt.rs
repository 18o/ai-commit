use crate::config::AppConfig;

pub fn get_system_prompt() -> String {
    AppConfig::load().map(|config| config.prompts.system_prompt).unwrap_or_else(|_| default_system_prompt())
}

pub fn get_user_prompt_template() -> String {
    AppConfig::load()
        .map(|config| config.prompts.user_prompt_template)
        .unwrap_or_else(|_| default_user_prompt_template())
}

pub fn format_commit_prompt(diff: &str) -> String {
    get_user_prompt_template().replace("{diff}", diff)
}

pub fn format_commit_prompt_with_keywords(diff: &str, keywords: &str) -> String {
    let template = r#"Based on the following git diff, generate a commit message.

User provided keywords/context: {keywords}

Git diff:
```diff
{diff}
```

Please focus on the user's keywords/context when generating the commit message. The keywords indicate the key changes or focus areas. Please provide only the commit message, no explanations or additional text."#;
    template.replace("{diff}", diff).replace("{keywords}", keywords)
}

fn default_system_prompt() -> String {
    r#"You are an expert software developer and git commit message writer.

Generate concise, clear commit messages following the Conventional Commits specification:
- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Changes that do not affect the meaning of the code
- refactor: A code change that neither fixes a bug nor adds a feature
- perf: A code change that improves performance
- test: Adding missing tests or correcting existing tests
- chore: Changes to the build process or auxiliary tools

Format: type(scope): description

PREFERRED FORMAT: Single line under 72 characters
Use bullet points ONLY when there are truly MULTIPLE UNRELATED functional changes.

Default to single line. Only use bullets for truly unrelated changes."#
        .to_string()
}

fn default_user_prompt_template() -> String {
    r#"Analyze the following git diff and generate a commit message.

IMPORTANT: Default to a single descriptive line under 72 characters.
Only use bullet points if there are multiple COMPLETELY UNRELATED functional changes.

Git diff:
```diff
{diff}
```

Provide only the commit message."#
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_commit_prompt_replaces_diff() {
        let result = format_commit_prompt("some diff content");
        assert!(result.contains("some diff content"));
        assert!(!result.contains("{diff}"));
    }

    #[test]
    fn test_format_commit_prompt_with_keywords_replaces_both() {
        let result = format_commit_prompt_with_keywords("my diff", "fix auth");
        assert!(result.contains("my diff"));
        assert!(result.contains("fix auth"));
        assert!(!result.contains("{diff}"));
        assert!(!result.contains("{keywords}"));
    }

    #[test]
    fn test_default_system_prompt_not_empty() {
        let prompt = default_system_prompt();
        assert!(!prompt.is_empty());
        assert!(prompt.contains("Conventional Commits"));
    }

    #[test]
    fn test_default_user_prompt_template_has_placeholder() {
        let template = default_user_prompt_template();
        assert!(template.contains("{diff}"));
    }
}
