use serde::{Deserialize, Serialize};

/// Represents an audio device
#[derive(Clone, Serialize, Deserialize)]
pub struct Device {
    /// Audio device name
    pub name: String,
}
