use serde::{Deserialize, Serialize};

use crate::models::keybind_effect::KeybindEffectItem;

/// Represents a user profile
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Profile {
    /// Profile name
    name: String,
    /// Keybind(s) config
    keybind_config: Vec<KeybindEffectItem>,
}
