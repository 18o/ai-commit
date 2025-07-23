use anyhow::Result;
pub mod commit;
pub mod install;
pub mod uninstall;

pub async fn execute_command(command: &str) -> Result<()> {
    match command {
        "install" => install::install_hook(),
        "uninstall" => uninstall::uninstall_hook(),
        "commit" => commit::handle_commit().await,
        _ => Err(anyhow::anyhow!("Unknown command: {}", command)),
    }
}
