use serde::{Deserialize, Serialize};

use crate::models::profile::Profile;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub last_input_device_used: Option<String>,
    pub last_output_device_used: Option<String>,
    pub last_profile_index_used: Option<String>,
   // TODO "Separate global config from profiles could improve perf if lots of profiles/keys (useful ?)"
    pub profiles: Vec<Profile>
}