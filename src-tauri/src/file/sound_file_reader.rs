use std::path::PathBuf;

use anyhow::Result;

/// Trait for sound file media support
pub trait SoundFileReader {
    /// Must create an instance, parse the files and returns an Ok(instance) in case of success
    fn new(path: PathBuf) -> Result<Self>
    where
        Self: Sized;
    /// Returns file content as a Vec<f32>
    fn get_content(&self) -> &Vec<f32>;
}
