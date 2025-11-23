use cpal::{InputCallbackInfo, OutputCallbackInfo, StreamError};
use ringbuf::{
    HeapCons, HeapProd,
    traits::{Consumer, Producer},
};

/// Method called in case of stream error
pub fn stream_error_callback(error: StreamError) {
    let error_string = match error {
        StreamError::DeviceNotAvailable => "Device select seems to be disconnected !".to_owned(),
        StreamError::BackendSpecific { err } => {
            "An unknown error happened, reason: {}".to_owned() + &err.description
        }
    };
    log::error!("{}", error_string);
}

/// Callback called for each audio input device frame
pub fn input_callback(
    buffer: &[f32],
    _callback_info: &InputCallbackInfo,
    channel_sender: &mut HeapProd<f32>,
) {
    channel_sender.push_slice(buffer);
}

/// Callback called for each audio output device frame
pub fn output_callback(
    output_buffer: &mut [f32],
    _callback_info: &OutputCallbackInfo,
    channel_receiver: &mut HeapCons<f32>,
) {
    // Feed output buffer from last input buffer
    let count_sample = channel_receiver.pop_slice(output_buffer);
    // Missing sample
    let missing_sample_size = output_buffer.len() - count_sample;

    // Loop over remaining buffer values that should be set at 0.0
    for buffer_value in output_buffer
        .iter_mut()
        .skip(count_sample)
        .take(missing_sample_size)
    {
        // Add silence to missing frames in output buffer
        *buffer_value = 0.0;
    }
}
