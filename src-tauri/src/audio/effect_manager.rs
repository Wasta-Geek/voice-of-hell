use crate::models::runtime_audio_effect::RuntimeAudioEffect;

/// Manage audio effects to be played
pub struct EffectManager {
    /// Effects currently playing
    effect_list: Vec<Box<dyn RuntimeAudioEffect>>,
}

impl EffectManager {
    /// Returns a new EffectManager instance
    pub fn new() -> Self {
        EffectManager {
            effect_list: Default::default(),
        }
    }

    /// Queue a new audio effect to be played (can be a permanent effect)
    pub fn add_audio_effect(&mut self, effect: Box<dyn RuntimeAudioEffect>) {
        self.effect_list.push(effect);
    }

    /// Merge every queued effect into the given buffer, then drop finished effects
    pub fn consume_sample(&mut self, buffer: &mut [f32]) {
        for effect in self.effect_list.iter_mut() {
            effect.consume_sample(buffer);
        }

        self.effect_list.retain(|effect| !effect.is_finished());
    }
}
