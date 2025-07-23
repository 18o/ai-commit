use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub commit: CommitConfig,
    pub hooks: HookConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub endpoint: String,
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

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                endpoint: "https://ark.cn-beijing.volces.com/api/v3/chat/completions".to_string(),
                model: "doubao-1-5-pro-32k-250115".to_string(),
                max_tokens: Some(1000),
                temperature: Some(0.7),
                context_limit: 200000,
            },
            commit: CommitConfig {
                auto_confirm: false,
                dry_run_by_default: false,
                gpg_sign: None,
                ignore_lock_files: true,
                custom_ignore_patterns: vec![],
            },
            hooks: HookConfig { enabled: false, hook_types: vec!["prepare-commit-msg".to_string()] },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

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

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

        Ok(config_dir.join("ai-commit").join("config.toml"))
    }

    pub fn get_api_key() -> Result<String> {
        std::env::var("AI_COMMIT_ARK_API_KEY")
            .or_else(|_| std::env::var("ARK_API_KEY"))
            .map_err(|_| anyhow::anyhow!("API key not found. Please set AI_COMMIT_ARK_API_KEY environment variable"))
    }
}
