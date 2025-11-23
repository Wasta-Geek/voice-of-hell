use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::models::app_config::{AppConfig, StoredConfig};

/// Manages app config
pub struct ConfigManager {
    /// App config
    app_config: AppConfig,
    /// Path to project config directory
    project_path_dir: PathBuf,
}

impl ConfigManager {
    /// Create a [ConfigManager] object with the given app_config
    pub fn new(app_config: AppConfig) -> Self {
        let project_dir = Self::get_project_dir_instance();
        let project_path_dir = project_dir.config_dir();

        Self {
            app_config: app_config,
            project_path_dir: project_path_dir.to_path_buf(),
        }
    }

    /// Get current config
    pub fn get_config(&self) -> AppConfig {
        self.app_config.clone()
    }

    /// Replace config with given config
    ///
    pub fn put_config(&mut self, config: AppConfig) {
        self.app_config = config;
    }

    /// Retrieves config from filesystem
    ///
    pub fn read_config_from_system(&mut self) {
        // Config file path
        let config_file_path = self.get_project_config_file_path();

        // Try to read existing config file
        match fs::read_to_string(config_file_path) {
            Ok(raw_file_content) =>
            // Extract file content as JSON content
            {
                match serde_json::from_slice::<StoredConfig>(raw_file_content.as_bytes()) {
                    Ok(json_file_content) => {
                        self.app_config.stored.last_profile_index_used =
                            json_file_content.last_profile_index_used;
                        self.app_config.stored.profiles = json_file_content.profiles;
                        log::info!(
                            "Config file properly read from: {:?}",
                            self.get_project_config_file_path()
                        );
                    }
                    Err(err) => {
                        log::error!(
                            "Error while parsing config file as JSON, path: {:?}, reason: {}",
                            self.get_project_config_file_path(),
                            err
                        )
                    }
                }
            }
            Err(err) => log::error!(
                "Couldn't read config file, path: {:?}, reason: {}",
                self.get_project_config_file_path(),
                err
            ),
        };
    }

    /// Save config to filesystem
    ///
    pub fn save_config(&self) {
        // Check if config directory exists; if not -> creates
        if !self.project_path_dir.exists() {
            fs::create_dir_all(self.project_path_dir.clone()).unwrap_or_else(|_| {
                panic!(
                    "Couldn't create config directory at {:?}.",
                    self.project_path_dir
                )
            });
        }
        // Config file path
        let config_file_path = self.get_project_config_file_path();

        // Create file
        let mut file = File::create(config_file_path).expect("Couldn't create config file.");
        // File content to write
        let file_content =
            serde_json::to_string(&self.app_config.stored).expect("Couldn't serialized config.");

        // Save config in file
        file.write_all(file_content.as_bytes())
            .expect("Couldn't write config.");

        log::info!("Config file saved at {:?}.", self.project_path_dir);
    }

    /// Returns the full path to the config file to used
    ///
    fn get_project_config_file_path(&self) -> PathBuf {
        self.project_path_dir.join("config.json")
    }

    /// Returns a ProjectDirs object
    ///
    fn get_project_dir_instance() -> ProjectDirs {
        ProjectDirs::from("", "WastaGeek", env!("CARGO_PKG_NAME"))
            .expect("Couldn't open project config path.")
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        let mut result = Self::new(AppConfig::default());
        result.read_config_from_system();
        result
    }
}
