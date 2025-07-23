use ai_commit::commands::execute_command;
use anyhow::Result;
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("ai-commit")
        .version("1.0.0")
        .about("AI-assisted Git commit message generator (defaults to 'commit' if no subcommand)")
        .author("John")
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
        .subcommand(
            Command::new("amend")
                .about("Amend the last commit with staged changes using AI-generated message")
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
                        .help("Show generated message without amending")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("config")
                .about("Manage configuration")
                .subcommand(Command::new("show").about("Show current configuration"))
                .subcommand(Command::new("init").about("Initialize default configuration"))
                .subcommand(Command::new("edit-prompts").about("Show how to edit prompt templates")),
        )
        .get_matches();

    let command = match matches.subcommand() {
        Some(("install", _)) => "install",
        Some(("uninstall", _)) => "uninstall",
        Some(("amend", _)) => "amend",
        Some(("commit", _)) => "commit",
        _ => "commit",
    };

    execute_command(command).await
}
