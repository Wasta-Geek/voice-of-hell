use serde::{Deserialize, Serialize};

use crate::models::profile::Profile;

/// App config
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    /// Stored config
    pub stored: StoredConfig,
    /// Runtime config
    pub runtime: RuntimeConfig,
}

/// Stored config (filesystem)
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct StoredConfig {
    /// Last input audio device used
    pub last_input_device_used: Option<String>,
    /// Last output audio device used
    pub last_output_device_used: Option<String>,
    /// Last profile index used
    pub last_profile_index_used: Option<String>,
    /// User profiles
    // TODO "Separate global config from profiles could improve perf if lots of profiles/keys (useful ?)"
    pub profiles: Vec<Profile>,
}

/// Runetime config (dynamic)
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct RuntimeConfig {
    /// Is user registering keybind now (does not trigger any sound effect)
    pub keybind_listening: bool,
}
