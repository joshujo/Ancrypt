pub mod vault;
pub mod commands;
pub mod set_up;
use tauri::async_runtime::Mutex;

use crate::commands::commands::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default() 
        .manage(Mutex::new(VaultCollection::default()))
        .manage(Mutex::new(OpenVault::default()))
        .setup(|_| {
            set_up::set_up();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            request_vaults,
            create_vault
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
