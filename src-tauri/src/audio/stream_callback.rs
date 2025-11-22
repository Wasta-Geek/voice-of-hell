use cpal::{InputCallbackInfo, OutputCallbackInfo, StreamError};
use ringbuf::{
    traits::{Consumer, Producer},
    HeapCons, HeapProd,
};

pub fn stream_error_callback(error: StreamError) {
    let error_string = match error {
        StreamError::DeviceNotAvailable => "Device select seems to be disconnected !".to_owned(),
        StreamError::BackendSpecific { err } => {
            "An unknown error happened, reason: {}".to_owned() + &err.description
        }
    };
    log::error!("{}", error_string);
}

pub fn input_callback(
    buffer: &[f32],
    _callback_info: &InputCallbackInfo,
    channel_sender: &mut HeapProd<f32>,
) {
    channel_sender.push_slice(buffer);
}

pub fn output_callback(
    output_buffer: &mut [f32],
    _callback_info: &OutputCallbackInfo,
    channel_receiver: &mut HeapCons<f32>,
) {
    // Feed output buffer from last input buffer
    let count_sample = channel_receiver.pop_slice(output_buffer);
    // Missing sample
    let missing_sample_size = output_buffer.len() - count_sample;

    // Loop over empty output buffer part
    for i in count_sample..count_sample + missing_sample_size {
        // Add silence to missing frames in output buffer
        output_buffer[i] = 0.0;
    }
}
