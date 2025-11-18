use ironmonger::domain::secret::AppSecret;

const SECRET_LENGTH: usize = 192;

#[test]
fn test_valid_secret_creation() {
    let valid_secret = "a".repeat(SECRET_LENGTH);
    let secret = AppSecret::new(valid_secret.clone());
    assert!(secret.is_ok());
    assert_eq!(secret.unwrap().as_str(), valid_secret);
}

#[test]
fn test_invalid_length() {
    let invalid_secret = "a".repeat(SECRET_LENGTH - 1);
    let secret = AppSecret::new(invalid_secret);
    assert!(secret.is_err());
}

#[test]
fn test_invalid_characters() {
    let mut invalid = "a".repeat(SECRET_LENGTH - 1);
    invalid.push('g');
    let secret = AppSecret::new(invalid);
    assert!(secret.is_err());
}

#[test]
fn test_is_valid() {
    let valid_secret = AppSecret::new("0123456789abcdef".repeat(SECRET_LENGTH / 16)).unwrap();
    assert!(valid_secret.is_valid());
}

#[test]
fn test_display_trait() {
    let secret = AppSecret::new("a".repeat(SECRET_LENGTH)).unwrap();
    assert_eq!(format!("{}", secret), "a".repeat(SECRET_LENGTH));
}

#[test]
fn test_as_ref() {
    let secret = AppSecret::new("b".repeat(SECRET_LENGTH)).unwrap();
    let s: &str = secret.as_ref();
    assert_eq!(s, "b".repeat(SECRET_LENGTH));
}
