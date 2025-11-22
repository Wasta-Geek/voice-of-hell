use arc_swap::ArcSwap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

use crate::{
    audio::device_manager::DeviceManager,
    emitter::emit_event,
    models::app_config::{AppConfig, RuntimeConfig, StoredConfig},
    profile::config_manager::ConfigManager,
};

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

#[tauri::command]
pub fn input_device_selected(
    state: State<'_, Mutex<DeviceManager>>,
    device_name: &str,
) -> Result<(), ()> {
    let mut device_manager = state.lock().unwrap();

    let _ = device_manager.set_input_device(device_name);
    Ok(())
}

#[tauri::command]
pub fn output_device_selected(
    state: State<'_, Mutex<DeviceManager>>,
    device_name: &str,
) -> Result<(), ()> {
    let mut device_manager = state.lock().unwrap();

    let _ = device_manager.set_output_device(device_name);
    Ok(())
}

#[tauri::command]
pub fn get_config(state: State<'_, Arc<ArcSwap<ConfigManager>>>) -> Result<StoredConfig, ()> {
    let config_manager = state.load();

    let config = config_manager.get_config();
    Ok(config.stored)
}

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
    state.store(new_config_manager);

    let config = config_manager.get_config();
    Ok(config.stored)
}

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
