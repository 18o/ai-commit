# AI Commit Tool

An intelligent Git commit message generator that uses AI to analyze your code changes and create meaningful, conventional commit messages automatically.

## Overview

AI Commit Tool integrates with your Git workflow to automatically generate high-quality commit messages following the Conventional Commits specification. It analyzes your staged or unstaged changes and uses AI to craft appropriate commit messages, saving time and ensuring consistency across your project.

## Features

- **AI-Generated Commit Messages**: Automatically analyzes git diffs and generates contextual commit messages following conventional commit format
- **Keyword-Guided Generation**: Provide keywords or context to guide AI focus on specific aspects of your changes
- **Smart Format Selection**: Automatically chooses between concise single-line messages or detailed bullet-point format based on change complexity
- **Dry Run Mode**: Preview generated messages without committing via `--dry-run`
- **Context Limit**: Control how much diff content is sent to the AI via `--context-limit`
- **Amend Support**: Generate new messages for amending previous commits with additional changes
- **Lock File Filtering**: Automatically ignores common lock files (Cargo.lock, package-lock.json, yarn.lock, etc.) from analysis
- **Custom Ignore Patterns**: Glob-based patterns to filter out files from analysis (e.g. `**/generated/**`)
- **GPG Signing Support**: Works seamlessly with GPG-signed commits
- **Environment Variables**: API credentials read from environment variables, never stored in config files
- **Configurable Env Var Names**: Customize which environment variable names are used via `[env]` section
- **Fully Configurable**: Customizable commit behavior, ignore patterns, and AI prompts

## Installation

### Prerequisites

- Rust (latest stable version)
- Git repository
- An OpenAI-compatible API endpoint and key

### Build from Source

```bash
git clone <repository-url>
cd ai-commit
cargo build --release
```

### Setup

1. **Set environment variables**:

   ```bash
   export AI_COMMIT_API_KEY="your-api-key-here"
   export AI_COMMIT_MODEL="your-model-name"
   # Optional: override the default endpoint
   export AI_COMMIT_ENDPOINT="https://your-api-endpoint/chat/completions"
   ```

2. **Initialize configuration** (optional, creates default config with prompt templates):

   ```bash
   ai-commit config init
   ```

## Usage

### Basic Commands

Generate commit message for staged changes:

```bash
ai-commit
# or explicitly
ai-commit commit
```

Generate commit message with keywords to guide AI:

```bash
ai-commit -k "fix authentication bug"
ai-commit commit -k "add user profile feature"
```

Preview commit message without committing (dry-run mode):

```bash
ai-commit --dry-run
ai-commit commit --dry-run
```

Limit context sent to AI:

```bash
ai-commit --context-limit 100000
```

Amend the last commit with new changes:

```bash
ai-commit amend
ai-commit amend -k "improve error handling"
```

Combine options:

```bash
ai-commit commit -k "improve performance" --dry-run --context-limit 50000
ai-commit amend -k "add validation" --dry-run
```

### Configuration Commands

```bash
# Initialize default configuration
ai-commit config init

# View current configuration (shows env var names, not values)
ai-commit config show

# Get help with editing prompts
ai-commit config edit-prompts
```

### Git Hooks Integration

Install git hooks for automatic commit message assistance:

```bash
ai-commit install
```

Remove git hooks:

```bash
ai-commit uninstall
```

## Configuration

The tool stores configuration in `~/.config/ai-commit/config.toml`. Initialize with default settings:

```bash
ai-commit config init
```

### Configuration Structure

```toml
# Customize which environment variable names to read
# Defaults shown below — only uncomment and change if needed
[env]
# endpoint_env = "AI_COMMIT_ENDPOINT"
# api_key_env = "AI_COMMIT_API_KEY"
# model_env = "AI_COMMIT_MODEL"

[commit]
auto_confirm = false
dry_run_by_default = false
ignore_lock_files = true
custom_ignore_patterns = []
context_limit = 200000

[hooks]
enabled = false
hook_types = []

[prompts]
system_prompt = """You are an expert software developer..."""
user_prompt_template = """Analyze the following git diff...
```diff
{diff}
```"""
```

### Configuration Options

#### Environment Variables (`[env]`)

Customize the environment variable names used to read API credentials. This lets you integrate with existing CI/CD environments without setting extra variables.

| Field | Default | Description |
|-------|---------|-------------|
| `endpoint_env` | `AI_COMMIT_ENDPOINT` | Env var for API endpoint URL |
| `api_key_env` | `AI_COMMIT_API_KEY` | Env var for API key |
| `model_env` | `AI_COMMIT_MODEL` | Env var for model name |

Example — using OpenAI-compatible variables:

```toml
[env]
api_key_env = "OPENAI_API_KEY"
model_env = "OPENAI_MODEL"
endpoint_env = "OPENAI_BASE_URL"
```

#### Commit Settings (`[commit]`)

| Field | Default | Description |
|-------|---------|-------------|
| `auto_confirm` | `false` | Skip confirmation prompt |
| `dry_run_by_default` | `false` | Always run in dry-run mode |
| `ignore_lock_files` | `true` | Filter out lock files from analysis |
| `custom_ignore_patterns` | `[]` | Glob patterns for files to ignore (e.g. `["**/generated/**"]`) |
| `context_limit` | `200000` | Maximum characters of diff sent to AI |

#### Prompt Settings (`[prompts]`)

| Field | Description |
|-------|-------------|
| `system_prompt` | System prompt that defines AI behavior and commit format |
| `user_prompt_template` | Template for analyzing diffs — must contain `{diff}` placeholder |

### Customizing AI Prompts

Edit `~/.config/ai-commit/config.toml` to customize how the AI generates commit messages:

```toml
[prompts]
system_prompt = """You are a senior developer focused on clear, concise commits.
Generate conventional commit messages prioritizing single-line format.
Use bullet points only for truly unrelated changes."""

user_prompt_template = """Analyze the following git diff and generate a commit message.
Prefer single-line format under 72 characters.

Git diff:
```diff
{diff}
```

Provide only the commit message."""
```

**Tips:**
- Keep the `{diff}` placeholder in templates
- Test changes with `ai-commit --dry-run`
- Configuration reloads automatically on next run

## Commit Message Format

The tool generates messages following the Conventional Commits specification.

### Single-line Format (preferred)

Used for focused changes with a single purpose:

```
feat: add user authentication system
fix: resolve database connection timeout
refactor: improve error handling in auth module
```

### Multi-line Format (for complex changes)

Used when there are multiple truly unrelated functional changes:

```
feat: add user management and notification system

- Implement user CRUD operations with validation
- Add email notification service for user events
- Create admin dashboard for user management
```

### Supported Types

| Type | Description |
|------|-------------|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation only changes |
| `style` | Code style changes (formatting, etc.) |
| `refactor` | Code changes that neither fix bugs nor add features |
| `perf` | Performance improvements |
| `test` | Adding or correcting tests |
| `chore` | Build process or auxiliary tool changes |

## Workflow Examples

### Initial Setup

```bash
export AI_COMMIT_API_KEY="your-api-key"
export AI_COMMIT_MODEL="your-model"
ai-commit config init
ai-commit config show
```

### Standard Workflow

```bash
git add .
ai-commit
# Review generated message and confirm
```

### Amend Workflow

```bash
git add .
ai-commit amend
# Review generated message for combined changes
```

### Preview Changes

```bash
git add .
ai-commit --dry-run
```

### Using a Different AI Provider

```bash
export AI_COMMIT_API_KEY="sk-..."
export AI_COMMIT_MODEL="gpt-4o"
export AI_COMMIT_ENDPOINT="https://api.openai.com/v1/chat/completions"
ai-commit commit
```

Or configure custom env var names in `config.toml`:

```toml
[env]
api_key_env = "OPENAI_API_KEY"
model_env = "OPENAI_MODEL"
endpoint_env = "OPENAI_BASE_URL"
```

## Technical Details

### Diff Analysis

- Analyzes git diffs to understand code changes
- Filters out lock files and build artifacts automatically
- Supports glob-based custom ignore patterns
- Enforces configurable context limit to prevent oversized API requests
- Supports both staged and unstaged change analysis

### AI Integration

- Compatible with any OpenAI-compatible chat completions API
- API credentials managed through environment variables only (never stored in files)
- HTTP connection timeout (10s connect, 60s total)
- Handles API errors gracefully with proper exit codes
- Supports fully customizable prompts for different commit styles

### Security

- API keys read from environment variables, never written to config files
- `config show` displays env var names only, never actual values
- Works with GPG-signed commits
- Respects git configuration settings

## Contributing

Contributions are welcome! Please feel free to:

- Submit bug reports and feature requests through issues
- Create pull requests for improvements
- Share feedback and suggestions

### Development Setup

```bash
git clone <repository-url>
cd ai-commit
cargo test
cargo clippy
cargo run -- --help
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.
