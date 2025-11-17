# 🔐 Ironmonger

> **Cryptographically Secure Secret Generator for Applications**

> **[Português](docs/README.pt-BR.md)** | **[Español](docs/README.es.md)**

**Ironmonger** is a Rust CLI tool for generating and managing highly secure application secrets using modern cryptographic algorithms (PBKDF2 + SHA3-512) and multiple entropy sources.

[![Rust](https://img.shields.io/badge/rust-1.56%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## ✨ Features

- 🔒 **Extremely Secure**: PBKDF2-HMAC-SHA256 (600k iterations) + SHA3-512
- 🎲 **High Entropy**: Combines MAC, timestamp, PID, hostname and CSPRNG
- 🎯 **Customizable**: Choose variable name (APP_SECRET, JWT_SECRET, etc.)
- 📁 **Multiple Files**: Support for different .env files
- 🧪 **100% Tested**: 19 unit tests, zero warnings
- ⚡ **Clean Code**: Clean architecture following SOLID and DDD

---

## 🏷️ Information

- **Version**: 0.1.0
- **Rust Edition**: 2021
- **License**: MIT

---

## 🛠️ Installation

### Requirements

- Rust >= 1.56
- Cargo

### Via Clone

```bash
git clone https://github.com/EchoSistema/ironmonger.git
cd ironmonger
cargo install --path .
```

### Verify Installation

```bash
ironmonger --help
```

---

## 🚀 Usage

### Generate APP_SECRET (default)

```bash
ironmonger create:secret
```

**Output:**
```
✓ New APP_SECRET generated and saved to .env
  Secret: a3f5d8c2e9b1f4a7c6e8d3b2f9a1c4e7b3d6f8a2c5e9b1d4f7a3c6e8b2d5f9a1
```

### Generate with Custom Name

```bash
# JWT Secret
ironmonger create:secret -n JWT_SECRET

# Database Secret
ironmonger create:secret -n DATABASE_SECRET
```

### Generate in Custom File

```bash
ironmonger create:secret -n API_KEY -f config/.env.production
```

### Help

```bash
ironmonger create:secret --help
```

**Available options:**
- `-n, --name <KEY_NAME>`: Environment variable name (default: APP_SECRET)
- `-f, --file <FILE_PATH>`: .env file path (default: .env)

---

## 🔧 How It Works

### 1. Entropy Collection

Ironmonger collects entropy from multiple sources:

| Source | Description |
|--------|-------------|
| **MAC Address** | Unique hardware identifier |
| **Timestamp** | UTC in nanoseconds + microseconds |
| **Process ID** | Current process ID |
| **Random Bytes** | 32 bytes from CSPRNG (rand) |
| **Hostname** | System name |

**Total**: ~87+ bytes of raw entropy

### 2. Cryptographic Derivation

```
Entropy (87+ bytes)
        ↓
   PBKDF2-HMAC-SHA256
   (600,000 iterations)
   Salt: 32 random bytes
        ↓
   Output: 64 bytes
        ↓
     SHA3-512
        ↓
   Result: 32 bytes
        ↓
   Hex Encode
        ↓
   Secret: 64 hex characters
```

### 3. Algorithms Used

- **PBKDF2-HMAC-SHA256**: Key derivation with 600,000 iterations (OWASP 2023)
- **SHA3-512** (Keccak): Cryptographically strong final hash
- **Unique Salt**: 32 random bytes per generation

### 4. Persistence

- Creates `.env` file if it doesn't exist
- Updates existing entry or adds new one
- Preserves other environment variables

---

## 📦 Dependencies

### Production

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.1 | CLI argument parsing |
| `thiserror` | 1.0 | Custom error types |
| `sha2` | 0.10 | SHA-256 (PBKDF2) |
| `sha3` | 0.10 | SHA3-512 (Keccak) |
| `pbkdf2` | 0.12 | Key derivation |
| `rand` | 0.8 | CSPRNG |
| `chrono` | 0.4 | Timestamps |
| `mac_address` | 1.1 | System MAC |
| `hostname` | 0.4 | System hostname |
| `hex` | 0.4 | Hex encoding |

---

## 🔐 Security

### Security Properties

✅ **Non-deterministic**: Each execution generates unique secret

✅ **Brute-Force Resistant**: PBKDF2 with 600k iterations

✅ **High Entropy**: ~87+ bytes from multiple sources

✅ **Modern Algorithms**: SHA3-512 (NIST-approved)

✅ **Unique Salt**: Prevents rainbow table attacks

### Security Comparison

| Aspect | Simple Version | Ironmonger (Current) |
|--------|----------------|---------------------|
| Entropy | ~47 bytes | ~87+ bytes |
| Algorithm | Simple SHA-256 | PBKDF2 + SHA3-512 |
| Iterations | 1 | 600,000 |
| Random bytes | 0 | 32 bytes (CSPRNG) |
| Salt | ❌ | ✅ (32 bytes) |
| Resistance | Low | **Extremely High** |

### Generation Time

- **~600ms** per secret (intentional for security)
- High time makes brute-force attacks unfeasible

---

## 📚 Documentation

For complete technical documentation, see:

📖 **[DOCUMENTATION.md](DOCUMENTATION.md)** - Architecture, modules, code examples

### Available Topics:

- Architecture (Clean Architecture + DDD)
- Directory structure
- Detailed modules and components
- Cryptographic security
- Code examples
- How to use as library
- Tests and benchmarks

---

## 🏗️ Architecture

```
┌──────────────────────────────┐
│     Application (CLI)        │
│         main.rs              │
└──────────┬───────────────────┘
           │
┌──────────▼───────────────────┐
│     Infrastructure           │
│  • SecretGenerator           │
│  • EnvFileWriter             │
└──────────┬───────────────────┘
           │
┌──────────▼───────────────────┐
│        Domain                │
│  • AppSecret (newtype)       │
│  • SecretError               │
└──────────────────────────────┘
```

**Principles followed:**
- Clean Code (Robert C. Martin)
- SOLID
- Domain-Driven Design (DDD)
- Newtype Pattern
- Error Handling with Result<T, E>

---

## 🧪 Tests

### Run Tests

```bash
cargo test
```

### Statistics

- **19 unit tests**
- **100% coverage** on critical layers
- **Zero warnings** from clippy
- **Formatted** with cargo fmt

### Specific Tests

```bash
cargo test test_generate_uniqueness
cargo test test_salt_generation
```

---

## 🛤️ Roadmap

- [ ] `rotate-secret` command for automatic rotation
- [ ] `verify-secret` command for validation
- [ ] Dry-run mode (preview without saving)
- [ ] Environment profiles (dev, staging, prod)
- [ ] Archive old secrets
- [ ] Custom Key Derivation support
- [ ] Vault integration (HashiCorp Vault, AWS Secrets Manager)

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a branch (`git checkout -b feature/new-feature`)
3. Commit following [Conventional Commits](https://www.conventionalcommits.org/) (`git commit -m 'feat: add X'`)
4. Run `cargo fmt` and `cargo clippy`
5. Ensure `cargo test` passes
6. Push to the branch (`git push origin feature/new-feature`)
7. Open a Pull Request

### Conventions

- ✅ Code formatted with `cargo fmt`
- ✅ Zero warnings from `cargo clippy`
- ✅ Tests for new features
- ✅ Updated documentation
- ✅ Semantic commits (feat, fix, docs, refactor, test, chore)

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👨‍💻 Author

**EchoSistema**

- GitHub: [@EchoSistema](https://github.com/EchoSistema)
- Repository: [ironmonger](https://github.com/EchoSistema/ironmonger)

---

## 🙏 Acknowledgments

- [OWASP](https://owasp.org/) - Security guidelines
- [NIST](https://www.nist.gov/) - Cryptographic standards
- Rust Community - Amazing tools and libraries

---

**Built with ❤️ in Rust**
