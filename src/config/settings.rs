use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub commit: CommitConfig,
    #[serde(default)]
    pub hooks: HookConfig,
    #[serde(default)]
    pub prompts: PromptConfig,
    #[serde(default)]
    pub env: EnvConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvConfig {
    #[serde(default = "default_endpoint_env")]
    pub endpoint_env: String,
    #[serde(default = "default_api_key_env")]
    pub api_key_env: String,
    #[serde(default = "default_model_env")]
    pub model_env: String,
    #[serde(default = "default_max_tokens_env")]
    pub max_tokens_env: String,
    #[serde(default = "default_temperature_env")]
    pub temperature_env: String,
}

fn default_endpoint_env() -> String {
    "AI_COMMIT_ENDPOINT".into()
}
fn default_api_key_env() -> String {
    "AI_COMMIT_API_KEY".into()
}
fn default_model_env() -> String {
    "AI_COMMIT_MODEL".into()
}
fn default_max_tokens_env() -> String {
    "AI_COMMIT_MAX_TOKENS".into()
}
fn default_temperature_env() -> String {
    "AI_COMMIT_TEMPERATURE".into()
}

pub struct ApiConfig {
    pub endpoint: String,
    pub api_key: String,
    pub model: String,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
}

impl ApiConfig {
    pub fn from_env(env_config: &EnvConfig) -> Result<Self> {
        let endpoint = std::env::var(&env_config.endpoint_env)
            .unwrap_or_else(|_| "https://ark.cn-beijing.volces.com/api/v3/chat/completions".to_string());
        let api_key = std::env::var(&env_config.api_key_env)
            .map_err(|_| anyhow::anyhow!("API key not found. Set {} environment variable", env_config.api_key_env))?;
        let model = std::env::var(&env_config.model_env)
            .map_err(|_| anyhow::anyhow!("Model not found. Set {} environment variable", env_config.model_env))?;
        let max_tokens =
            std::env::var(&env_config.max_tokens_env).ok().and_then(|v| v.parse::<usize>().ok()).unwrap_or(4096);
        let temperature =
            std::env::var(&env_config.temperature_env).ok().and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.7);

        Ok(Self { endpoint, api_key, model, max_tokens: Some(max_tokens), temperature: Some(temperature) })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitConfig {
    #[serde(default = "default_false")]
    pub auto_confirm: bool,
    #[serde(default = "default_false")]
    pub dry_run_by_default: bool,
    pub gpg_sign: Option<bool>,
    #[serde(default = "default_true")]
    pub ignore_lock_files: bool,
    #[serde(default)]
    pub custom_ignore_patterns: Vec<String>,
    #[serde(default = "default_context_limit")]
    pub context_limit: usize,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_false() -> bool {
    false
}
fn default_true() -> bool {
    true
}
fn default_context_limit() -> usize {
    800000
}

fn default_language() -> String {
    "en".to_string()
}

impl Default for CommitConfig {
    fn default() -> Self {
        Self {
            auto_confirm: false,
            dry_run_by_default: false,
            gpg_sign: None,
            ignore_lock_files: true,
            custom_ignore_patterns: Vec::new(),
            context_limit: 800000,
            language: default_language(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HookConfig {
    #[serde(default = "default_false")]
    pub enabled: bool,
    #[serde(default)]
    pub hook_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptConfig {
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,
    #[serde(default = "default_system_prompt_zh")]
    pub system_prompt_zh: String,
    #[serde(default = "default_user_prompt_template")]
    pub user_prompt_template: String,
    #[serde(default = "default_user_prompt_template_zh")]
    pub user_prompt_template_zh: String,
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

IMPORTANT: ALWAYS write the commit message in English, regardless of the language used in the diff content.

PREFERRED FORMAT: Single line under 72 characters
Use bullet points ONLY when there are truly MULTIPLE UNRELATED functional changes.

Default to single line. Only use bullets for truly unrelated changes."#
        .to_string()
}

fn default_system_prompt_zh() -> String {
    r#"你是一位专业的软件开发工程师和 Git 提交信息编写专家。

生成简洁、清晰的提交信息，遵循约定式提交规范。
重要：提交信息必须使用中文编写，类型（type）部分可以使用英文。

格式规范：
- feat: 新功能
- fix: 修复 bug
- docs: 仅文档变更
- style: 不影响代码含义的格式变更
- refactor: 代码重构，既不修复 bug 也不添加新功能
- perf: 性能优化
- test: 添加或修正测试
- chore: 构建过程或辅助工具的变更

格式：类型(范围): 描述
示例：
- feat: 添加用户认证功能
- fix: 修复登录超时问题
- refactor: 优化错误处理逻辑

首选格式：单行，不超过 72 个字符
仅当存在多个完全不相关的功能变更时才使用多行（列表）格式。

默认使用单行格式。只有在真正不相关的变更时才使用列表。

严格要求：输出必须是中文提交信息（类型部分除外）。"#
        .to_string()
}

fn default_user_prompt_template() -> String {
    r#"Analyze the following git diff and generate a commit message.

Git diff:
```diff
{diff}
```

REQUIREMENTS (follow strictly):
1. Write the commit message in English ONLY. Never use Chinese or any other language, even if the diff contains non-English text.
2. Default to a single descriptive line under 72 characters.
3. Only use bullet points if there are multiple COMPLETELY UNRELATED functional changes.

Output ONLY the commit message, nothing else."#
        .to_string()
}

fn default_user_prompt_template_zh() -> String {
    r#"分析以下 git diff 并生成中文提交信息。

重要要求：
1. 提交信息描述部分必须使用中文
2. 类型（type）可以使用英文，如 feat、fix 等
3. 默认使用单行描述，不超过 72 个字符
4. 仅当存在多个完全不相关的功能变更时才使用列表格式

示例输出格式：
- feat: 添加多语言支持
- fix: 修复登录超时问题
- refactor: 重构数据处理逻辑

Git diff:
```diff
{diff}
```

请只输出中文提交信息，不要包含其他内容。"#
        .to_string()
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            system_prompt: default_system_prompt(),
            system_prompt_zh: default_system_prompt_zh(),
            user_prompt_template: default_user_prompt_template(),
            user_prompt_template_zh: default_user_prompt_template_zh(),
        }
    }
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            endpoint_env: default_endpoint_env(),
            api_key_env: default_api_key_env(),
            model_env: default_model_env(),
            max_tokens_env: default_max_tokens_env(),
            temperature_env: default_temperature_env(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        debug!("Loading configuration from: {}", config_path.display());
        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "Config file not found: {}. Run 'ai-commit config init' to create it.",
                config_path.display()
            ));
        }
        let config_content = fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&config_content)?;
        Ok(config)
    }

    pub fn load_or_create() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            Self::load()
        } else {
            let default = AppConfig::default();
            default.save()?;
            Ok(default)
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
        let home = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(home.join("ai-commit").join("config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default_no_panic() {
        let config = AppConfig::default();
        assert!(config.commit.ignore_lock_files);
        assert!(!config.commit.auto_confirm);
        assert_eq!(config.commit.context_limit, 800000);
    }

    #[test]
    fn test_env_config_default() {
        let env = EnvConfig::default();
        assert_eq!(env.endpoint_env, "AI_COMMIT_ENDPOINT");
        assert_eq!(env.api_key_env, "AI_COMMIT_API_KEY");
        assert_eq!(env.model_env, "AI_COMMIT_MODEL");
        assert_eq!(env.max_tokens_env, "AI_COMMIT_MAX_TOKENS");
        assert_eq!(env.temperature_env, "AI_COMMIT_TEMPERATURE");
    }

    #[test]
    fn test_commit_config_default() {
        let config = CommitConfig::default();
        assert!(!config.auto_confirm);
        assert!(config.ignore_lock_files);
        assert!(config.custom_ignore_patterns.is_empty());
        assert_eq!(config.context_limit, 800000);
    }

    #[test]
    fn test_prompt_config_default() {
        let config = PromptConfig::default();
        assert!(!config.system_prompt.is_empty());
        assert!(config.user_prompt_template.contains("{diff}"));
    }

    #[test]
    fn test_parse_minimal_config() {
        let toml_str = r#"
[prompts]
system_prompt = "test system"
user_prompt_template = "test {diff}"

[env]
endpoint_env = "MY_ENDPOINT"
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.prompts.system_prompt, "test system");
        assert_eq!(config.env.endpoint_env, "MY_ENDPOINT");
        assert_eq!(config.env.api_key_env, "AI_COMMIT_API_KEY");
    }

    #[test]
    fn test_parse_empty_config_uses_defaults() {
        let toml_str = "";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert!(config.commit.ignore_lock_files);
        assert_eq!(config.env.model_env, "AI_COMMIT_MODEL");
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let config = AppConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.commit.ignore_lock_files, config.commit.ignore_lock_files);
        assert_eq!(parsed.env.endpoint_env, config.env.endpoint_env);
    }
}
