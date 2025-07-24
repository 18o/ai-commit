use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub commit: CommitConfig,
    pub hooks: HookConfig,
    pub prompts: PromptConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub endpoint: String,
    pub api_key: String,
    pub model: String,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub context_limit: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitConfig {
    pub auto_confirm: bool,
    pub dry_run_by_default: bool,
    pub gpg_sign: Option<bool>,
    pub ignore_lock_files: bool,
    pub custom_ignore_patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HookConfig {
    pub enabled: bool,
    pub hook_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptConfig {
    pub system_prompt: String,
    pub user_prompt_template: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig::load().unwrap_or_else(|_| {
            eprintln!("Failed to load configuration, using default settings.");
            std::process::exit(1);
        })
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        debug!("Loading configuration from: {}", config_path.display());
        if config_path.exists() {
            let config_content = fs::read_to_string(&config_path)?;
            let config: AppConfig = toml::from_str(&config_content)?;
            Ok(config)
        } else {
            let default_config = AppConfig::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_content = toml::to_string_pretty(self)?;
        fs::write(&config_path, config_content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("Could not find HOME directory"))?;
        Ok(PathBuf::from(home).join(".config").join("ai-commit").join("config.toml"))
    }

    pub fn get_api_key() -> Result<String> {
        std::env::var("AI_COMMIT_ARK_API_KEY")
            .or_else(|_| std::env::var("ARK_API_KEY"))
            .map_err(|_| anyhow::anyhow!("API key not found. Please set AI_COMMIT_ARK_API_KEY environment variable"))
    }
}
