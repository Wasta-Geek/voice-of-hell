use arc_swap::ArcSwap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

use crate::{
    audio::device_manager::DeviceManager,
    emitter::emit_event,
    models::app_config::{AppConfig, StoredConfig},
    profile::config_manager::ConfigManager,
};

/// [Tauri command] called by front when app has loaded (synchronization method)
#[tauri::command]
pub fn app_ready(app: AppHandle, state: State<'_, Mutex<DeviceManager>>) -> Result<(), ()> {
    // Lock mutex for mutable state
    let mut device_manager = state.lock().unwrap();

    device_manager.refresh_devices();

    emit_event(
        &app,
        "input_devices_refreshed",
        &device_manager.input_devices,
    );
    emit_event(
        &app,
        "output_devices_refreshed",
        &device_manager.output_devices,
    );
    Ok(())
}

/// [Tauri command] Use a given audio input device, returns the config with the device stored as last used
#[tauri::command]
pub fn input_device_selected(
    device_state: State<'_, Mutex<DeviceManager>>,
    config_state: State<'_, Arc<ArcSwap<ConfigManager>>>,
    device_name: &str,
) -> Result<StoredConfig, ()> {
    let mut device_manager = device_state.lock().unwrap();

    match device_manager.set_input_device(device_name) {
        Ok(()) => Ok(update_stored_config(&config_state, |stored| {
            stored.last_input_device_used = Some(device_name.to_owned())
        })),
        Err(error) => {
            log::error!("Couldn't use input device {device_name}, error: {error:#}");
            Ok(config_state.load().get_config().stored)
        }
    }
}

/// [Tauri command] Use a given audio output device, returns the config with the device stored as last used
#[tauri::command]
pub fn output_device_selected(
    device_state: State<'_, Mutex<DeviceManager>>,
    config_state: State<'_, Arc<ArcSwap<ConfigManager>>>,
    device_name: &str,
) -> Result<StoredConfig, ()> {
    let mut device_manager = device_state.lock().unwrap();

    match device_manager.set_output_device(device_name) {
        Ok(()) => Ok(update_stored_config(&config_state, |stored| {
            stored.last_output_device_used = Some(device_name.to_owned())
        })),
        Err(error) => {
            log::error!("Couldn't use output device {device_name}, error: {error:#}");
            Ok(config_state.load().get_config().stored)
        }
    }
}

/// Apply the given change to the stored config, returns the updated stored config
fn update_stored_config(
    state: &State<'_, Arc<ArcSwap<ConfigManager>>>,
    update: impl Fn(&mut StoredConfig),
) -> StoredConfig {
    // rcu: retried if the config is replaced meanwhile (concurrent save_config), no change lost
    state.rcu(|config_manager| {
        let mut config = config_manager.get_config();
        update(&mut config.stored);
        ConfigManager::new(config)
    });

    state.load().get_config().stored
}

/// [Tauri command] Retrieves app config
#[tauri::command]
pub fn get_config(state: State<'_, Arc<ArcSwap<ConfigManager>>>) -> Result<StoredConfig, ()> {
    let config_manager = state.load();

    let config = config_manager.get_config();
    Ok(config.stored)
}

/// [Tauri command] Save app config
#[tauri::command]
pub fn save_config(
    state: State<'_, Arc<ArcSwap<ConfigManager>>>,
    config: StoredConfig,
) -> Result<StoredConfig, ()> {
    let config_manager = state.load();
    let previous_config = config_manager.get_config();

    let new_config_manager = Arc::new(ConfigManager::new(AppConfig {
        stored: config,
        runtime: previous_config.runtime,
    }));
    // Return the config just saved (config_manager still holds the previous one)
    let config = new_config_manager.get_config();
    state.store(new_config_manager);

    Ok(config.stored)
}

/// [Tauri command] (Un)Set if user if registering a Keybind
#[tauri::command]
pub fn set_keybind_listening_state(
    state: State<'_, Arc<ArcSwap<ConfigManager>>>,
    is_listening: bool,
) -> Result<(), ()> {
    let config_manager = state.load();
    let mut new_config = config_manager.get_config();
    new_config.runtime.keybind_listening = is_listening;

    let new_config_manager = Arc::new(ConfigManager::new(new_config));
    state.store(new_config_manager);
    Ok(())
}
