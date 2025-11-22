use cpal::{
    Device, Stream, StreamConfig,
    traits::{DeviceTrait, StreamTrait},
};
use ringbuf::{HeapRb, traits::Split};

use crate::{
    audio::{
        sound_manager::SoundManager,
        stream_callback::{input_callback, output_callback, stream_error_callback},
    },
    error::MyError,
};

pub struct StreamManager {
    /// Input stream, None if stream not currently opened
    input_stream: Option<Stream>,
    /// Output stream, None if stream not currently opened
    output_stream: Option<Stream>,
    /// Manages sound (effects, volume, ...)
    sound_manager: SoundManager,
}

impl StreamManager {
    /// Generates new StreamManager instance
    pub fn new() -> Self {
        Self {
            input_stream: Option::None,
            output_stream: Option::None,
            sound_manager: SoundManager::new(),
        }
    }

    /// Start streams on input / output devices given
    pub fn start_streams(
        &mut self,
        input_device: &Device,
        output_device: &Device,
        config: &StreamConfig,
    ) -> Result<(), MyError> {
        // Clear previous streams (if any already opened)
        self.clear_streams();

        // Producer / Consumer
        let (mut producer, mut consumer) = HeapRb::new(96000).split();

        // Create input stream
        match input_device.build_input_stream(
            &config,
            move |data: &[f32], callback_info| input_callback(data, callback_info, &mut producer),
            stream_error_callback,
            None,
        ) {
            Ok(stream) => {
                stream.play().map_err(|_| MyError {})?;
                self.input_stream = Some(stream)
            }
            Err(error) => log::error!("Couldn't open input stream, reason: {}", error),
        }

        // Create output stream
        match output_device.build_output_stream(
            &config,
            move |data: &mut [f32], callback_info| {
                output_callback(data, callback_info, &mut consumer)
            },
            stream_error_callback,
            None,
        ) {
            Ok(stream) => {
                stream.play().map_err(|_| MyError {})?;
                self.output_stream = Some(stream)
            }
            Err(error) => log::error!("Couldn't open output stream, reason: {}", error),
        }
        Ok(())
    }

    /// Properly clear input / output streams (if opened)
    pub fn clear_streams(&mut self) {
        if let Some(_) = self.input_stream.take() {
            log::info!("Previous input stream cleared.");
        }
        if let Some(_) = self.output_stream.take() {
            log::info!("Previous output stream cleared.");
        }
    }
}
