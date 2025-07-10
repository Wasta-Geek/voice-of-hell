use std::sync::Mutex;

use tauri::{AppHandle, State};

use crate::{emitter::emit_event, models::shared_state::SharedState};

#[tauri::command]
pub fn app_ready(app: AppHandle, state: State<'_, Mutex<SharedState>>) -> Result<(), ()> {
    // Lock mutex for mutable state
    let mut state = state.lock().unwrap();
  
    state.device_manager.refresh_devices();

    emit_event(&app, "input_devices_refreshed", &state.device_manager.input_devices);
    emit_event(&app, "output_devices_refreshed", &state.device_manager.output_devices);
    Ok(())
}


#[tauri::command]
pub fn input_device_selected(state: State<'_, Mutex<SharedState>>, device_name: &str) -> Result<(), ()> {
    // Lock mutex for mutable state
    let mut state = state.lock().unwrap();

    let _ = state.device_manager.set_input_device(device_name);
    Ok(())
}

#[tauri::command]
pub fn output_device_selected(state: State<'_, Mutex<SharedState>>, device_name: &str) -> Result<(), ()> {
    // Lock mutex for mutable state
    let mut state = state.lock().unwrap();

    let _ = state.device_manager.set_output_device(device_name);
    Ok(())
}