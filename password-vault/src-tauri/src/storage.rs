use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultEntry {
    pub id: String,
    pub name: String,
    pub username: String,
    pub encrypted_password: String,
    pub url: String,
    pub notes: String,
    pub category: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultData {
    pub salt: String,
    pub master_hash: String,
    pub entries: Vec<VaultEntry>,
}

impl Default for VaultData {
    fn default() -> Self {
        VaultData {
            salt: String::new(),
            master_hash: String::new(),
            entries: Vec::new(),
        }
    }
}

pub struct VaultStorage {
    data: Mutex<VaultData>,
    path: PathBuf,
}

impl VaultStorage {
    pub fn new() -> Self {
        let app_data = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        let vault_dir = app_data.join("password-vault");
        fs::create_dir_all(&vault_dir).ok();
        let path = vault_dir.join("vault.json");

        let data = if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            VaultData::default()
        };

        VaultStorage {
            data: Mutex::new(data),
            path,
        }
    }

    pub fn is_initialized(&self) -> bool {
        let data = self.data.lock().unwrap();
        !data.salt.is_empty() && !data.master_hash.is_empty()
    }

    pub fn set_master_password(&self, salt_hex: &str, hash_hex: &str) {
        let mut data = self.data.lock().unwrap();
        data.salt = salt_hex.to_string();
        data.master_hash = hash_hex.to_string();
        self.save(&data);
    }

    pub fn get_salt(&self) -> String {
        self.data.lock().unwrap().salt.clone()
    }

    pub fn get_master_hash(&self) -> String {
        self.data.lock().unwrap().master_hash.clone()
    }

    pub fn get_entries(&self) -> Vec<VaultEntry> {
        self.data.lock().unwrap().entries.clone()
    }

    pub fn add_entry(&self, entry: VaultEntry) {
        let mut data = self.data.lock().unwrap();
        data.entries.push(entry);
        self.save(&data);
    }

    pub fn update_entry(&self, entry: VaultEntry) {
        let mut data = self.data.lock().unwrap();
        if let Some(pos) = data.entries.iter().position(|e| e.id == entry.id) {
            data.entries[pos] = entry;
            self.save(&data);
        }
    }

    pub fn delete_entry(&self, id: &str) {
        let mut data = self.data.lock().unwrap();
        data.entries.retain(|e| e.id != id);
        self.save(&data);
    }

    fn save(&self, data: &VaultData) {
        if let Ok(content) = serde_json::to_string_pretty(data) {
            fs::write(&self.path, content).ok();
        }
    }
}
