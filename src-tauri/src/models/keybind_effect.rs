use serde::{Deserialize, Serialize};

use crate::models::{keycode::LocalKeycode, sound_effects::SoundEffect};

/// Represents a keybind effect (keycode(s) + sound effect)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeybindEffectItem {
    /// Associated key(s)
    keycode_list: Vec<LocalKeycode>,
    /// Associated sound effect
    sound_effect: SoundEffect,
}
