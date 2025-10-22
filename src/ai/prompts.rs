pub const SYSTEM_PROMPT: &str = r#"You are an expert software developer and git commit message writer. 

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

Keep the description under 72 characters for the title.
If needed, add a blank line and more detailed explanation.

Focus on WHAT changed and WHY, not HOW."#;

pub const USER_PROMPT_TEMPLATE: &str = r#"Based on the following git diff, generate a commit message following conventional commit format.

Git diff:
```diff
{diff}
```

Please provide only the commit message, no explanations or additional text."#;

pub const USER_PROMPT_WITH_KEYWORDS_TEMPLATE: &str = r#"Based on the following git diff, generate a commit message following conventional commit format.

User provided keywords/context: {keywords}

Git diff:
```diff
{diff}
```

Please focus on the user's keywords/context when generating the commit message. The keywords indicate the key changes or focus areas. Please provide only the commit message, no explanations or additional text."#;

pub fn format_commit_prompt(diff: &str) -> String {
    USER_PROMPT_TEMPLATE.replace("{diff}", diff)
}

pub fn format_commit_prompt_with_keywords(diff: &str, keywords: &str) -> String {
    USER_PROMPT_WITH_KEYWORDS_TEMPLATE
        .replace("{diff}", diff)
        .replace("{keywords}", keywords)
}
