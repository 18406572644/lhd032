mod commands;
mod crypto;
mod storage;
mod hibp;

use commands::{AppVault, MasterKey, AppHibp, AppSecurity, SecurityManager};
use storage::VaultStorage;
use hibp::HibpManager;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .manage(AppVault(Mutex::new(VaultStorage::new())))
        .manage(MasterKey(Mutex::new(None)))
        .manage(AppHibp(HibpManager::new()))
        .manage(AppSecurity(Mutex::new(SecurityManager::new())))
        .invoke_handler(tauri::generate_handler![
            commands::setup_master_password,
            commands::verify_master_password,
            commands::is_vault_initialized,
            commands::get_entries,
            commands::add_entry,
            commands::update_entry,
            commands::delete_entry,
            commands::decrypt_password,
            commands::decrypt_history_password,
            commands::restore_history_password,
            commands::generate_password,
            commands::get_hibp_settings,
            commands::save_hibp_settings,
            commands::check_single_password,
            commands::batch_scan_passwords,
            commands::is_scan_running,
            commands::cancel_scan,
            commands::load_offline_hibp_db,
            commands::is_offline_db_loaded,
            commands::get_pwned_entries,
            commands::lock_vault,
            commands::get_security_settings,
            commands::save_security_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run()
}
