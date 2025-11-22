use serde::{Deserialize, Serialize};

use crate::models::keybind_effect::KeybindEffectItem;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Profile {
    name: String,
    keybind_config: Vec<KeybindEffectItem>,
}
