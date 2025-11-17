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

    fn update_secret_in_lines(&self, mut lines: Vec<String>, secret: &AppSecret) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_secret() -> AppSecret {
        AppSecret::new("a".repeat(192)).unwrap()
    }

    #[test]
    fn test_new_env_writer() {
        let writer = EnvFileWriter::new(Path::new(".env"), "TEST_SECRET");
        assert_eq!(writer.env_path, PathBuf::from(".env"));
        assert_eq!(writer.key_name, "TEST_SECRET");
    }

    #[test]
    fn test_default_env_writer() {
        let writer = EnvFileWriter::default();
        assert_eq!(writer.env_path, PathBuf::from(ENV_FILE_PATH));
    }

    #[test]
    fn test_update_secret_in_lines_new_entry() {
        let writer = EnvFileWriter::default();
        let secret = create_test_secret();
        let lines = vec!["OTHER_VAR=value".to_string()];

        let updated = writer.update_secret_in_lines(lines, &secret);

        assert_eq!(updated.len(), 3);
        assert_eq!(updated[0], "OTHER_VAR=value");
        assert_eq!(updated[1], "");
        assert!(updated[2].starts_with("APP_SECRET="));
    }

    #[test]
    fn test_update_secret_in_lines_replace_existing() {
        let writer = EnvFileWriter::default();
        let secret = create_test_secret();
        let lines = vec![
            "OTHER_VAR=value".to_string(),
            "APP_SECRET=old_secret".to_string(),
        ];

        let updated = writer.update_secret_in_lines(lines, &secret);

        assert_eq!(updated.len(), 2);
        assert_eq!(updated[0], "OTHER_VAR=value");
        assert!(updated[1].starts_with("APP_SECRET="));
        assert!(updated[1].contains(&"a".repeat(64)));
    }

    #[test]
    fn test_write_to_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let env_path = temp_dir.path().join(".env");
        let writer = EnvFileWriter::new(&env_path, "APP_SECRET");
        let secret = create_test_secret();

        let result = writer.write(&secret);
        assert!(result.is_ok());
        assert!(env_path.exists());

        let content = fs::read_to_string(&env_path).unwrap();
        assert!(content.contains("APP_SECRET="));
        assert!(content.contains(&"a".repeat(64)));
    }

    #[test]
    fn test_write_updates_existing_secret() {
        let temp_dir = TempDir::new().unwrap();
        let env_path = temp_dir.path().join(".env");

        fs::write(&env_path, "APP_SECRET=old_value\n").unwrap();

        let writer = EnvFileWriter::new(&env_path, "APP_SECRET");
        let secret = create_test_secret();

        writer.write(&secret).unwrap();

        let content = fs::read_to_string(&env_path).unwrap();
        assert!(content.contains(&"a".repeat(64)));
        assert!(!content.contains("old_value"));
    }

    #[test]
    fn test_custom_key_name() {
        let temp_dir = TempDir::new().unwrap();
        let env_path = temp_dir.path().join(".env");
        let writer = EnvFileWriter::new(&env_path, "JWT_SECRET");
        let secret = create_test_secret();

        writer.write(&secret).unwrap();

        let content = fs::read_to_string(&env_path).unwrap();
        assert!(content.contains("JWT_SECRET="));
        assert!(!content.contains("APP_SECRET="));
    }
}
