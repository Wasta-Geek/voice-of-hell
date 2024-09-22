// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Menu, MenuItem};

use crate::keyboard::keyboard_manager::KeyboardManager;

pub mod keyboard;

fn main() {
    let _keyboard_manager = KeyboardManager::new();
    let menu = Menu::new()
      .add_native_item(MenuItem::Quit);
    tauri::Builder::default()
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}