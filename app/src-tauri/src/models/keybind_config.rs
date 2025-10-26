use serde::{Deserialize, Serialize};

use crate::models::{keycode::LocalKeycode, sound_effects::SoundEffect};

#[derive(Clone, Serialize, Deserialize)]
pub struct KeybindEffectItem {
    keycode_list: Vec<LocalKeycode>,
    sound_effect: SoundEffect,
}
