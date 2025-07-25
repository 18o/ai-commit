use anyhow::Result;
pub mod amend;
pub mod commit;
pub mod config;
pub mod install;
pub mod uninstall;

pub async fn execute_command(command: &str) -> Result<()> {
    match command {
        "install" => install::install_hook(),
        "uninstall" => uninstall::uninstall_hook(),
        "commit" => commit::handle_commit().await,
        "amend" => amend::handle_amend().await,
        "config-init" => config::init_config(),
        "config-show" => config::show_config(),
        "config-edit-prompts" => config::edit_prompts_help(),
        _ => Err(anyhow::anyhow!("Unknown command: {}", command)),
    }
}
