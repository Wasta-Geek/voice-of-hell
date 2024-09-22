// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod keyboard;
pub mod audio;
pub mod log;

use audio::device_manager::DeviceManager;
use tauri::{Menu, MenuItem};

use crate::keyboard::keyboard_manager::KeyboardManager;
use crate::log::init_logger;

fn main() {
    // Init logger
    init_logger ();
    // Init keyboard manager
    let _keyboard_manager = KeyboardManager::new();
    // Init audio devices manager
    let _device_manager = DeviceManager::new();

    let menu = Menu::new()
      .add_native_item(MenuItem::Quit);
    tauri::Builder::default()
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}