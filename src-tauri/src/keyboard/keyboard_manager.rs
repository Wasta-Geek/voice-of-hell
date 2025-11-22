use arc_swap::ArcSwap;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{
    collections::HashSet,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};
use tauri::{AppHandle, Emitter, Manager};

use crate::{
    models::{keyboard_state::KeyboardState, keycode::VecInto},
    profile::config_manager::ConfigManager,
};

pub struct KeyboardManager {
    thread_handle: Option<JoinHandle<()>>,
    thread_should_run: Arc<AtomicBool>,
}

impl KeyboardManager {
    /// Create a Keyboard manager with the given config_manager
    pub fn new(app_handle: AppHandle) -> Self {
        let mut ret = KeyboardManager {
            thread_handle: None,
            thread_should_run: Arc::new(AtomicBool::new(true)),
        };

        let thread_shound_run_clone = ret.thread_should_run.clone();

        // Launch thread and poll for key pressed
        let join_handle = thread::spawn(move || poll(thread_shound_run_clone, app_handle));
        ret.thread_handle = Some(join_handle);

        ret
    }

    /// Cleanup function that properly clean internal thread
    pub fn exit_thread(&mut self) {
        if let Some(thread_join_handle) = self.thread_handle.take() {
            self.thread_should_run.store(false, Ordering::Relaxed);
            let _ = thread_join_handle.join();
        }
    }
}

/// Polling method that manages key pressed / released. This method is run by KeyboardManager internal thread.
fn poll(thread_should_run_bool: Arc<AtomicBool>, app_handle: AppHandle) {
    let device_state: DeviceState = DeviceState::new();
    let mut previous_key_pressed_list = HashSet::new();

    while thread_should_run_bool.load(Ordering::Relaxed) {
        let key_pressed_list: HashSet<Keycode> = device_state.get_keys().into_iter().collect();
        let key_released_list: HashSet<_> = previous_key_pressed_list
            .difference(&key_pressed_list)
            .into_iter()
            .collect();
        let new_key_pressed_list: HashSet<_> = key_pressed_list
            .difference(&previous_key_pressed_list)
            .into_iter()
            .collect();

        let config_manager_state = app_handle.state::<Arc<ArcSwap<ConfigManager>>>();
        let config_manager = config_manager_state.load();
        let config = config_manager.get_config();

        // // If at least one key released
        // if key_released_list.len() > 0 {
        //     log::warn!("Key released: {:?}", key_released_list);
        //     // if let Some(last_profile_index_used) = config.stored.last_profile_index_used {}
        // }

        if config.runtime.keybind_listening
            && (new_key_pressed_list.len() > 0 || key_released_list.len() > 0)
        {
            let emit_result = app_handle.emit(
                "keyboard_state_changed",
                KeyboardState {
                    key_pressed_list: Vec::from_iter(key_pressed_list.clone().into_iter())
                        .vec_into(),
                    key_released_list: Vec::from_iter(key_released_list.clone().into_iter())
                        .vec_into(),
                },
            );
            if let Err(error) = emit_result {
                log::error!("Couldn't send event for keyboard state changed")
            }
        }
        previous_key_pressed_list = key_pressed_list;
    }
}
