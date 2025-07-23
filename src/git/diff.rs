use anyhow::Result;
use git2::{DiffOptions, Repository};
use std::str;

use crate::config::AppConfig;

pub fn get_staged_diff() -> Result<String> {
    let repo = Repository::open_from_env().unwrap_or_else(|e| {
        eprintln!("Failed to open git repository. Make sure you're in a git repository: {e}");
        std::process::exit(1);
    });

    let head = repo.head()?.peel_to_tree()?;
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let index_tree = repo.find_tree(oid)?;

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(3);

    let diff = repo.diff_tree_to_tree(Some(&head), Some(&index_tree), Some(&mut diff_opts))?;

    format_diff(diff)
}

pub fn get_unstaged_diff() -> Result<String> {
    let repo = Repository::open_from_env().unwrap_or_else(|e| {
        eprintln!("Failed to open git repository. Make sure you're in a git repository: {e}");
        std::process::exit(1);
    });

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(3);
    diff_opts.include_untracked(false);

    let diff = repo.diff_index_to_workdir(None, Some(&mut diff_opts))?;

    format_diff(diff)
}

fn format_diff(diff: git2::Diff) -> Result<String> {
    let mut diff_content = String::new();
    let config = AppConfig::load().unwrap_or_default();

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(path) = delta.new_file().path() {
            if config.commit.ignore_lock_files && should_ignore_file(path) {
                return true;
            }

            if should_ignore_by_custom_patterns(path, &config.commit.custom_ignore_patterns) {
                return true;
            }
        }

        if let Ok(content) = str::from_utf8(line.content()) {
            diff_content.push_str(content);
        }
        true
    })?;

    Ok(diff_content)
}

fn should_ignore_file(path: &std::path::Path) -> bool {
    let ignored_files = [
        "Cargo.lock",
        "bun.lock",
        "bun.lockb",
        "package-lock.json",
        "yarn.lock",
        "pnpm-lock.yaml",
        "poetry.lock",
        "Pipfile.lock",
        "composer.lock",
        "Gemfile.lock",
        "go.sum",
    ];

    if let Some(filename) = path.file_name() {
        if let Some(filename_str) = filename.to_str() {
            return ignored_files.contains(&filename_str);
        }
    }

    false
}

fn should_ignore_by_custom_patterns(path: &std::path::Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    for pattern in patterns {
        if path_str.contains(pattern) {
            return true;
        }
    }

    false
}

pub fn get_unstaged_diff_debug() -> Result<String> {
    let repo = Repository::open_from_env().unwrap_or_else(|e| {
        eprintln!("Failed to open git repository. Make sure you're in a git repository: {e}");
        std::process::exit(1);
    });

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(3);
    diff_opts.include_untracked(false);

    let diff = repo.diff_index_to_workdir(None, Some(&mut diff_opts))?;

    println!("Debug: Found {} deltas", diff.deltas().len());

    let mut diff_content = String::new();
    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(path) = delta.new_file().path() {
            println!("Debug: Processing file: {path:?}");
        }

        if let Ok(content) = str::from_utf8(line.content()) {
            diff_content.push_str(content);
        }
        true
    })?;

    println!("Debug: Total diff content length: {}", diff_content.len());
    Ok(diff_content)
}
