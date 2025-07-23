mod settings;

pub mod prompt;
pub use settings::AppConfig;

use anyhow::Result;

pub fn show_config() -> Result<()> {
    let config = AppConfig::load()?;
    println!("Current configuration:");
    println!("{}", toml::to_string_pretty(&config)?);
    Ok(())
}

pub fn init_config() -> Result<()> {
    let config = AppConfig::default();
    config.save()?;
    let config_path = AppConfig::config_path()?;
    println!("Configuration file created at: {}", config_path.display());
    println!("You can now edit the prompts and other settings in this file.");
    Ok(())
}

pub fn edit_prompts() -> Result<()> {
    let config_path = AppConfig::config_path()?;
    println!("Edit your configuration file at: {}", config_path.display());
    println!("\nYou can customize the following prompt sections:");
    println!("- [prompts.system_prompt]: The system prompt that defines AI behavior");
    println!("- [prompts.user_prompt_template]: Template for analyzing diffs (use {{diff}} placeholder)");
    println!("- [prompts.simple_prompt_template]: Template for simple single-line messages");
    Ok(())
}
