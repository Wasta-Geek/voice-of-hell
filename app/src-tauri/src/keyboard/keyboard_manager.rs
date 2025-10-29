use arc_swap::ArcSwap;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{
    collections::HashSet,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread::{self, JoinHandle},
};

use crate::profile::config_manager::ConfigManager;

pub struct KeyboardManager {
    config_manager_arc: Arc<ArcSwap<ConfigManager>>,
    thread_handle: Option<JoinHandle<()>>,
    thread_should_run: Arc<AtomicBool>
}

impl KeyboardManager {
    pub fn new(config_manager_arc: Arc<ArcSwap<ConfigManager>>) -> Self {
        let mut ret = KeyboardManager {
            config_manager_arc: config_manager_arc,
            thread_handle: None,
            thread_should_run: Arc::new(AtomicBool::new(true))
        };

        let thread_shound_run_clone = ret.thread_should_run.clone();
        let config_manager_clone = ret.config_manager_arc.clone();
        // Launch thread and poll for key pressed
        let join_handle = thread::spawn(move || poll(thread_shound_run_clone, config_manager_clone));
        ret.thread_handle = Some(join_handle);

        ret
    }

    pub fn exit_thread(&mut self) {
        if let Some(thread_join_handle) = self.thread_handle.take() {
            self.thread_should_run.store(false, Ordering::Relaxed);
            let _ = thread_join_handle.join();
        }
    }
}

fn poll(thread_should_run_bool: Arc<AtomicBool>, config_manager_arc: Arc<ArcSwap<ConfigManager>>) {
    let device_state: DeviceState = DeviceState::new();
    let mut previous_key_pressed_list = HashSet::new();

    while thread_should_run_bool.load(Ordering::Relaxed) {
        let key_pressed_list: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

        if !key_pressed_list.is_empty() {
            let config_manager = config_manager_arc.load();
            let config = config_manager.get_config();

            if let Some(last_profile_index_used) = config.last_profile_index_used {}
        }
        previous_key_pressed_list = key_pressed_list;
    }
}
