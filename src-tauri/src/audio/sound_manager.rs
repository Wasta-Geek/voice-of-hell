use std::{
    collections::VecDeque,
    hint::spin_loop,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};

use ringbuf::{
    HeapCons, HeapProd, HeapRb,
    traits::{Consumer, Producer, Split},
};

use crate::audio::globals::BUFFER_SIZE;

/// Manages sound effects
pub struct SoundManager {
    /// Contains last input buffer content
    input_buffer_queue: Arc<Mutex<VecDeque<Vec<f32>>>>,
    /// Optional input heap producer (exists until input stream is created and passed to its callback)
    input_heap_prod: Option<HeapProd<f32>>,
    /// Optional output heap consumer (exists until output stream is created and passed to its callback)
    output_heap_cons: Option<HeapCons<f32>>,

    /// Internal thread handle
    thread_handle: Option<JoinHandle<()>>,
    /// Atomic boolean used by internal thread as conditional stop
    thread_should_run: Arc<AtomicBool>,
}

impl SoundManager {
    /// Returns a new SoundManager instance
    pub fn new() -> Self {
        let (input_prod, input_cons) = HeapRb::new(4096).split();
        let (output_prod, output_cons) = HeapRb::new(4096).split();

        let thread_should_run = Arc::new(AtomicBool::new(true));
        let thread_should_run_cloned = thread_should_run.clone();

        // Launch thread and poll for key pressed
        let thread_handle = thread::spawn(move || {
            audio_processing(thread_should_run_cloned, input_cons, output_prod)
        });

        Self {
            input_buffer_queue: Arc::new(Mutex::new(VecDeque::new())),
            input_heap_prod: Some(input_prod),
            output_heap_cons: Some(output_cons),
            thread_handle: Some(thread_handle),
            thread_should_run: thread_should_run,
        }
    }

    /// Acquire input heap producer instance
    pub fn take_input_buffer_producer(&mut self) -> HeapProd<f32> {
        self.input_heap_prod
            .take()
            .expect("Input buffer heap producer can only be fetched once")
    }

    /// Acquire output heap consumer instance
    pub fn take_output_buffer_consumer(&mut self) -> HeapCons<f32> {
        self.output_heap_cons
            .take()
            .expect("Output buffer heap consumer can only be fetched once")
    }

    /// Add audio sample to input buffer
    pub async fn feed_input_buffer(&mut self, buffer: Vec<f32>) {
        // Lock mutex for mutable
        let mut input_buffer_queue = self.input_buffer_queue.lock().unwrap();

        input_buffer_queue.push_back(buffer);
    }

    /// Cleanup function that properly clean internal thread
    pub fn exit_thread(&mut self) {
        if let Some(thread_handle) = self.thread_handle.take() {
            self.thread_should_run.store(false, Ordering::Relaxed);
            let _ = thread_handle.join();
        }
    }
}

/// Polling method that manages key pressed / released. This method is run by KeyboardManager internal thread.
fn audio_processing(
    thread_should_run_bool: Arc<AtomicBool>,
    mut input_heap_consumer: HeapCons<f32>,
    mut output_heap_producer: HeapProd<f32>,
) {
    let mut tmp_buffer = [0.0; BUFFER_SIZE as usize];

    while thread_should_run_bool.load(Ordering::Relaxed) {
        // Reset tmp_buffer value with silence
        tmp_buffer.fill(0.0);

        // Read available input samples
        let size = input_heap_consumer.pop_slice(&mut tmp_buffer);

        if size > 0 {
            // Push samples for output device
            output_heap_producer.push_slice(&tmp_buffer);
        } else {
            // CPU instruction to put in "energy mode for a short time"
            // No idea if it's really impactful as CPU consumption seems to stay the same
            // TODO Measure in release mode OR replace by condvar
            spin_loop();
        }
    }
}
