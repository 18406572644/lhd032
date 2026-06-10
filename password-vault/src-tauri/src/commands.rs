use crate::crypto;
use crate::storage::{VaultEntry, VaultStorage};
use std::sync::Mutex;
use tauri::State;

pub struct AppVault(pub Mutex<VaultStorage>);
pub struct MasterKey(pub Mutex<Option<[u8; 32]>>);

#[tauri::command]
pub fn setup_master_password(password: String, vault: State<AppVault>) -> Result<String, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let salt = crypto::generate_salt();
    let hash = crypto::hash_password(&password, &salt);
    let salt_hex = hex::encode(&salt);
    storage.set_master_password(&salt_hex, &hash);
    Ok("ok".to_string())
}

#[tauri::command]
pub fn verify_master_password(password: String, vault: State<AppVault>, key_state: State<MasterKey>) -> Result<bool, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    if !storage.is_initialized() {
        return Err("vault not initialized".to_string());
    }
    let salt = hex::decode(&storage.get_salt()).map_err(|e| e.to_string())?;
    let stored_hash = storage.get_master_hash();
    let valid = crypto::verify_password(&password, &salt, &stored_hash);
    if valid {
        let master_key = crypto::derive_key(&password, &salt);
        let mut mk = key_state.0.lock().map_err(|e| e.to_string())?;
        *mk = Some(master_key);
    }
    Ok(valid)
}

#[tauri::command]
pub fn is_vault_initialized(vault: State<AppVault>) -> Result<bool, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    Ok(storage.is_initialized())
}

#[tauri::command]
pub fn get_entries(vault: State<AppVault>) -> Result<Vec<VaultEntry>, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    Ok(storage.get_entries())
}

#[tauri::command]
pub fn add_entry(entry: VaultEntry, vault: State<AppVault>, key_state: State<MasterKey>) -> Result<String, String> {
    let mk = key_state.0.lock().map_err(|e| e.to_string())?;
    let master_key = mk.ok_or("not unlocked")?;
    let encrypted = crypto::encrypt(&entry.encrypted_password, &master_key, &entry.id)?;
    let mut encrypted_entry = entry.clone();
    encrypted_entry.encrypted_password = encrypted;
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    storage.add_entry(encrypted_entry);
    Ok("ok".to_string())
}

#[tauri::command]
pub fn update_entry(entry: VaultEntry, vault: State<AppVault>, key_state: State<MasterKey>) -> Result<String, String> {
    let mk = key_state.0.lock().map_err(|e| e.to_string())?;
    let master_key = mk.ok_or("not unlocked")?;
    let encrypted = crypto::encrypt(&entry.encrypted_password, &master_key, &entry.id)?;
    let mut encrypted_entry = entry.clone();
    encrypted_entry.encrypted_password = encrypted;
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    storage.update_entry(encrypted_entry);
    Ok("ok".to_string())
}

#[tauri::command]
pub fn delete_entry(id: String, vault: State<AppVault>) -> Result<String, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    storage.delete_entry(&id);
    Ok("ok".to_string())
}

#[tauri::command]
pub fn decrypt_password(entry_id: String, encrypted: String, key_state: State<MasterKey>) -> Result<String, String> {
    let mk = key_state.0.lock().map_err(|e| e.to_string())?;
    let master_key = mk.ok_or("not unlocked")?;
    crypto::decrypt(&encrypted, &master_key, &entry_id)
}

#[tauri::command]
pub fn generate_password(length: usize, use_uppercase: bool, use_lowercase: bool, use_digits: bool, use_symbols: bool) -> String {
    crypto::generate_random_password(length, use_uppercase, use_lowercase, use_digits, use_symbols)
}
