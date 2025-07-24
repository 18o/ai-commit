# AI Commit Tool

An intelligent Git commit message generator that uses AI to analyze your code changes and create meaningful, conventional commit messages automatically.

## Overview

AI Commit Tool integrates with your Git workflow to automatically generate high-quality commit messages following the Conventional Commits specification. It analyzes your staged or unstaged changes and uses AI to craft appropriate commit messages, saving time and ensuring consistency across your project.

## Features

- **AI-Generated Commit Messages**: Automatically analyzes git diffs and generates contextual commit messages following conventional commit format
- **Dry Run Mode**: Preview generated messages for unstaged changes without committing
- **Amend Support**: Generate new messages for amending previous commits with additional changes
- **Lock File Filtering**: Automatically ignores common lock files (Cargo.lock, package-lock.json, yarn.lock, etc.) from analysis
- **GPG Signing Support**: Works seamlessly with GPG-signed commits
- **Configurable**: Customizable API settings, ignore patterns, behavior options, and AI prompts

## Installation

### Prerequisites

- Rust (latest stable version)
- Git repository
- Doubao AI API key (currently only support Doubao)

### Build from Source

```bash
git clone <repository-url>
cd ai-commit
cargo build --release
```

## Usage

### Basic Commands

Generate commit message for staged changes:

```bash
ai-commit
# or explicitly
ai-commit commit
```

Preview commit message for unstaged changes (dry-run mode):

```bash
# Will automatically enter dry-run mode if no staged changes found
ai-commit
```

Amend the last commit with new changes:

```bash
ai-commit amend
```

### Command Options

```bash
# Show generated message without committing
ai-commit --dry-run

# Limit context sent to AI (default: 200000 characters)
ai-commit --context-limit 100000

# Amend with dry-run
ai-commit amend --dry-run
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

The tool supports configuration through `~/.config/ai-commit/config.toml`:

```toml
[api]
endpoint = "https://ark.cn-beijing.volces.com/api/v3/chat/completions"
model = "doubao-1-5-pro-32k-250115"
max_tokens = 1000
temperature = 0.7
context_limit = 200000

[commit]
auto_confirm = false
dry_run_by_default = false
ignore_lock_files = true
custom_ignore_patterns = []

[hooks]
enabled = false
hook_types = ["prepare-commit-msg"]

[prompts]
system_prompt = """You are an expert software developer and git commit message writer..."""
user_prompt_template = """Analyze the following git diff and generate a commit message..."""
simple_prompt_template = """Generate a concise single-line commit message..."""
```

### Configuration Management

Initialize default configuration:

```bash
ai-commit config init
```

View current configuration:

```bash
ai-commit config show
```

Get help with editing prompts:

```bash
ai-commit config edit-prompts
```

### Configuration Options

#### API Settings (`[api]`)

- `endpoint`: AI service endpoint URL
- `model`: AI model to use for generation
- `max_tokens`: Maximum tokens for AI response
- `temperature`: Creativity level (0.0-1.0)
- `context_limit`: Maximum characters to send to AI

#### Commit Settings (`[commit]`)

- `auto_confirm`: Skip confirmation prompt
- `dry_run_by_default`: Always run in dry-run mode
- `ignore_lock_files`: Filter out lock files from analysis
- `custom_ignore_patterns`: Additional file patterns to ignore

#### Hook Settings (`[hooks]`)

- `enabled`: Enable git hooks integration
- `hook_types`: Types of git hooks to install

#### Prompt Settings (`[prompts]`)

- `system_prompt`: System prompt that defines AI behavior
- `user_prompt_template`: Template for analyzing diffs (use `{diff}` placeholder)
- `simple_prompt_template`: Template for simple single-line messages

### Customizing AI Prompts

You can customize how the AI generates commit messages by editing the prompt templates in your configuration file. The prompts support:

- **System Prompt**: Defines the AI's role and commit format preferences
- **User Prompt Template**: Instructions for analyzing diffs and generating messages
- **Simple Prompt Template**: Focused template for concise single-line messages

Example custom prompt:

````toml
[prompts]
system_prompt = """You are a senior developer focused on clear, concise commits.
Generate conventional commit messages prioritizing single-line format.
Use bullet points only for truly unrelated changes."""

user_prompt_template = """Generate a commit message for these changes.
Prefer single-line format under 72 characters.

Git diff:
```diff
{diff}
````

Provide only the commit message."""

```

## Commit Message Format

The tool generates messages following the Conventional Commits specification:

### Single-line Format (preferred)

```

feat: add user authentication system
fix: resolve database connection timeout
refactor: improve error handling in auth module

```

### Multi-line Format (for complex changes)

```

feat: add user management and notification system

- Implement user CRUD operations with validation
- Add email notification service for user events
- Create admin dashboard for user management

````

### Supported Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code changes that neither fix bugs nor add features
- `perf`: Performance improvements
- `test`: Adding or correcting tests
- `chore`: Build process or auxiliary tool changes

## Workflow Examples

### Standard Workflow

```bash
# Make changes to your code
git add .
ai-commit
# Review generated message and confirm
````

### Amend Workflow

```bash
# Make additional changes after last commit
git add .
ai-commit amend
# Review generated message for combined changes
```

### Preview Changes

```bash
# Check what message would be generated without committing
git add .
ai-commit --dry-run
```

### Configuration Workflow

```bash
# Initialize configuration
ai-commit config init

# Customize settings in ~/.config/ai-commit/config.toml
# Then use normally
git add .
ai-commit
```

## Technical Details

### Diff Analysis

- Analyzes git diffs to understand code changes
- Filters out lock files and build artifacts automatically
- Considers file types, change patterns, and modification scope
- Supports both staged and unstaged change analysis

### AI Integration

- Uses advanced language models for commit message generation
- Sends contextual diff information for accurate analysis
- Respects token limits and context windows
- Handles API errors gracefully with fallback messages
- Supports customizable prompts for different commit styles

### Configuration Management

- XDG Base Directory specification compliant
- TOML format for easy editing
- Environment variable support for API keys
- Fallback to defaults if configuration is missing
- Hot-reload of configuration changes

### Security

- Works with GPG-signed commits
- Respects git configuration settings
- No code or sensitive information stored externally
- API keys managed through environment variables
- Local configuration files only

## Contributing

Contributions are welcome! Please feel free to:

- Submit bug reports and feature requests through issues
- Create pull requests for improvements
- Share feedback and suggestions
- Help improve documentation

### Development Setup

```bash
git clone <repository-url>
cd ai-commit
cargo test
cargo run -- --help
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.
