use ironmonger::infrastructure::secret_generator::SecretGenerator;

#[test]
fn test_secret_generator_new() {
    let generator = SecretGenerator::new();
    assert!(std::mem::size_of_val(&generator) == 0);
}

#[test]
fn test_generate_produces_valid_secret() {
    let generator = SecretGenerator::new();
    let result = generator.generate();
    assert!(result.is_ok());

    let secret = result.unwrap();
    assert_eq!(secret.as_str().len(), 192);
    assert!(secret.as_str().chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_generate_uniqueness() {
    let generator = SecretGenerator::new();
    let secret1 = generator.generate().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1));
    let secret2 = generator.generate().unwrap();

    assert_ne!(secret1.as_str(), secret2.as_str());
}
