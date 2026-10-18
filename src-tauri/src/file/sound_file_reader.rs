use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::file::wav_file_reader::WavFileReader;

/// Trait for sound file media support
pub trait SoundFileReader {
    /// Must create an instance, parse the files and returns an Ok(instance) in case of success
    fn new(path: PathBuf) -> Result<Self>
    where
        Self: Sized;
    /// Returns file content as a Vec<f32>
    fn get_content(&self) -> &Vec<f32>;
}

/// Returns the reader matching the file extension
pub fn create_sound_file_reader(path: PathBuf) -> Result<Box<dyn SoundFileReader>> {
    match path.extension() {
        Some(extension_osstr) => match extension_osstr.to_str() {
            Some(extension) => match extension.to_lowercase().as_str() {
                "wav" => Ok(Box::new(WavFileReader::new(path)?)),
                _ => Err(anyhow!(
                    "Media file extension is not supported: {}",
                    extension
                )),
            },
            None => Err(anyhow!(
                "Couldn't retrieve media file extension, path: {:?}",
                path
            )),
        },
        None => Err(anyhow!("Media file has no extension, path: {:?}", path)),
    }
}
