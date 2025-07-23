use std::process::Command;

use anyhow::Result;

use crate::ai::AiClient;
use git2::{DiffOptions, Repository, Signature};

pub async fn handle_commit() -> Result<()> {
    let ai_client = AiClient::new();

    // 获取diff内容
    let mut diff_content = get_staged_diff()?;
    if diff_content.is_empty() {
        println!("No staged changes found. Checking for unstaged changes...");
        diff_content = get_unstaged_diff()?;
        if diff_content.is_empty() {
            println!("No changes found to commit.");
            return Ok(());
        }
        println!("Found unstaged changes. Please stage them first with 'git add'.");
        return Ok(());
    }

    println!("Staged changes found. Generating commit message...");
    println!("Generating commit message using AI service...");

    match ai_client.generate_commit_message(&diff_content).await {
        Ok(message) => {
            println!("AI generated commit message: {message}");
            // 如果AI生成的消息为空，使用默认消息
            if message.is_empty() {
                println!("AI did not generate a commit message, using default.");
                return Ok(());
            }
            println!("Generated commit message:");
            println!("─────────────────────");
            println!("{message}");
            println!("─────────────────────");

            // 询问用户是否确认提交
            if !confirm_commit()? {
                println!("Commit cancelled.");
                return Ok(());
            }

            // 执行commit，考虑GPG签名
            execute_commit(&message)?;
        }
        Err(e) => {
            println!("Failed to generate commit message: {e}");
        }
    };

    Ok(())
}

fn confirm_commit() -> Result<bool> {
    use std::io::{self, Write};

    print!("Do you want to commit with this message? (y/N): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

fn execute_commit(message: &str) -> Result<()> {
    // 方法1: 使用git命令行 (推荐，自动处理GPG签名)
    execute_commit_with_cli(message)

    // 方法2: 使用git2库 (需要手动处理GPG)
    // execute_commit_with_git2(message)
}

fn execute_commit_with_cli(message: &str) -> Result<()> {
    println!("Committing changes...");

    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", message]);

    // 检查是否需要GPG签名
    if is_gpg_signing_enabled()? {
        println!("GPG signing is enabled, using git command for proper signing...");
        // git命令会自动处理GPG签名
    }

    let status = cmd.status()?;

    if status.success() {
        println!("✅ Commit successful!");

        // 显示提交信息
        show_commit_info()?;
    } else {
        return Err(anyhow::anyhow!("Commit failed"));
    }

    Ok(())
}

fn execute_commit_with_git2(message: &str) -> Result<()> {
    let repo = Repository::open_from_env()?;
    let signature = get_git_signature(&repo)?;
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;

    let parent_commit = match repo.head() {
        Ok(head) => Some(head.peel_to_commit()?),
        Err(_) => None,
    };

    let parents = match &parent_commit {
        Some(commit) => vec![commit],
        None => vec![],
    };

    let commit_oid = repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &parents)?;

    println!("✅ Commit successful! {commit_oid}");
    Ok(())
}

fn get_git_signature(repo: &Repository) -> Result<Signature> {
    // 尝试从git config获取用户信息
    let config = repo.config()?;
    let name = config.get_string("user.name").unwrap_or_else(|_| "Unknown".to_string());
    let email = config.get_string("user.email").unwrap_or_else(|_| "unknown@example.com".to_string());

    Ok(Signature::now(&name, &email)?)
}

fn is_gpg_signing_enabled() -> Result<bool> {
    let output = Command::new("git").args(["config", "--get", "commit.gpgsign"]).output()?;
    let status = output.status;
    let stdout = output.stdout;
    if status.success() {
        let value_cow = String::from_utf8_lossy(&stdout);
        let value = value_cow.trim();
        Ok(value == "true")
    } else {
        // 检查全局配置
        let output = Command::new("git").args(["config", "--global", "--get", "commit.gpgsign"]).output()?;

        if output.status.success() {
            let value_cow = String::from_utf8_lossy(&output.stdout);
            let value = value_cow.trim();
            Ok(value == "true")
        } else {
            Ok(false)
        }
    }
}

fn show_commit_info() -> Result<()> {
    let output = Command::new("git").args(["log", "-1", "--oneline"]).output()?;

    if output.status.success() {
        let commit_info = String::from_utf8_lossy(&output.stdout);
        println!("Latest commit: {}", commit_info.trim());
    }

    Ok(())
}

fn get_staged_diff() -> Result<String> {
    let repo = Repository::open_from_env().unwrap_or_else(|e| {
        eprintln!("Failed to open git repository. Make sure you're in a git repository: {e}");
        std::process::exit(1);
    });

    let head = repo.head()?.peel_to_tree()?;
    let mut index = repo.index()?;
    let index_tree = repo.find_tree(index.write_tree()?)?;

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(3);

    let diff = repo.diff_tree_to_tree(Some(&head), Some(&index_tree), Some(&mut diff_opts))?;

    let mut diff_content = String::new();

    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        if let Ok(content) = str::from_utf8(line.content()) {
            diff_content.push_str(content);
        }
        true
    })?;

    Ok(diff_content)
}

fn get_unstaged_diff() -> Result<String> {
    let repo = Repository::open_from_env().unwrap_or_else(|e| {
        eprintln!("Failed to open git repository. Make sure you're in a git repository: {e}");
        std::process::exit(1);
    });

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(3);
    diff_opts.include_untracked(false); // 不包含未跟踪文件

    // 比较索引和工作目录
    let diff = repo.diff_index_to_workdir(None, Some(&mut diff_opts))?;

    format_diff(diff)
}

fn format_diff(diff: git2::Diff) -> Result<String> {
    let mut diff_content = String::new();

    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        if let Ok(content) = str::from_utf8(line.content()) {
            diff_content.push_str(content);
        }
        true
    })?;

    Ok(diff_content)
}
