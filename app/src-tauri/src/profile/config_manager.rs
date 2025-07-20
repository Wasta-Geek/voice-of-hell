use std::{fs::{self, File}, io::Write};

use directories::{ProjectDirs};

use crate::models::global_config::GlobalConfig;

pub struct ConfigManager {
    global_config: GlobalConfig
}

impl ConfigManager {
    pub fn new() -> Self {
        // ProjectDirs instance
        let project_dir = Self::get_project_dir_instance();
        // Project config directory path
        let project_path_dir = project_dir.config_dir();
        // Variable used to init new Self instance
        let mut last_profile_used = None;
    
        // Try to read existing config file
        if let Ok(raw_file_content) = fs::read_to_string(project_path_dir) {
            // Extract file content as JSON content
            if let Ok(json_file_content) = serde_json::from_slice::<GlobalConfig>(raw_file_content.as_bytes()) {
                last_profile_used = json_file_content.last_profile_used;
            }
        };

        Self {
            global_config: GlobalConfig { last_profile_used: last_profile_used }
        }
    }

    pub fn save_global_config(&self) {
        // ProjectDirs instance
        let project_dir = Self::get_project_dir_instance();
        // Project config directory path
        let project_path_dir = project_dir.config_dir();

        // Check if config directory exists; if not -> creates
        if !project_path_dir.exists() {
            fs::create_dir_all(project_path_dir).expect(format!("Couldn't create config directory at {project_path_dir:?}.").as_str());
        }

        // Create file
        let mut file = File::create(project_path_dir.join(".config")).expect("Couldn't create global config file.");
        // File content to write
        let file_content = serde_json::to_string(&self.global_config).expect("Couldn't serialized global config.");
        
        // Save config in file
        file.write(file_content.as_bytes()).expect("Couldn't write global config.");

        log::warn!("Global config file saved at {project_path_dir:?}.");
    }

    fn get_project_dir_instance() -> ProjectDirs {
        ProjectDirs::from("",  "WastaGeek", env!("CARGO_PKG_NAME") ).expect("Couldn't open project config path.")
    }
}


impl Drop for ConfigManager {
    fn drop(&mut self) {
        // Save global config on exit
        self.save_global_config();
    }
}