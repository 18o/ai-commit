mod settings {
    use std::fs;
    use std::path::Path;

    #[derive(Debug, Default)]
    pub struct AppConfig {
        pub hook_enabled: bool,
        pub ai_service_url: String,
        pub commit_message_template: String,
    }

    // impl AppConfig {
    //     pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
    //         let config_content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    //         toml::de::from_str(&config_content).map_err(|e| e.to_string())
    //     }

    //     pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
    //         let config_content = toml::ser::to_string(self).map_err(|e| e.to_string())?;
    //         fs::write(path, config_content).map_err(|e| e.to_string())
    //     }
    // }
}
