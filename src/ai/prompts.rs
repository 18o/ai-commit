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

SMART FORMAT SELECTION:
- Single feature/fix/refactor (even across multiple files): Use only title line under 72 characters
- Multiple unrelated features or different functional changes: Add blank line + bullet points

Use bullet points ONLY when the commit contains multiple DIFFERENT functional changes, not when:
- Refactoring one feature across multiple files
- Adding one feature with supporting files
- Fixing one issue that affects multiple files
- Making related changes for a single purpose

BULLET POINT GUIDELINES:
- Focus on WHAT functionality was added/changed/fixed, not which files
- Describe user-facing features, API changes, or system improvements
- Avoid mentioning specific file names or paths
- Keep each point concise and focused on the functional impact

Examples:

Single purpose (no bullet points needed):
```
refactor: improve error handling in authentication module
```

Multiple purposes (needs bullet points):
```
feat: add user management and notification system

- Implement user CRUD operations with validation
- Add email notification service for user events
- Create admin dashboard for user management
```

WRONG format (don't do this):
```
feat: add user system

- Update UserController.js to add CRUD methods
- Modify database schema in migration files
- Add new routes in routes/users.js
```

Focus on functional purpose and impact, not file-level changes."#;

pub const USER_PROMPT_TEMPLATE: &str = r#"Analyze the following git diff and determine if it represents:
1. A SINGLE functional purpose (one feature, fix, or refactor) - use title only
2. MULTIPLE different functional purposes - use title + bullet points

When using bullet points:
- Describe WHAT functionality was implemented/changed/fixed
- Focus on features, capabilities, or system improvements
- Do NOT mention specific file names or paths
- Keep each point focused on user or system impact

Git diff:
```diff
{diff}
```

Provide only the commit message in the appropriate format."#;

pub fn estimate_diff_complexity(diff: &str) -> DiffComplexity {
    // let lines = diff.lines().count();
    let files_changed = diff.matches("diff --git").count();
    // let additions = diff.matches("+").count();
    // let deletions = diff.matches("-").count();

    // Count different types of changes to detect multiple purposes
    let has_new_features = diff.contains("class ") || diff.contains("function ") || diff.contains("def ");
    let has_test_changes = diff.contains("test") || diff.contains("spec");
    let has_config_changes = diff.contains("config") || diff.contains(".toml") || diff.contains(".json");
    let has_documentation = diff.contains("README") || diff.contains(".md");

    let change_types =
        [has_new_features, has_test_changes, has_config_changes, has_documentation].iter().filter(|&&x| x).count();

    // Focus on logical complexity, not just size
    if change_types <= 1 && files_changed <= 10 {
        // Single purpose, even if many files
        DiffComplexity::Simple
    } else if change_types == 2 && files_changed <= 15 {
        // Two related purposes
        DiffComplexity::Medium
    } else if change_types > 2 || files_changed > 15 {
        // Multiple different purposes
        DiffComplexity::Complex
    } else {
        // Default to simple for focused changes
        DiffComplexity::Simple
    }
}

pub fn format_commit_prompt(diff: &str) -> String {
    USER_PROMPT_TEMPLATE.replace("{diff}", diff)
}

#[derive(Debug, PartialEq)]
pub enum DiffComplexity {
    Simple,
    Medium,
    Complex,
}
