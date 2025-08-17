pub mod commands;
pub mod set_up;
pub mod vault;
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
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(Mutex::new(VaultCollection::default()))
        .setup(|_| {
            set_up::set_up();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            request_vaults,
            create_vault,
            open_vault,
            retrieve_password_list,
            copy_to_clipboard,
            add_password,
            lock_vault,
            delete_password,
            request_delete_vault,
            five_number_rng,
            clear_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
