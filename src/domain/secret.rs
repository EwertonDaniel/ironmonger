use super::errors::{Result, SecretError};
use std::fmt;

const SECRET_LENGTH: usize = 192;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppSecret(String);

impl AppSecret {
    pub fn new(value: String) -> Result<Self> {
        if value.len() != SECRET_LENGTH {
            return Err(SecretError::InvalidFormat);
        }

        if !value.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(SecretError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub(crate) fn new_unchecked(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0.len() == SECRET_LENGTH && self.0.chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl fmt::Display for AppSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for AppSecret {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
