use crate::domain::errors::{Result, SecretError};
use crate::domain::secret::AppSecret;
use chrono::Utc;
use mac_address::get_mac_address;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::{Sha256, Sha512};
use sha3::{Digest, Sha3_512};

const PBKDF2_ITERATIONS: u32 = 1_000_000;
const SALT_SIZE: usize = 64;
const ENTROPY_SIZE: usize = 128;
const OUTPUT_SIZE: usize = 96;

pub struct SecretGenerator;

impl SecretGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self) -> Result<AppSecret> {
        let entropy = self.collect_entropy()?;
        let salt1 = self.generate_salt();
        let salt2 = self.generate_salt();

        let secret_bytes = self.derive_key_multi_layer(&entropy, &salt1, &salt2);
        let hash = hex::encode(secret_bytes);

        Ok(AppSecret::new_unchecked(hash))
    }

    pub(crate) fn collect_entropy(&self) -> Result<Vec<u8>> {
        let mut entropy = Vec::new();

        entropy.extend_from_slice(self.get_mac_address()?.as_bytes());
        entropy.extend_from_slice(self.get_timestamp().as_bytes());
        entropy.extend_from_slice(&self.get_process_id());
        entropy.extend_from_slice(&self.get_random_bytes());

        let hostname = self.get_hostname();
        entropy.extend_from_slice(hostname.as_bytes());

        Ok(entropy)
    }

    fn get_mac_address(&self) -> Result<String> {
        get_mac_address()
            .map_err(|_| SecretError::NoMacAddress)?
            .map(|ma| ma.to_string())
            .ok_or(SecretError::NoMacAddress)
    }

    fn get_timestamp(&self) -> String {
        let now = Utc::now();
        format!(
            "{}:{}",
            now.timestamp_nanos_opt().unwrap_or(0),
            now.timestamp_micros()
        )
    }

    fn get_process_id(&self) -> [u8; 8] {
        std::process::id().to_le_bytes().repeat(2)[..8]
            .try_into()
            .unwrap()
    }

    pub(crate) fn get_random_bytes(&self) -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        bytes
    }

    fn get_hostname(&self) -> String {
        hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub(crate) fn generate_salt(&self) -> [u8; SALT_SIZE] {
        let mut rng = rand::thread_rng();
        let mut salt = [0u8; SALT_SIZE];
        rng.fill_bytes(&mut salt);
        salt
    }

    fn derive_key_multi_layer(
        &self,
        entropy: &[u8],
        salt1: &[u8],
        salt2: &[u8],
    ) -> [u8; OUTPUT_SIZE] {
        let mut layer1 = [0u8; ENTROPY_SIZE];
        pbkdf2_hmac::<Sha512>(entropy, salt1, PBKDF2_ITERATIONS, &mut layer1);

        let mut layer2 = [0u8; ENTROPY_SIZE];
        pbkdf2_hmac::<Sha256>(&layer1, salt2, PBKDF2_ITERATIONS / 2, &mut layer2);

        let mut combined = Vec::new();
        combined.extend_from_slice(&layer1);
        combined.extend_from_slice(&layer2);

        let mut hasher = Sha3_512::new();
        hasher.update(&combined);
        let hash1 = hasher.finalize();

        let mut hasher2 = Sha3_512::new();
        hasher2.update(hash1);
        hasher2.update(salt1);
        hasher2.update(salt2);
        let hash2 = hasher2.finalize();

        let mut output = [0u8; OUTPUT_SIZE];
        output[..64].copy_from_slice(&hash1[..64]);
        output[64..].copy_from_slice(&hash2[..32]);

        output
    }
}

impl Default for SecretGenerator {
    fn default() -> Self {
        Self::new()
    }
}
