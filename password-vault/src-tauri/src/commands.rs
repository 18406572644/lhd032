use crate::crypto;
use crate::storage::{VaultEntry, VaultStorage};
use crate::hibp::{HibpManager, HibpSettings, PwnedResult};
use std::sync::Mutex;
use tauri::State;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct AppVault(pub Mutex<VaultStorage>);
pub struct MasterKey(pub Mutex<Option<[u8; 32]>>);
pub struct AppHibp(pub Mutex<HibpManager>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecuritySettings {
    pub auto_lock_timeout_minutes: i64,
    pub lock_on_minimize: bool,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        SecuritySettings {
            auto_lock_timeout_minutes: 5,
            lock_on_minimize: false,
        }
    }
}

pub struct AppSecurity(pub Mutex<SecurityManager>);

pub struct SecurityManager {
    settings: SecuritySettings,
    settings_path: PathBuf,
}

impl SecurityManager {
    pub fn new() -> Self {
        let app_data = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        let vault_dir = app_data.join("password-vault");
        fs::create_dir_all(&vault_dir).ok();
        let settings_path = vault_dir.join("security-settings.json");

        let settings = if settings_path.exists() {
            let content = fs::read_to_string(&settings_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            SecuritySettings::default()
        };

        SecurityManager {
            settings,
            settings_path,
        }
    }

    pub fn get_settings(&self) -> SecuritySettings {
        self.settings.clone()
    }

    pub fn save_settings(&mut self, settings: SecuritySettings) {
        self.settings = settings.clone();
        if let Ok(content) = serde_json::to_string_pretty(&settings) {
            fs::write(&self.settings_path, content).ok();
        }
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub fn setup_master_password(password: String, vault: State<AppVault>, key_state: State<MasterKey>) -> Result<String, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let salt = crypto::generate_salt();
    let hash = crypto::hash_password(&password, &salt);
    let salt_hex = hex::encode(&salt);
    storage.set_master_password(&salt_hex, &hash);
    drop(storage);

    let master_key = crypto::derive_key(&password, &salt);
    let mut mk = key_state.0.lock().map_err(|e| e.to_string())?;
    *mk = Some(master_key);
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
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let entries = storage.get_entries();
    let existing = entries.iter().find(|e| e.id == entry.id).cloned();
    drop(entries);
    drop(storage);

    let mut encrypted_entry = entry.clone();
    if encrypted_entry.encrypted_password.is_empty() {
        if let Some(prev) = existing.as_ref() {
            encrypted_entry.encrypted_password = prev.encrypted_password.clone();
        } else {
            return Err("entry not found".to_string());
        }
    } else {
        encrypted_entry.encrypted_password = crypto::encrypt(&entry.encrypted_password, &master_key, &entry.id)?;
    }
    if let (Some(prev), false) = (existing.as_ref(), encrypted_entry.is_pwned) {
        encrypted_entry.is_pwned = prev.is_pwned;
        encrypted_entry.breach_count = prev.breach_count;
        encrypted_entry.last_pwned_check = prev.last_pwned_check.clone();
    }
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

#[tauri::command]
pub fn get_hibp_settings(hibp: State<AppHibp>) -> Result<HibpSettings, String> {
    let manager = hibp.0.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_settings())
}

#[tauri::command]
pub fn save_hibp_settings(settings: HibpSettings, hibp: State<AppHibp>) -> Result<String, String> {
    let manager = hibp.0.lock().map_err(|e| e.to_string())?;
    manager.save_settings(settings);
    Ok("ok".to_string())
}

#[tauri::command]
pub fn check_single_password(
    password: String,
    hibp: State<AppHibp>,
) -> Result<(bool, u64), String> {
    let manager = hibp.0.lock().map_err(|e| e.to_string())?;
    manager.check_password(&password)
}

#[tauri::command]
pub fn batch_scan_passwords(
    vault: State<AppVault>,
    key_state: State<MasterKey>,
    hibp: State<AppHibp>,
) -> Result<Vec<PwnedResult>, String> {
    let mk = key_state.0.lock().map_err(|e| e.to_string())?;
    let master_key = mk.ok_or("not unlocked")?;

    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let entries = storage.get_entries();
    drop(storage);

    let mut passwords: Vec<(String, String)> = Vec::new();
    for entry in &entries {
        match crypto::decrypt(&entry.encrypted_password, &master_key, &entry.id) {
            Ok(pw) => passwords.push((entry.id.clone(), pw)),
            Err(_) => {}
        }
    }

    let manager = hibp.0.lock().map_err(|e| e.to_string())?;
    let results = manager.batch_check(passwords, |_, _| {});

    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    for result in &results {
        let entries = storage.get_entries();
        if let Some(pos) = entries.iter().position(|e| e.id == result.entry_id) {
            let mut entry = entries[pos].clone();
            entry.is_pwned = result.is_pwned;
            entry.breach_count = result.breach_count;
            entry.last_pwned_check = Some(now.clone());
            storage.update_entry(entry);
        }
    }

    let manager = hibp.0.lock().map_err(|e| e.to_string())?;
    manager.update_last_scan(now);

    Ok(results)
}

#[tauri::command]
pub fn load_offline_hibp_db(hibp: State<AppHibp>) -> Result<bool, String> {
    let manager = hibp.0.lock().map_err(|e| e.to_string())?;
    manager.load_offline_db()
}

#[tauri::command]
pub fn is_offline_db_loaded(hibp: State<AppHibp>) -> Result<bool, String> {
    let manager = hibp.0.lock().map_err(|e| e.to_string())?;
    Ok(manager.is_offline_loaded())
}

#[tauri::command]
pub fn get_pwned_entries(vault: State<AppVault>) -> Result<Vec<VaultEntry>, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let entries = storage.get_entries();
    Ok(entries.into_iter().filter(|e| e.is_pwned).collect())
}

#[tauri::command]
pub fn lock_vault(key_state: State<MasterKey>) -> Result<String, String> {
    let mut mk = key_state.0.lock().map_err(|e| e.to_string())?;
    *mk = None;
    Ok("ok".to_string())
}

#[tauri::command]
pub fn get_security_settings(security: State<AppSecurity>) -> Result<SecuritySettings, String> {
    let manager = security.0.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_settings())
}

#[tauri::command]
pub fn save_security_settings(settings: SecuritySettings, security: State<AppSecurity>) -> Result<String, String> {
    let mut manager = security.0.lock().map_err(|e| e.to_string())?;
    manager.save_settings(settings);
    Ok("ok".to_string())
}
