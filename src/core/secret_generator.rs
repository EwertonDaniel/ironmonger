// src/core/secret_generator.rs
use anyhow::{Context, Result};
use chrono::Utc;
use mac_address::get_mac_address;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct SecretGenerator;

impl SecretGenerator {
    pub fn generate() -> Result<String> {
        let mac = get_mac_address()
            .context("Failed to get MAC address")?
            .map(|ma| ma.to_string())
            .unwrap_or_default();

        let timestamp = Utc::now().timestamp_micros().to_string();

        let mut hasher = Sha256::new();
        hasher.update(mac);
        hasher.update(timestamp);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    pub fn write_to_env(secret: &str) -> Result<()> {
        let env_path = Path::new(".env");
        if !env_path.exists() {
            fs::File::create(env_path).context("Failed to create .env file")?;
        }

        let file = fs::File::open(env_path).context("Failed to open .env file")?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();
        let mut found = false;

        for line in reader.lines() {
            let l = line?;
            if l.starts_with("APP_SECRET=") {
                lines.push(format!("APP_SECRET={}", secret));
                found = true;
            } else {
                lines.push(l);
            }
        }

        if !found {
            lines.push(String::new());
            lines.push(format!("APP_SECRET={}", secret));
        }

        let mut outfile = fs::File::create(env_path).context("Failed to write .env file")?;
        for line in &lines {
            writeln!(outfile, "{}", line)?;
        }

        Ok(())
    }
}
