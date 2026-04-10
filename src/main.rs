#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use ai_commit::commands::{amend, commit, config, install, uninstall};
use anyhow::Result;
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let matches = Command::new("ai-commit")
        .version(env!("CARGO_PKG_VERSION"))
        .about("AI-assisted Git commit message generator (defaults to 'commit' if no subcommand)")
        .author("John")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .arg(
            Arg::new("keywords")
                .short('k')
                .long("keywords")
                .value_name("KEYWORDS")
                .help("Keywords or context to guide AI commit message generation (implies 'commit' command)")
                .global(false),
        )
        .subcommand(Command::new("install").about("Install git hooks for AI commit assistance"))
        .subcommand(Command::new("uninstall").about("Remove AI commit hooks"))
        .subcommand(
            Command::new("commit")
                .about("Generate AI commit message for staged changes")
                .arg(
                    Arg::new("context-limit")
                        .long("context-limit")
                        .value_name("CHARS")
                        .help("Maximum characters to send to AI (default: from config)")
                        .value_parser(clap::value_parser!(usize)),
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .help("Show generated message without committing")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("keywords")
                        .short('k')
                        .long("keywords")
                        .value_name("KEYWORDS")
                        .help("Keywords or context to guide AI commit message generation"),
                ),
        )
        .subcommand(
            Command::new("amend")
                .about("Amend the last commit with staged changes using AI-generated message")
                .arg(
                    Arg::new("context-limit")
                        .long("context-limit")
                        .value_name("CHARS")
                        .help("Maximum characters to send to AI (default: from config)")
                        .value_parser(clap::value_parser!(usize)),
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .help("Show generated message without amending")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("keywords")
                        .short('k')
                        .long("keywords")
                        .value_name("KEYWORDS")
                        .help("Keywords or context to guide AI commit message generation"),
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

    match matches.subcommand() {
        Some(("install", _)) => install::install_hook(),
        Some(("uninstall", _)) => uninstall::uninstall_hook(),
        Some(("amend", sub_matches)) => {
            let keywords = sub_matches.get_one::<String>("keywords").map(|s| s.as_str());
            let dry_run = sub_matches.get_flag("dry-run");
            let context_limit = sub_matches.get_one::<usize>("context-limit").copied();
            amend::handle_amend(keywords, dry_run, context_limit).await
        }
        Some(("commit", sub_matches)) => {
            let keywords = sub_matches.get_one::<String>("keywords").map(|s| s.as_str());
            let dry_run = sub_matches.get_flag("dry-run");
            let context_limit = sub_matches.get_one::<usize>("context-limit").copied();
            commit::handle_commit(keywords, dry_run, context_limit).await
        }
        Some(("config", sub_matches)) => match sub_matches.subcommand() {
            Some(("show", _)) => config::show_config(),
            Some(("init", _)) => config::init_config(),
            Some(("edit-prompts", _)) => config::edit_prompts_help(),
            _ => config::show_config(),
        },
        Some((_, sub_matches)) => {
            let keywords = sub_matches.get_one::<String>("keywords").map(|s| s.as_str());
            let dry_run = sub_matches.get_flag("dry-run");
            let context_limit = sub_matches.get_one::<usize>("context-limit").copied();
            commit::handle_commit(keywords, dry_run, context_limit).await
        }
        _ => {
            let keywords = matches.get_one::<String>("keywords").map(|s| s.as_str());
            commit::handle_commit(keywords, false, None).await
        }
    }
}
