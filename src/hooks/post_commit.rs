use std::process::Command;

pub fn run_post_commit_hook() {
    // Execute actions after a commit is made
    let output =
        Command::new("echo").arg("Post-commit hook executed.").output().expect("Failed to execute post-commit hook");

    if !output.status.success() {
        eprintln!("Error executing post-commit hook: {}", String::from_utf8_lossy(&output.stderr));
    }
}
