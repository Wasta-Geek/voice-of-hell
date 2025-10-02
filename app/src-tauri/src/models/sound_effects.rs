use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub enum SoundEffects {
    PlaySound(String),
    IncreaseVolume(u8),
    DecreaseVolume(u8),
    ClearAllEffects,
}