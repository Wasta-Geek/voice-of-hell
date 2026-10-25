use anyhow::Result;
use cpal::StreamConfig;
use std::path::PathBuf;

use crate::{
    audio::conversion::{convert_channels, resample},
    file::sound_file_reader::create_sound_file_reader,
    models::runtime_audio_effect::RuntimeAudioEffect,
};

/// Effect: Play a sound file
pub struct AudioEffectPlaySound {
    /// List of audio sample remaining (that still needs to be played)
    sample_list: Vec<f32>,
    /// Index of the next sample to play
    position: usize,
}

impl AudioEffectPlaySound {
    /// Returns a new AudioEffectPlaySound instance, file samples converted to the stream format
    pub fn new(path: PathBuf, stream_config: &StreamConfig) -> Result<Self> {
        let sound_file_reader = create_sound_file_reader(path)?;
        let stream_channels = stream_config.channels as usize;

        let sample_list = convert_channels(
            sound_file_reader.get_content().to_vec(),
            sound_file_reader.get_channel_count() as usize,
            stream_channels,
        );
        let sample_list = resample(
            sample_list,
            stream_channels,
            sound_file_reader.get_sample_rate(),
            stream_config.sample_rate.0,
        )?;

        Ok(AudioEffectPlaySound {
            sample_list: sample_list,
            position: 0,
        })
    }
}

impl RuntimeAudioEffect for AudioEffectPlaySound {
    // Not needed for this effect
    fn compute_next_sample(&mut self, _current_sample_list: &[f32]) {}

    fn consume_sample(&mut self, buffer: &mut [f32]) {
        // Merge remaining samples into target buffer (could be less than buffer size at the end)
        let remaining_sample_list = &self.sample_list[self.position..];
        let merged_size = remaining_sample_list.len().min(buffer.len());

        buffer
            .iter_mut()
            .zip(remaining_sample_list)
            .for_each(|(buffer_sample, sample)| *buffer_sample += sample);

        self.position += merged_size;
    }

    fn is_finished(&self) -> bool {
        self.position >= self.sample_list.len()
    }
}
