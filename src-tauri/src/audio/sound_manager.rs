use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct SoundManager {
    // Contains last input buffer content
    input_buffer_queue: Arc<Mutex<VecDeque<Vec<f32>>>>,
}

impl SoundManager {
    pub fn new() -> Self {
        Self {
            input_buffer_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn feed_input_buffer(&mut self, buffer: Vec<f32>) {
        // Lock mutex for mutable
        let mut input_buffer_queue = self.input_buffer_queue.lock().unwrap();

        input_buffer_queue.push_back(buffer);
    }
}
