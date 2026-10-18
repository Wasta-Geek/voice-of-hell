/// Represent an enabled audio effect (Send: created by caller thread, consumed by audio thread)
pub trait RuntimeAudioEffect: Send {
    /// Feed the effect with current input samples (effects that transform the voice)
    fn compute_next_sample(&mut self, current_sample_list: &[f32]);
    /// Merge the effect next samples into the given buffer
    fn consume_sample(&mut self, buffer: &mut [f32]);
    /// Returns true when the effect has nothing left to play (removed from playing effects)
    fn is_finished(&self) -> bool;
}
