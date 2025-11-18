use crate::domain::errors::{Result, SecretError};
use crate::domain::secret::AppSecret;
use crate::infrastructure::{ENV_FILE_PATH, SECRET_KEY_NAME};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub struct EnvFileWriter {
    env_path: PathBuf,
    key_name: String,
}

impl EnvFileWriter {
    pub fn env_path(&self) -> &PathBuf {
        &self.env_path
    }

    pub fn key_name(&self) -> &str {
        &self.key_name
    }
}

impl EnvFileWriter {
    pub fn new(path: &Path, key_name: &str) -> Self {
        Self {
            env_path: path.to_path_buf(),
            key_name: key_name.to_string(),
        }
    }

    pub fn with_default_path() -> Self {
        Self::new(Path::new(ENV_FILE_PATH), SECRET_KEY_NAME)
    }

    pub fn write(&self, secret: &AppSecret) -> Result<()> {
        self.ensure_file_exists()?;

        let lines = self.read_env_lines()?;
        let updated_lines = self.update_secret_in_lines(lines, secret);

        self.write_env_lines(&updated_lines)?;

        Ok(())
    }

    fn ensure_file_exists(&self) -> Result<()> {
        if !self.env_path.exists() {
            fs::File::create(&self.env_path)?;
        }
        Ok(())
    }

    fn read_env_lines(&self) -> Result<Vec<String>> {
        let file = fs::File::open(&self.env_path)?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .collect::<std::io::Result<Vec<String>>>()
            .map_err(SecretError::from)
    }

    pub(crate) fn update_secret_in_lines(
        &self,
        mut lines: Vec<String>,
        secret: &AppSecret,
    ) -> Vec<String> {
        let secret_line = format!("{}={}", self.key_name, secret.as_str());
        let key_prefix = format!("{}=", self.key_name);
        let mut found = false;

        for line in &mut lines {
            if line.starts_with(&key_prefix) {
                *line = secret_line.clone();
                found = true;
                break;
            }
        }

        if !found {
            if !lines.is_empty() && !lines.last().unwrap().is_empty() {
                lines.push(String::new());
            }
            lines.push(secret_line);
        }

        lines
    }

    fn write_env_lines(&self, lines: &[String]) -> Result<()> {
        let mut file = fs::File::create(&self.env_path)?;

        for line in lines {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }
}

impl Default for EnvFileWriter {
    fn default() -> Self {
        Self::with_default_path()
    }
}
