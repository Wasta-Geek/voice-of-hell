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

use std::sync::{Arc, Mutex};

use arc_swap::ArcSwap;
use audio::device_manager::DeviceManager;
use tauri::Manager;

use crate::log::init_logger;
use crate::{keyboard::keyboard_manager::KeyboardManager, profile::config_manager::ConfigManager};

fn main() {
    // Init logger
    init_logger();
    // Init needed managers
    let device_manager = DeviceManager::new();
    let config_manager = ConfigManager::default();
    let config_manager_arc = Arc::new(ArcSwap::from_pointee(config_manager));

    // Create tauri app
    tauri::Builder::default()
        .setup(move |app| {
            let keyboard_manager = KeyboardManager::new(app.handle().clone());

            app.manage(Mutex::new(device_manager));
            app.manage(config_manager_arc);
            app.manage(Mutex::new(keyboard_manager));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_ready,
            commands::input_device_selected,
            commands::output_device_selected,
            commands::get_config,
            commands::save_config,
            commands::set_keybind_listening_state
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            match event {
                tauri::RunEvent::ExitRequested { .. } => {
                    let keyboard_manager_mutex = app_handle.state::<Mutex<KeyboardManager>>();
                    let mut keyboard_manager = keyboard_manager_mutex.lock().unwrap();
                    keyboard_manager.exit_thread();

                    let config_manager_arc = app_handle.state::<Arc<ArcSwap<ConfigManager>>>();
                    let config_manager = config_manager_arc.load_full();

                    // Save config to Config file on app exit
                    config_manager.save_config();
                }
                _ => (),
            }
        });
}
