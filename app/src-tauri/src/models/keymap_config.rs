use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{keycode::LocalKeycode, sound_effects::SoundEffects};

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct KeymapConfig {
    keymap_effect_config: HashMap<Vec<LocalKeycode>, SoundEffects>
}