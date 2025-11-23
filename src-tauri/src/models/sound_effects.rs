use chrono::{DateTime, Utc, serde::ts_milliseconds_option};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a sound effect
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SoundEffect {
    /// Empty sound effect (just used as default)
    DoNothing,
    /// Play a file sound
    PlaySound {
        /// File name
        name: String,
        /// File last modified date
        #[serde(rename = "lastModified")]
        #[serde(with = "ts_milliseconds_option")]
        last_modified: Option<DateTime<Utc>>,
        /// File path
        path: Option<PathBuf>,
    },
    /// Increase global volume
    IncreaseVolume {
        /// How much volume percent to increase
        volume: u8,
    },
    /// Decrease global volume
    DecreaseVolume {
        /// How much volume percent to decrease
        volume: u8,
    },
    /// Clear all current sound effects played
    ClearAllEffects,
}
