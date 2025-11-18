use ironmonger::domain::secret::AppSecret;
use ironmonger::infrastructure::env_writer::EnvFileWriter;
use ironmonger::infrastructure::ENV_FILE_PATH;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn create_test_secret() -> AppSecret {
    AppSecret::new("a".repeat(192)).unwrap()
}

#[test]
fn test_new_env_writer() {
    let writer = EnvFileWriter::new(Path::new(".env"), "TEST_SECRET");
    assert_eq!(writer.env_path(), &PathBuf::from(".env"));
    assert_eq!(writer.key_name(), "TEST_SECRET");
}

#[test]
fn test_default_env_writer() {
    let writer = EnvFileWriter::default();
    assert_eq!(writer.env_path(), &PathBuf::from(ENV_FILE_PATH));
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
