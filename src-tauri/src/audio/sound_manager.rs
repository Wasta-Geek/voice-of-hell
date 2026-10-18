use std::{
    hint::spin_loop,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

use anyhow::{Result, anyhow};
use ringbuf::{
    HeapCons, HeapProd, HeapRb,
    traits::{Consumer, Observer, Producer, Split},
};

use crate::{
    audio::{effect_manager::EffectManager, globals::BUFFER_SIZE},
    models::runtime_audio_effect::RuntimeAudioEffect,
};

/// Manages sound effects
pub struct SoundManager {
    /// Sends effects to be played to the internal thread
    effect_sender: Sender<Box<dyn RuntimeAudioEffect>>,
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
        let (effect_sender, effect_receiver) = mpsc::channel();

        let thread_should_run = Arc::new(AtomicBool::new(true));
        let thread_should_run_cloned = thread_should_run.clone();

        // Launch thread and poll for key pressed
        let thread_handle = thread::spawn(move || {
            audio_processing(
                thread_should_run_cloned,
                input_cons,
                output_prod,
                effect_receiver,
            )
        });

        Self {
            effect_sender: effect_sender,
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

    /// Queue an audio effect to be played by the internal thread
    pub fn play_effect(&self, effect: Box<dyn RuntimeAudioEffect>) -> Result<()> {
        self.effect_sender
            .send(effect)
            .map_err(|_| anyhow!("Audio processing thread is not running"))
    }

    /// Cleanup function that properly clean internal thread
    pub fn exit_thread(&mut self) {
        if let Some(thread_handle) = self.thread_handle.take() {
            self.thread_should_run.store(false, Ordering::Relaxed);
            let _ = thread_handle.join();
        }
    }
}

/// Audio processing loop (mic input + effects -> output). This method is run by SoundManager internal thread.
///
/// Mic samples are forwarded as soon as they're available. Without mic samples (no input device,
/// input stream stopped...), silent blocks are generated when the output is about to run out so that
/// effects still play.
fn audio_processing(
    thread_should_run_bool: Arc<AtomicBool>,
    mut input_heap_consumer: HeapCons<f32>,
    mut output_heap_producer: HeapProd<f32>,
    effect_receiver: Receiver<Box<dyn RuntimeAudioEffect>>,
) {
    let mut tmp_buffer = [0.0; BUFFER_SIZE as usize];
    let mut effect_manager = EffectManager::new();

    while thread_should_run_bool.load(Ordering::Relaxed) {
        // Queue newly requested effects (non-blocking)
        for effect in effect_receiver.try_iter() {
            effect_manager.add_audio_effect(effect);
        }

        // Read available input samples
        let input_size = input_heap_consumer.pop_slice(&mut tmp_buffer);

        let block_size = if input_size > 0 {
            input_size
        } else if output_heap_producer.occupied_len() < BUFFER_SIZE as usize {
            // No input while output is about to run out: generate a silent block
            tmp_buffer.fill(0.0);
            tmp_buffer.len()
        } else {
            // CPU instruction to put in "energy mode for a short time"
            // No idea if it's really impactful as CPU consumption seems to stay the same
            // TODO Measure in release mode OR replace by condvar
            spin_loop();
            continue;
        };
        let block = &mut tmp_buffer[..block_size];

        // Merge playing effects into the block, then keep it within the valid sample range
        effect_manager.consume_sample(block);
        block
            .iter_mut()
            .for_each(|sample| *sample = sample.clamp(-1.0, 1.0));

        // Only push the block if the output buffer has room for all of it, otherwise drop it entirely.
        // Stereo samples are stored alternately (left, right, left, right...): pushing only part of
        // the block could cut a left/right pair in half, and every following sample would then be
        // played on the wrong side.
        if output_heap_producer.vacant_len() >= block_size {
            output_heap_producer.push_slice(block);
        }
    }
}
