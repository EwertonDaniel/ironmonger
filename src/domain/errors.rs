use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecretError {
    #[error("No MAC address found on this system")]
    NoMacAddress,

    #[error("Failed to access .env file: {0}")]
    EnvFileAccess(#[from] std::io::Error),

    #[error("Invalid secret format: expected 64 hexadecimal characters")]
    InvalidFormat,

    #[error("Secret operation failed: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SecretError>;
