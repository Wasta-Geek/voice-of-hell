use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc, serde::ts_milliseconds_option};
use cpal::StreamConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    audio::effects::{do_nothing::AudioEffectDoNothing, play_sound::AudioEffectPlaySound},
    models::runtime_audio_effect::RuntimeAudioEffect,
};

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

impl SoundEffect {
    /// Transform "config effect" into a RuntimeAudioEffect that's ready to be consumed by the given stream
    pub fn make_runtime(
        &self,
        stream_config: &StreamConfig,
    ) -> Result<Box<dyn RuntimeAudioEffect>> {
        let runtime_instance: Box<dyn RuntimeAudioEffect> = match &self {
            SoundEffect::DoNothing => Box::new(AudioEffectDoNothing {}),
            SoundEffect::PlaySound {
                name,
                last_modified: _,
                path,
            } => {
                let unwrapped_path = path
                    .clone()
                    .ok_or_else(|| anyhow!("No file selected for sound effect {name}"))?;
                Box::new(AudioEffectPlaySound::new(unwrapped_path, stream_config)?)
            }
            // TODO Not implemented yet: returns an error instead of panicking the caller thread
            SoundEffect::IncreaseVolume { .. }
            | SoundEffect::DecreaseVolume { .. }
            | SoundEffect::ClearAllEffects => {
                return Err(anyhow!("Sound effect not implemented yet: {:?}", self));
            }
        };
        Ok(runtime_instance)
    }
}
