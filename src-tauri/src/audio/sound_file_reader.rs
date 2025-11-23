/// Trait responsible for playing a sound
pub trait SoundFileReader {
    /// Play sound from given sound file path
    fn play_sound(soundfile_path: &'static str);
}
