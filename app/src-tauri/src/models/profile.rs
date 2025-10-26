use serde::{Deserialize, Serialize};

use crate::models::keybind_config::KeybindEffectItem;

#[derive(Clone, Serialize, Deserialize)]
pub struct Profile {
    name: String,
    keybind_config: Vec<KeybindEffectItem>,
}
