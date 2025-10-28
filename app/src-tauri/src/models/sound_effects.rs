use chrono::{DateTime, Utc, serde::ts_milliseconds_option};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SoundEffect {
    DoNothing,
    PlaySound {
        name: String,
        #[serde(rename = "lastModified")]
        #[serde(with = "ts_milliseconds_option")]
        last_modified: Option<DateTime<Utc>>,
        path: Option<PathBuf>,
    },
    IncreaseVolume {
        volume: u8,
    },
    DecreaseVolume {
        volume: u8,
    },
    ClearAllEffects,
}
