// src/core/secret_generator.rs
use anyhow::{Context, Result};
use chrono::Utc;
use mac_address::get_mac_address;
use reqwest::blocking::get;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
use regex::Regex;

pub struct SecretGenerator;

impl SecretGenerator {
    /// Gather system info, hash it and return hex string
    pub fn generate() -> Result<String> {
        // Get MAC address
        let mac = get_mac_address()
            .context("Failed to get MAC address")?
            .map(|ma| ma.to_string())
            .unwrap_or_default();

        // Get external IP
        let ip = get("https://api.ipify.org")?
            .text()
            .unwrap_or_default();

        // Timestamp with microseconds
        let timestamp = Utc::now().timestamp_micros().to_string();

        // Combine and hash
        let mut hasher = Sha256::new();
        hasher.update(mac);
        hasher.update(ip);
        hasher.update(timestamp);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Write or update APP_SECRET in .env
    pub fn write_to_env(secret: &str) -> Result<()> {
        let env_path = Path::new(".env");
        if !env_path.exists() {
            fs::write(env_path, "")
                .context("Failed to create .env file")?;
        }

        // Read whole .env into memory
        let mut content = fs::read_to_string(env_path)
            .context("Failed to read .env file")?;
        let new_line = format!("APP_SECRET={}{}", secret, if content.ends_with('\n') { "" } else { "\n" });

        // Regex to find existing APP_SECRET line
        let re = Regex::new(r"(?m)^APP_SECRET=.*$")?;
        if re.is_match(&content) {
            // Replace existing line
            content = re.replace_all(&content, &new_line as &str).to_string();
        } else {
            // Append at end
            content.push_str(&new_line);
        }

        // Write back
        fs::write(env_path, content)
            .context("Failed to write .env file")?;
        Ok(())
    }
}
