use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::models::{keycode::LocalKeycode, sound_effects::SoundEffect};

/// Represents a keybind effect (keycode(s) + sound effect)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeybindEffectItem {
    /// Associated key(s)
    pub keycode_list: HashSet<LocalKeycode>,
    /// Associated sound effect
    pub sound_effect: SoundEffect,
}
