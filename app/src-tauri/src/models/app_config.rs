use serde::{Deserialize, Serialize};

use crate::models::profile::Profile;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub stored: StoredConfig,
    pub runtime: RuntimeConfig,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct StoredConfig {
    pub last_input_device_used: Option<String>,
    pub last_output_device_used: Option<String>,
    pub last_profile_index_used: Option<String>,
    // TODO "Separate global config from profiles could improve perf if lots of profiles/keys (useful ?)"
    pub profiles: Vec<Profile>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct RuntimeConfig {
    pub keybind_listening: bool,
}
