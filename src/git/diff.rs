use anyhow::Result;
use git2::{DiffOptions, Repository};
use std::path::Path;
use std::str;

use crate::config::CommitConfig;

fn open_repo() -> Result<Repository> {
    Repository::open_from_env().map_err(|e| anyhow::anyhow!("Not in a git repository: {e}"))
}

pub fn get_staged_diff(commit_config: Option<&CommitConfig>) -> Result<String> {
    let repo = open_repo()?;

    let head = repo.head()?.peel_to_tree()?;
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let index_tree = repo.find_tree(oid)?;

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(3);

    let diff = repo.diff_tree_to_tree(Some(&head), Some(&index_tree), Some(&mut diff_opts))?;

    format_diff(diff, commit_config)
}

pub fn get_unstaged_diff(commit_config: Option<&CommitConfig>) -> Result<String> {
    let repo = open_repo()?;

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(3);
    diff_opts.include_untracked(false);

    let diff = repo.diff_index_to_workdir(None, Some(&mut diff_opts))?;

    format_diff(diff, commit_config)
}

fn format_diff(diff: git2::Diff, commit_config: Option<&CommitConfig>) -> Result<String> {
    let mut diff_content = String::new();
    let ignore_lock_files = commit_config.map(|c| c.ignore_lock_files).unwrap_or(true);
    let custom_patterns: &[String] = commit_config.map(|c| c.custom_ignore_patterns.as_slice()).unwrap_or(&[]);

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(path) = delta.new_file().path() {
            if ignore_lock_files && should_ignore_file(path) {
                return true;
            }

            if should_ignore_by_custom_patterns(path, custom_patterns) {
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

fn should_ignore_file(path: &Path) -> bool {
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

    if let Some(filename) = path.file_name()
        && let Some(filename_str) = filename.to_str()
    {
        return ignored_files.contains(&filename_str);
    }

    false
}

fn should_ignore_by_custom_patterns(path: &Path, patterns: &[String]) -> bool {
    for pattern in patterns {
        let Ok(glob) = glob::Pattern::new(pattern) else {
            continue;
        };
        if glob.matches_path(path) {
            return true;
        }
    }

    false
}

pub fn get_amend_diff(commit_config: Option<&CommitConfig>) -> Result<String> {
    let repo = open_repo()?;

    let head_commit = repo.head()?.peel_to_commit()?;
    let parent_tree = if head_commit.parent_count() > 0 {
        head_commit.parent(0)?.tree()?
    } else {
        let empty_tree_id = repo.treebuilder(None)?.write()?;
        repo.find_tree(empty_tree_id)?
    };

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(3);
    diff_opts.include_untracked(false);

    let diff = repo.diff_tree_to_workdir_with_index(Some(&parent_tree), Some(&mut diff_opts))?;

    format_diff(diff, commit_config)
}

pub fn get_last_commit_message() -> Result<String> {
    let repo = open_repo()?;

    let head_commit = repo.head()?.peel_to_commit()?;
    Ok(head_commit.message().unwrap_or("").to_string())
}

pub fn get_truncated_diff(diff: &str, limit: usize) -> String {
    if diff.len() <= limit {
        return diff.to_string();
    }

    let mut end = limit;
    while end > 0 && diff.as_bytes()[end - 1] != b'\n' {
        end -= 1;
    }

    format!("{}\n\n[... diff truncated: {}/{} characters ...]", &diff[..end], limit, diff.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_should_ignore_file_lock_files() {
        assert!(should_ignore_file(Path::new("Cargo.lock")));
        assert!(should_ignore_file(Path::new("package-lock.json")));
        assert!(should_ignore_file(Path::new("yarn.lock")));
        assert!(should_ignore_file(Path::new("pnpm-lock.yaml")));
        assert!(should_ignore_file(Path::new("go.sum")));
        assert!(should_ignore_file(Path::new("src/Cargo.lock")));
        assert!(!should_ignore_file(Path::new("src/main.rs")));
        assert!(!should_ignore_file(Path::new("src/unlock.rs")));
    }

    #[test]
    fn test_should_ignore_by_custom_patterns_glob() {
        let patterns = vec!["**/generated/**".to_string()];
        assert!(should_ignore_by_custom_patterns(Path::new("src/generated/mod.rs"), &patterns));
        assert!(!should_ignore_by_custom_patterns(Path::new("src/main.rs"), &patterns));
    }

    #[test]
    fn test_should_ignore_by_custom_patterns_extension() {
        let patterns = vec!["*.lock".to_string()];
        assert!(should_ignore_by_custom_patterns(Path::new("Cargo.lock"), &patterns));
        assert!(!should_ignore_by_custom_patterns(Path::new("src/unlock.rs"), &patterns));
    }

    #[test]
    fn test_should_ignore_by_custom_patterns_invalid_glob() {
        let patterns = vec!["[invalid".to_string()];
        assert!(!should_ignore_by_custom_patterns(Path::new("anything"), &patterns));
    }

    #[test]
    fn test_truncate_diff_within_limit() {
        let diff = "some diff content";
        assert_eq!(get_truncated_diff(diff, 100), diff);
    }

    #[test]
    fn test_truncate_diff_exceeds_limit() {
        let diff = "line1\nline2\nline3\nline4\nline5\n";
        let result = get_truncated_diff(diff, 12);
        assert!(result.contains("diff truncated"));
        assert!(result.starts_with("line1\nline2\n"));
    }

    #[test]
    fn test_truncate_diff_empty() {
        assert_eq!(get_truncated_diff("", 100), "");
    }
}
