// Prompt 配置已统一由 AppConfig 管理。
// 默认 prompts 定义在 settings::PromptConfig::default() 中。
// 运行时 prompts 从已加载的 AppConfig 直接传给 AiClient，不再重复加载配置文件。

use crate::config::settings::PromptConfig;

/// 获取默认 system prompt（用于 config init 等非热路径场景）
pub fn default_system_prompt() -> String {
    PromptConfig::default().system_prompt
}

/// 获取默认 user prompt template
pub fn default_user_prompt_template() -> String {
    PromptConfig::default().user_prompt_template
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_system_prompt_not_empty() {
        let prompt = default_system_prompt();
        assert!(!prompt.is_empty());
        assert!(prompt.contains("Conventional Commits"));
    }

    #[test]
    fn test_default_user_prompt_template_has_placeholder() {
        let template = default_user_prompt_template();
        assert!(template.contains("{diff}"));
    }
}
