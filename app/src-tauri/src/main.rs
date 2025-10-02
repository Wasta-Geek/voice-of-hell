//! # Executable crate for voice-of-hell project

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub(crate) mod audio;
pub(crate) mod commands;
pub(crate) mod emitter;
pub(crate) mod error;
pub(crate) mod keyboard;
pub(crate) mod log;
pub(crate) mod models;
pub(crate) mod profile;

use std::sync::Mutex;

use audio::device_manager::DeviceManager;
use tauri::Manager;

use crate::log::init_logger;
use crate::models::shared_state::SharedState;
use crate::{keyboard::keyboard_manager::KeyboardManager, profile::config_manager::ConfigManager};

fn main() {
    // Init logger
    init_logger();
    // Init keyboard manager
    let _keyboard_manager = KeyboardManager::new();
    // Init audio devices manager
    let device_manager = DeviceManager::new();
    // Config file(s) manager
    let config_manager = ConfigManager::new();

    // Create tauri app
    tauri::Builder::default()
        .setup(move |app| {
            app.manage(Mutex::new(SharedState {
                device_manager: device_manager,
                config_manager: config_manager,
            }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_ready,
            commands::input_device_selected,
            commands::output_device_selected,
            commands::get_config,
            commands::save_config
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            match event {
                tauri::RunEvent::ExitRequested { .. } => {
                    let state_mutex = app_handle.state::<Mutex<SharedState>>();
                    let state = state_mutex.lock().unwrap();

                    // Save config to Config file on app exit
                    state.config_manager.save_config();
                },
                _ => (),
            }
        });
}
