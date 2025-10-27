use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SoundEffect {
    DoNothing,
    PlaySound{file: String},
    IncreaseVolume{volume: u8},
    DecreaseVolume{volume: u8},
    ClearAllEffects,
}