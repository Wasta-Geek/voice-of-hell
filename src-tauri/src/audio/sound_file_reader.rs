pub trait SoundFileReader {
    fn play_sound(soundfile_path: &'static str);
}
