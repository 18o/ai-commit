use crate::config::AppConfig;

pub fn get_system_prompt() -> String {
    AppConfig::load().map(|config| config.prompts.system_prompt).unwrap_or_else(|_| get_default_system_prompt())
}

pub fn get_user_prompt_template() -> String {
    AppConfig::load()
        .map(|config| config.prompts.user_prompt_template)
        .unwrap_or_else(|_| get_default_user_prompt_template())
}

pub fn get_simple_prompt_template() -> String {
    AppConfig::load()
        .map(|config| config.prompts.simple_prompt_template)
        .unwrap_or_else(|_| get_default_simple_prompt_template())
}

pub fn format_commit_prompt(diff: &str) -> String {
    get_user_prompt_template().replace("{diff}", diff)
}

pub fn format_simple_commit_prompt(diff: &str) -> String {
    get_simple_prompt_template().replace("{diff}", diff)
}

pub fn get_default_system_prompt() -> String {
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

WHEN TO USE SINGLE LINE (preferred):
- Adding one feature (even across multiple files)
- Fixing one bug (even if it affects multiple files)  
- Refactoring one area/component (even with many files)
- Making changes for one single purpose
- Updating configuration for one goal
- Adding tests for one feature

WHEN TO USE BULLET POINTS (rare):
- Adding authentication system AND payment processing (2 unrelated features)
- Fixing database bug AND adding new API endpoints (unrelated changes)
- Multiple completely different functional areas modified

Default to single line. Only use bullets for truly unrelated changes."#
        .to_string()
}

pub fn get_default_user_prompt_template() -> String {
    r#"Analyze the following git diff and generate a commit message.

IMPORTANT: Default to a single descriptive line under 72 characters.
Only use bullet points if there are multiple COMPLETELY UNRELATED functional changes.

Most commits should be single-line format like:
- feat: add user profile management
- fix: resolve memory leak in image processing
- refactor: simplify authentication middleware

Git diff:
```diff
{diff}
```

Provide only the commit message."#
        .to_string()
}

pub fn get_default_simple_prompt_template() -> String {
    r#"Generate a concise single-line commit message for the following changes.

Focus on the main purpose/goal of these changes in under 72 characters.

Git diff:
```diff
{diff}
```

Provide only the commit message (single line, no bullet points)."#
        .to_string()
}
