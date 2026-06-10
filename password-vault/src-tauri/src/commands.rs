use crate::crypto;
use crate::storage::{VaultEntry, VaultStorage, PasswordHistory, MAX_HISTORY_ENTRIES, ExpiryStatus};
use crate::hibp::{HibpManager, HibpSettings, PwnedResult};
use std::sync::Mutex;
use tauri::State;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct AppVault(pub Mutex<VaultStorage>);
pub struct MasterKey(pub Mutex<Option<[u8; 32]>>);
pub struct AppHibp(pub HibpManager);

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

    if encrypted_entry.rotation_days.is_none() {
        encrypted_entry.rotation_days = VaultEntry::default_rotation_days(&encrypted_entry.category);
    }

    if encrypted_entry.expires_at.is_none() {
        if let Some(rd) = encrypted_entry.rotation_days {
            let now = Utc::now();
            let expiry = now + chrono::Duration::days(rd as i64);
            encrypted_entry.expires_at = Some(expiry.to_rfc3339());
        }
    }

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
    let password_changed = !encrypted_entry.encrypted_password.is_empty();

    if !password_changed {
        if let Some(prev) = existing.as_ref() {
            encrypted_entry.encrypted_password = prev.encrypted_password.clone();
            encrypted_entry.is_pwned = prev.is_pwned;
            encrypted_entry.breach_count = prev.breach_count;
            encrypted_entry.last_pwned_check = prev.last_pwned_check.clone();
            encrypted_entry.history = prev.history.clone();
        } else {
            return Err("entry not found".to_string());
        }
    } else {
        if let Some(prev) = existing.as_ref() {
            let old_encrypted = prev.encrypted_password.clone();
            if !old_encrypted.is_empty() {
                let mut history = prev.history.clone();
                history.push(PasswordHistory {
                    encrypted_password: old_encrypted,
                    changed_at: Utc::now().to_rfc3339(),
                });
                if history.len() > MAX_HISTORY_ENTRIES {
                    let overflow = history.len() - MAX_HISTORY_ENTRIES;
                    history.drain(0..overflow);
                }
                encrypted_entry.history = history;
            } else {
                encrypted_entry.history = prev.history.clone();
            }

            if encrypted_entry.rotation_days.is_none() {
                encrypted_entry.rotation_days = prev.rotation_days
                    .or_else(|| VaultEntry::default_rotation_days(&encrypted_entry.category));
            }

            if encrypted_entry.expires_at.is_none() {
                if let Some(rd) = encrypted_entry.rotation_days {
                    let new_expiry = Utc::now() + chrono::Duration::days(rd as i64);
                    encrypted_entry.expires_at = Some(new_expiry.to_rfc3339());
                }
            }
        }
        encrypted_entry.encrypted_password = crypto::encrypt(&entry.encrypted_password, &master_key, &entry.id)?;
    }

    if encrypted_entry.rotation_days.is_none() && encrypted_entry.expires_at.is_none() {
        if let Some(prev) = existing.as_ref() {
            encrypted_entry.rotation_days = prev.rotation_days;
            encrypted_entry.expires_at = prev.expires_at.clone();
        }
        if encrypted_entry.rotation_days.is_none() {
            encrypted_entry.rotation_days = VaultEntry::default_rotation_days(&encrypted_entry.category);
        }
    }
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    storage.update_entry(encrypted_entry);
    Ok("ok".to_string())
}

#[tauri::command]
pub fn decrypt_history_password(
    entry_id: String,
    encrypted: String,
    key_state: State<MasterKey>,
) -> Result<String, String> {
    let mk = key_state.0.lock().map_err(|e| e.to_string())?;
    let master_key = mk.ok_or("not unlocked")?;
    crypto::decrypt(&encrypted, &master_key, &entry_id)
}

#[tauri::command]
pub fn restore_history_password(
    entry_id: String,
    history_index: usize,
    vault: State<AppVault>,
    key_state: State<MasterKey>,
) -> Result<String, String> {
    let mk = key_state.0.lock().map_err(|e| e.to_string())?;
    let _master_key = mk.ok_or("not unlocked")?;

    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let entries = storage.get_entries();
    let existing = entries.iter().find(|e| e.id == entry_id).cloned();
    drop(entries);
    drop(storage);

    let existing = existing.ok_or("entry not found")?;
    if history_index >= existing.history.len() {
        return Err("history index out of range".to_string());
    }

    let history_entry = &existing.history[history_index];
    let restored_encrypted = history_entry.encrypted_password.clone();

    let mut new_history = existing.history.clone();
    new_history.truncate(history_index);
    new_history.push(PasswordHistory {
        encrypted_password: existing.encrypted_password.clone(),
        changed_at: Utc::now().to_rfc3339(),
    });
    if new_history.len() > MAX_HISTORY_ENTRIES {
        let overflow = new_history.len() - MAX_HISTORY_ENTRIES;
        new_history.drain(0..overflow);
    }

    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    storage.update_entry_with_history(&entry_id, |e| {
        e.encrypted_password = restored_encrypted;
        e.history = new_history;
        e.is_pwned = false;
        e.breach_count = 0;
        e.last_pwned_check = None;
    });

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
    Ok(hibp.0.get_settings())
}

#[tauri::command]
pub fn save_hibp_settings(settings: HibpSettings, hibp: State<AppHibp>) -> Result<String, String> {
    hibp.0.save_settings(settings);
    Ok("ok".to_string())
}

#[tauri::command]
pub fn check_single_password(
    password: String,
    hibp: State<AppHibp>,
) -> Result<(bool, u64), String> {
    hibp.0.check_password(&password)
}

#[tauri::command]
pub async fn batch_scan_passwords(
    vault: State<'_, AppVault>,
    key_state: State<'_, MasterKey>,
    hibp: State<'_, AppHibp>,
) -> Result<Vec<PwnedResult>, String> {
    let master_key = {
        let mk = key_state.0.lock().map_err(|e| e.to_string())?;
        mk.ok_or_else(|| "not unlocked".to_string())?
    };

    let entries = {
        let storage = vault.0.lock().map_err(|e| e.to_string())?;
        storage.get_entries()
    };

    let mut passwords: Vec<(String, String)> = Vec::new();
    for entry in &entries {
        match crypto::decrypt(&entry.encrypted_password, &master_key, &entry.id) {
            Ok(pw) => passwords.push((entry.id.clone(), pw)),
            Err(_) => {}
        }
    }

    if hibp.0.is_scan_running() {
        return Err("scan already running".to_string());
    }

    let results = hibp.0.batch_check_async(passwords, |_, _| {}).await;

    let now = Utc::now().to_rfc3339();

    {
        let storage = vault.0.lock().map_err(|e| e.to_string())?;
        storage.batch_update_pwned_status(&results, &now);
    }

    hibp.0.update_last_scan(now);

    Ok(results)
}

#[tauri::command]
pub fn is_scan_running(hibp: State<AppHibp>) -> Result<bool, String> {
    Ok(hibp.0.is_scan_running())
}

#[tauri::command]
pub fn cancel_scan(hibp: State<AppHibp>) -> Result<String, String> {
    hibp.0.cancel_scan();
    Ok("ok".to_string())
}

#[tauri::command]
pub fn load_offline_hibp_db(hibp: State<AppHibp>) -> Result<bool, String> {
    hibp.0.load_offline_db()
}

#[tauri::command]
pub fn is_offline_db_loaded(hibp: State<AppHibp>) -> Result<bool, String> {
    Ok(hibp.0.is_offline_loaded())
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

#[tauri::command]
pub fn get_expiring_entries(vault: State<AppVault>) -> Result<Vec<VaultEntry>, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let entries = storage.get_entries();
    Ok(entries.into_iter().filter(|e| {
        let status = e.get_expiry_status();
        status != ExpiryStatus::None && status != ExpiryStatus::Normal
    }).collect())
}

#[tauri::command]
pub fn get_expired_entries(vault: State<AppVault>) -> Result<Vec<VaultEntry>, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let entries = storage.get_entries();
    Ok(entries.into_iter().filter(|e| {
        e.get_expiry_status() == ExpiryStatus::Expired
    }).collect())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpiryNotificationResult {
    pub expired_count: usize,
    pub warning_3days_count: usize,
    pub warning_7days_count: usize,
    pub notified: bool,
}

#[tauri::command]
pub fn check_expiry_and_notify(vault: State<AppVault>) -> Result<ExpiryNotificationResult, String> {
    let storage = vault.0.lock().map_err(|e| e.to_string())?;
    let entries = storage.get_entries();
    drop(storage);

    let mut expired = Vec::new();
    let mut warning_3days = Vec::new();
    let mut warning_7days = Vec::new();

    for entry in &entries {
        match entry.get_expiry_status() {
            ExpiryStatus::Expired => expired.push(entry),
            ExpiryStatus::WarningSoon2 => warning_3days.push(entry),
            ExpiryStatus::WarningSoon => warning_7days.push(entry),
            _ => {}
        }
    }

    Ok(ExpiryNotificationResult {
        expired_count: expired.len(),
        warning_3days_count: warning_3days.len(),
        warning_7days_count: warning_7days.len(),
        notified: false,
    })
}
