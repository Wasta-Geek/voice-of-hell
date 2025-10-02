use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::profile::Profile;

#[derive(Clone, Default, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct AppConfig {
    pub last_input_device_used: Option<String>,
    pub last_output_device_used: Option<String>,
    pub last_profile_used: Option<String>,
   // TODO "Separate global config from profiles could improve perf if lots of profiles/keys (useful ?)"
    pub profiles: Vec<Profile>
}