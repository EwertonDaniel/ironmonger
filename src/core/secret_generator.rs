// src/core/secret_generator.rs
use anyhow::{Context, Result};
use chrono::Utc;
use mac_address::get_mac_address;
use reqwest::blocking::get;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use dotenvy::from_path_iter;

pub struct SecretGenerator;

impl SecretGenerator {
    pub fn generate() -> Result<String> {
        let mac = get_mac_address()
            .context("Failed to get MAC address")?
            .map(|ma| ma.to_string())
            .unwrap_or_default();

        let ip = get("https://api.ipify.org")?
            .text()
            .unwrap_or_default();

        let timestamp = Utc::now().timestamp_micros().to_string();

        let mut hasher = Sha256::new();
        hasher.update(mac);
        hasher.update(ip);
        hasher.update(timestamp);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    pub fn write_to_env(secret: &str) -> Result<()> {
        let env_path = Path::new(".env");
        if !env_path.exists() {
            fs::write(env_path, "").context("Failed to create .env file")?;
        }

        let mut vars = from_path_iter(env_path)?
            .filter_map(|item| item.ok())
            .collect::<Vec<_>>();

        vars.retain(|(k, _)| k != "APP_SECRET");
        vars.push(("APP_SECRET".into(), secret.into()));

        let content = vars
            .into_iter()
            .map(|(k, v)| format!("{}={}\n", k, v))
            .collect::<String>();
        fs::write(env_path, content).context("Failed to write .env file")?;
        Ok(())
    }
}
