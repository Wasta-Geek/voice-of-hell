use arc_swap::ArcSwap;
use device_query::{DeviceQuery, DeviceState};
use std::{
    collections::HashSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};
use tauri::{AppHandle, Emitter, Manager};

use crate::{
    audio::device_manager::DeviceManager,
    models::{
        keyboard_state::KeyboardState,
        keycode::{LocalKeycode, VecInto},
        sound_effects::SoundEffect,
    },
    profile::config_manager::ConfigManager,
};

/// Manage keyboard input / state
pub struct KeyboardManager {
    /// Internal thread handle
    thread_handle: Option<JoinHandle<()>>,
    /// Atomic boolean used by internal thread as conditional stop
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
    pub fn clear_before_exit(&mut self) {
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
        let key_pressed_list: HashSet<LocalKeycode> =
            device_state.get_keys().vec_into().into_iter().collect();
        let key_released_list: HashSet<_> = previous_key_pressed_list
            .difference(&key_pressed_list)
            .collect();
        let new_key_pressed_list: HashSet<_> = key_pressed_list
            .difference(&previous_key_pressed_list)
            .collect();

        let config_manager_state = app_handle.state::<Arc<ArcSwap<ConfigManager>>>();
        let config_manager = config_manager_state.load();
        let config = config_manager.get_config();

        // If at least one key newly pressed (and user is not registering a keybind)
        if !new_key_pressed_list.is_empty()
            && !config.runtime.keybind_listening
            && config.stored.last_profile_index_used.is_some()
        {
            log::debug!("Key pressed: {:?}", new_key_pressed_list);
            let profile_index_used_str = config.stored.last_profile_index_used;
            let profile_index_res = profile_index_used_str.unwrap().parse::<usize>();
            if let Ok(profile_index) = profile_index_res {
                let current_config = &config.stored.profiles[profile_index];
                // Loop over each keybind
                for keybind in &current_config.keybind_config {
                    // Triggered if all its keys are pressed, at least one of them newly
                    // (holding the keybind + pressing another key doesn't trigger it again)
                    if keybind.keycode_list.is_subset(&key_pressed_list)
                        && keybind
                            .keycode_list
                            .iter()
                            .any(|keycode| new_key_pressed_list.contains(keycode))
                    {
                        trigger_sound_effect(&app_handle, &keybind.sound_effect);
                    }
                }
            }
        }

        if config.runtime.keybind_listening
            && (!new_key_pressed_list.is_empty() || !key_released_list.is_empty())
        {
            let emit_result = app_handle.emit(
                "keyboard_state_changed",
                KeyboardState {
                    key_pressed_list: Vec::from_iter(key_pressed_list.clone()),
                    key_released_list: Vec::from_iter(
                        key_released_list.clone().into_iter().cloned(),
                    ),
                },
            );
            if let Err(error) = emit_result {
                log::error!(
                    "Couldn't send event for keyboard state changed, error: {}",
                    error
                )
            }
        }
        previous_key_pressed_list = key_pressed_list;
    }
}

/// Send the given sound effect to be played on current audio streams
fn trigger_sound_effect(app_handle: &AppHandle, sound_effect: &SoundEffect) {
    // TODO Preload sound files on profile load instead of reading them on each key press
    let play_result = sound_effect.make_runtime().and_then(|effect| {
        let device_manager_state = app_handle.state::<Mutex<DeviceManager>>();
        let device_manager = device_manager_state.lock().unwrap();
        device_manager.play_effect(effect)
    });

    if let Err(error) = play_result {
        log::error!(
            "Couldn't play sound effect {:?}, error: {:#}",
            sound_effect,
            error
        );
    }
}
