mod repository {
    use std::process::Command;

    pub struct GitRepository {
        path: String,
    }

    impl GitRepository {
        pub fn new(path: &str) -> Self {
            GitRepository {
                path: path.to_string(),
            }
        }

        pub fn get_commits(&self) -> Result<String, String> {
            let output = Command::new("git")
                .arg("log")
                .arg("--oneline")
                .current_dir(&self.path)
                .output()
                .map_err(|e| e.to_string())?;

            if output.status.success() {
                String::from_utf8(output.stdout).map_err(|e| e.to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }

        pub fn get_branches(&self) -> Result<String, String> {
            let output = Command::new("git")
                .arg("branch")
                .current_dir(&self.path)
                .output()
                .map_err(|e| e.to_string())?;

            if output.status.success() {
                String::from_utf8(output.stdout).map_err(|e| e.to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
    }
}