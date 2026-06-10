use sha1::{Digest, Sha1};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write, Read};
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

const HIBP_API_URL: &str = "https://api.pwnedpasswords.com/range/";
const HIBP_OFFLINE_DIR: &str = "hibp-offline";
const HIBP_OFFLINE_FILE: &str = "pwned-passwords-sha1-ordered-by-hash-v8.txt";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HibpSettings {
    pub enabled: bool,
    pub auto_scan_enabled: bool,
    pub auto_scan_interval_days: i64,
    pub last_scan_time: Option<String>,
    pub offline_mode: bool,
    pub offline_db_size_mb: f64,
}

impl Default for HibpSettings {
    fn default() -> Self {
        HibpSettings {
            enabled: true,
            auto_scan_enabled: false,
            auto_scan_interval_days: 7,
            last_scan_time: None,
            offline_mode: false,
            offline_db_size_mb: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PwnedResult {
    pub entry_id: String,
    pub is_pwned: bool,
    pub breach_count: u64,
}

pub struct HibpManager {
    settings: Mutex<HibpSettings>,
    offline_hashes: Mutex<Option<HashSet<String>>>,
    settings_path: PathBuf,
    offline_dir: PathBuf,
}

impl HibpManager {
    pub fn new() -> Self {
        let app_data = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        let vault_dir = app_data.join("password-vault");
        let settings_path = vault_dir.join("hibp-settings.json");
        let offline_dir = vault_dir.join(HIBP_OFFLINE_DIR);

        fs::create_dir_all(&offline_dir).ok();

        let settings = if settings_path.exists() {
            let content = fs::read_to_string(&settings_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HibpSettings::default()
        };

        HibpManager {
            settings: Mutex::new(settings),
            offline_hashes: Mutex::new(None),
            settings_path,
            offline_dir,
        }
    }

    pub fn get_settings(&self) -> HibpSettings {
        self.settings.lock().unwrap().clone()
    }

    pub fn save_settings(&self, settings: HibpSettings) {
        let mut s = self.settings.lock().unwrap();
        *s = settings.clone();
        if let Ok(content) = serde_json::to_string_pretty(&settings) {
            fs::write(&self.settings_path, content).ok();
        }
    }

    pub fn update_last_scan(&self, time: String) {
        let mut s = self.settings.lock().unwrap();
        s.last_scan_time = Some(time);
        let settings = s.clone();
        drop(s);
        self.save_settings(settings);
    }

    fn sha1_hash(password: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        hex::encode(result).to_uppercase()
    }

    pub fn check_password_online(&self, password: &str) -> Result<(bool, u64), String> {
        let hash = Self::sha1_hash(password);
        let prefix = &hash[..5];
        let suffix = &hash[5..];

        let url = format!("{}{}", HIBP_API_URL, prefix);

        let client = reqwest::blocking::Client::builder()
            .user_agent("PasswordVault-App/1.0")
            .build()
            .map_err(|e| e.to_string())?;

        let response = client
            .get(&url)
            .send()
            .map_err(|e| format!("HIBP API request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HIBP API returned status: {}", response.status()));
        }

        let body = response.text().map_err(|e| e.to_string())?;

        for line in body.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let hash_suffix = parts[0].trim();
                let count: u64 = parts[1].trim().parse().unwrap_or(0);
                if hash_suffix.eq_ignore_ascii_case(suffix) {
                    return Ok((true, count));
                }
            }
        }

        Ok((false, 0))
    }

    pub fn load_offline_db(&self) -> Result<bool, String> {
        let db_path = self.offline_dir.join(HIBP_OFFLINE_FILE);
        if !db_path.exists() {
            return Ok(false);
        }

        let file = File::open(&db_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut hashes = HashSet::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                let hash = line.split(':').next().unwrap_or("").trim().to_string();
                if !hash.is_empty() {
                    hashes.insert(hash);
                }
            }
        }

        let mut offline = self.offline_hashes.lock().unwrap();
        *offline = Some(hashes);

        let size = fs::metadata(&db_path)
            .map(|m| m.len() as f64 / (1024.0 * 1024.0))
            .unwrap_or(0.0);

        let mut settings = self.settings.lock().unwrap();
        settings.offline_db_size_mb = size;
        let s = settings.clone();
        drop(settings);
        self.save_settings(s);

        Ok(true)
    }

    pub fn check_password_offline(&self, password: &str) -> Result<bool, String> {
        let hash = Self::sha1_hash(password).to_uppercase();
        let hashes = self.offline_hashes.lock().unwrap();

        if let Some(set) = &*hashes {
            Ok(set.contains(&hash))
        } else {
            Err("Offline database not loaded".to_string())
        }
    }

    pub fn is_offline_loaded(&self) -> bool {
        self.offline_hashes.lock().unwrap().is_some()
    }

    pub fn get_offline_db_path(&self) -> PathBuf {
        self.offline_dir.join(HIBP_OFFLINE_FILE)
    }

    pub fn download_offline_db(
        &self,
        on_progress: impl Fn(u64, u64) + Send + 'static,
    ) -> Result<(), String> {
        let url = "https://downloads.pwnedpasswords.com/passwords/pwned-passwords-sha1-ordered-by-hash-v8.7z";
        let archive_path = self.offline_dir.join("pwned-passwords.7z");

        let client = reqwest::blocking::Client::builder()
            .user_agent("PasswordVault-App/1.0")
            .build()
            .map_err(|e| e.to_string())?;

        let mut response = client
            .get(url)
            .send()
            .map_err(|e| format!("Download failed: {}", e))?;

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;

        let mut file = File::create(&archive_path).map_err(|e| e.to_string())?;
        let mut buf = [0u8; 8192];

        loop {
            let n = response.read(&mut buf).map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            file.write_all(&buf[..n]).map_err(|e| e.to_string())?;
            downloaded += n as u64;
            on_progress(downloaded, total_size);
        }

        Ok(())
    }

    pub fn check_password(&self, password: &str) -> Result<(bool, u64), String> {
        let settings = self.settings.lock().unwrap();

        if settings.offline_mode {
            let pwned = self.check_password_offline(password)?;
            Ok((pwned, if pwned { 1 } else { 0 }))
        } else {
            self.check_password_online(password)
        }
    }

    pub fn batch_check(
        &self,
        passwords: Vec<(String, String)>,
        on_progress: impl Fn(usize, usize) + Send + 'static,
    ) -> Vec<PwnedResult> {
        let total = passwords.len();
        let mut results = Vec::new();

        for (i, (entry_id, password)) in passwords.iter().enumerate() {
            let (is_pwned, breach_count) = match self.check_password(password) {
                Ok(r) => r,
                Err(_) => (false, 0),
            };

            results.push(PwnedResult {
                entry_id: entry_id.clone(),
                is_pwned,
                breach_count,
            });

            on_progress(i + 1, total);
        }

        results
    }
}

impl Default for HibpManager {
    fn default() -> Self {
        Self::new()
    }
}
