use anyhow::Result;
use std::path::PathBuf;

use crate::{
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
    /// Returns a new AudioEffectPlaySound instance
    pub fn new(path: PathBuf) -> Result<Self> {
        let sound_file_reader = create_sound_file_reader(path)?;

        Ok(AudioEffectPlaySound {
            sample_list: sound_file_reader.get_content().to_vec(),
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
