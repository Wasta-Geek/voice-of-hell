use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum SoundEffect {
    DoNothing,
    PlaySound(String),
    IncreaseVolume(u8),
    DecreaseVolume(u8),
    ClearAllEffects,
}