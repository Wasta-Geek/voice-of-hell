use std::sync::{Arc, Mutex};
use arc_swap::ArcSwap;
use tauri::{AppHandle, State};

use crate::{
    audio::device_manager::DeviceManager, emitter::emit_event, models::app_config::AppConfig,
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
pub fn get_config(state: State<'_, Arc<ArcSwap<ConfigManager>>>) -> Result<AppConfig, ()> {
    let config_manager = state.load();

    let config = config_manager.get_config();
    Ok(config)
}

#[tauri::command]
pub fn save_config(state: State<'_, Arc<ArcSwap<ConfigManager>>>, config: AppConfig) -> Result<(), ()> {   
    let new_config_manager = Arc::new(ConfigManager::new(config));
    state.store(new_config_manager);

    Ok(())
}
