use crate::config::AppConfig;
use anyhow::Result;
use colored::*;
use std::io::Write;

pub fn init_config() -> Result<()> {
    println!("{}", "🔧 Initializing AI Commit configuration...".cyan());

    let config_path = AppConfig::config_path()?;

    if config_path.exists() {
        println!("{}", "⚠️  Configuration file already exists.".yellow());
        println!("Location: {}", config_path.display().to_string().bright_blue());

        print!("Do you want to overwrite it? (y/N): ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("{}", "❌ Configuration initialization cancelled.".red());
            return Ok(());
        }
    }

    let config = AppConfig::default();
    config.save()?;

    println!("{}", "✅ Configuration file created successfully!".green());
    println!("Location: {}", config_path.display().to_string().bright_blue());
    println!();
    println!("{}", "📝 Next steps:".bright_cyan().bold());
    println!("1. Set your API key: {}", "export AI_COMMIT_API_KEY=\"your-api-key\"".yellow());
    println!("2. Set your model: {}", "export AI_COMMIT_MODEL=\"your-model\"".yellow());
    println!("3. View configuration: {}", "ai-commit config show".yellow());
    println!("4. Edit prompts: Edit the [prompts] section in the config file");
    println!("5. Start using: {}", "ai-commit".yellow());

    Ok(())
}

pub fn show_config() -> Result<()> {
    println!("{}", "📋 Current AI Commit Configuration".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════".bright_blue());

    let config_path = AppConfig::config_path()?;

    if !config_path.exists() {
        println!("{}", "❌ Configuration file not found.".red());
        println!("Run {} to create it.", "ai-commit config init".yellow());
        return Ok(());
    }

    let config = AppConfig::load()?;

    println!("File location: {}", config_path.display().to_string().bright_blue());
    println!();
    println!("[env]");
    println!("  endpoint_env = \"{}\"", config.env.endpoint_env.bright_green());
    println!("  api_key_env = \"{}\"", config.env.api_key_env.bright_green());
    println!("  model_env = \"{}\"", config.env.model_env.bright_green());
    println!();
    println!("[commit]");
    println!("  auto_confirm = {}", config.commit.auto_confirm);
    println!("  dry_run_by_default = {}", config.commit.dry_run_by_default);
    println!("  ignore_lock_files = {}", config.commit.ignore_lock_files);
    println!("  context_limit = {}", config.commit.context_limit);
    println!("  custom_ignore_patterns = {:?}", config.commit.custom_ignore_patterns);
    println!();
    println!("[prompts]");
    let sys_len = config.prompts.system_prompt.len().min(50);
    println!("  system_prompt = \"{}...\"", &config.prompts.system_prompt[..sys_len]);
    let user_len = config.prompts.user_prompt_template.len().min(50);
    println!("  user_prompt_template = \"{}...\"", &config.prompts.user_prompt_template[..user_len]);

    Ok(())
}

pub fn edit_prompts_help() -> Result<()> {
    let config_path = AppConfig::config_path()?;

    println!("{}", "✏️  How to Edit AI Prompts".bright_cyan().bold());
    println!("{}", "═══════════════════════════".bright_blue());
    println!();
    println!("Configuration file location:");
    println!("{}", config_path.display().to_string().bright_blue());
    println!();
    println!("{}", "📝 Editable prompt sections:".bright_green().bold());
    println!();
    println!("{}", "[prompts.system_prompt]".yellow());
    println!("  • Defines AI behavior and commit format preferences");
    println!("  • Sets the overall style and rules for commit messages");
    println!();
    println!("{}", "[prompts.user_prompt_template]".yellow());
    println!("  • Template for analyzing git diffs");
    println!("  • Use {{diff}} as placeholder for the git diff content");
    println!("  • Controls how AI analyzes changes");
    println!();
    println!("{}", "[env]".yellow());
    println!("  • Customize environment variable names for API configuration");
    println!("  • endpoint_env, api_key_env, model_env");
    println!();
    println!("{}", "💡 Tips:".bright_green().bold());
    println!("  • Test changes with: ai-commit --dry-run");
    println!("  • Keep {{diff}} placeholder in templates");
    println!("  • Reload happens automatically on next run");
    println!("  • Back up your custom prompts before updates");

    if !config_path.exists() {
        println!();
        println!("{}", "⚠️  Configuration file not found.".yellow());
        println!("Run {} to create it first.", "ai-commit config init".cyan());
    }

    Ok(())
}
