use anyhow::Result;
use clap::{Arg, Command};

use crate::commands::execute_command;

mod ai;
mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("ai-commit")
        .version("1.0.0")
        .about("AI-assisted Git commit message generator (defaults to 'commit' if no subcommand)")
        .author("Your Name")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(Command::new("install").about("Install git hooks for AI commit assistance"))
        .subcommand(Command::new("uninstall").about("Remove AI commit hooks"))
        .subcommand(
            Command::new("commit")
                .about("Generate AI commit message for staged changes")
                .arg(
                    Arg::new("context-limit")
                        .long("context-limit")
                        .value_name("CHARS")
                        .help("Maximum characters to send to AI (default: 200000)")
                        .value_parser(clap::value_parser!(usize)),
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .help("Show generated message without committing")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .get_matches();

    // 简化匹配逻辑
    let command = match matches.subcommand() {
        Some(("install", _)) => "install",
        Some(("uninstall", _)) => "uninstall",
        Some(("commit", _)) => "commit",
        _ => "commit", // 默认执行commit
    };

    execute_command(command).await
}
