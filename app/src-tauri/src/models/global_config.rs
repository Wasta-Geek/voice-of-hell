use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub last_profile_used: Option<String>
}