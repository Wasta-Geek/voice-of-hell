use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::{Result, anyhow};
use cpal::Sample;
use hound::{SampleFormat, WavReader};

use crate::file::sound_file_reader::SoundFileReader;

/// Support WAV file samples reading
pub struct WavFileReader {
    /// File content
    samples: Vec<f32>,
}

impl SoundFileReader for WavFileReader {
    fn new(path: PathBuf) -> Result<Self> {
        let mut reader = WavReader::open(path)?;
        let samples = read_wav_reader_samples_as_f32_vec(&mut reader)?;

        Ok(WavFileReader { samples: samples })
    }

    fn get_content(&self) -> &Vec<f32> {
        &self.samples
    }
}

/// Convert hound WAV Reader samples to a Vec<f32>
fn read_wav_reader_samples_as_f32_vec(
    wav_reader: &mut WavReader<BufReader<File>>,
) -> Result<Vec<f32>> {
    let spec = wav_reader.spec();

    match spec.sample_format {
        SampleFormat::Float => {
            // WAV stores f32 already
            wav_reader
                .samples::<f32>()
                .map(|result| match result {
                    Ok(sample) => Ok(sample),
                    Err(err) => Err(anyhow!(
                        "Cannot retrieves file content with float format, error: {}",
                        err
                    )),
                })
                .collect::<Result<Vec<f32>>>()
        }
        SampleFormat::Int => match spec.bits_per_sample {
            8 => wav_reader
                .samples::<i8>()
                .map(|result| match result {
                    Ok(sample) => Ok(Sample::to_sample::<f32>(sample)),
                    Err(err) => Err(anyhow!(
                        "Cannot retrieves file content with i8 format, error: {}",
                        err
                    )),
                })
                .collect(),

            16 => wav_reader
                .samples::<i16>()
                .map(|result| match result {
                    Ok(sample) => Ok(Sample::to_sample::<f32>(sample)),
                    Err(err) => Err(anyhow!(
                        "Cannot retrieves file content with i16 format, error: {}",
                        err
                    )),
                })
                .collect(),

            // Internet black magic
            24 => wav_reader
                .samples::<i32>() // hound uses i32 for packed 24-bit
                .map(|result| match result {
                    Ok(sample) => Ok(Sample::to_sample::<f32>(sample >> 8)),
                    Err(err) => Err(anyhow!(
                        "Cannot retrieves file content with 24 bits per sample format, error: {}",
                        err
                    )),
                })
                .collect(),

            32 => wav_reader
                .samples::<i32>()
                .map(|result| match result {
                    Ok(sample) => Ok(Sample::to_sample::<f32>(sample)),
                    Err(err) => Err(anyhow!(
                        "Cannot retrieves file content with i32 format, error: {}",
                        err
                    )),
                })
                .collect(),

            bits_per_sample => Err(anyhow!(
                "Format of {} bits per sample not supported",
                bits_per_sample
            )),
        },
    }
}
