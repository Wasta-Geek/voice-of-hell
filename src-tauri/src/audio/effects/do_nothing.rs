use crate::models::runtime_audio_effect::RuntimeAudioEffect;

/// Effect :Empty (used as empty placeholder)
pub struct AudioEffectDoNothing {}

impl RuntimeAudioEffect for AudioEffectDoNothing {
    fn compute_next_sample(&mut self, _current_sample_list: &[f32]) {}

    fn consume_sample(&mut self, _buffer: &mut [f32]) {}

    fn is_finished(&self) -> bool {
        true
    }
}
