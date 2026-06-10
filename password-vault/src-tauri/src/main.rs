mod commands;
mod crypto;
mod storage;

use commands::{AppVault, MasterKey};
use storage::VaultStorage;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(AppVault(Mutex::new(VaultStorage::new())))
        .manage(MasterKey(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            commands::setup_master_password,
            commands::verify_master_password,
            commands::is_vault_initialized,
            commands::get_entries,
            commands::add_entry,
            commands::update_entry,
            commands::delete_entry,
            commands::decrypt_password,
            commands::generate_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run()
}
