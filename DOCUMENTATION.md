# Ironmonger - Technical Documentation

> **[Português](docs/DOCUMENTATION.pt-BR.md)** | **[Español](docs/DOCUMENTATION.es.md)**

## Table of Contents

1. [Architecture](#architecture)
2. [Directory Structure](#directory-structure)
3. [Modules and Components](#modules-and-components)
4. [Security](#security)
5. [Usage](#usage)
6. [Code Examples](#code-examples)

---

## Architecture

**Ironmonger** follows **Clean Architecture** and **Domain-Driven Design** (DDD) principles, clearly separating:

- **Domain**: Business rules and domain types
- **Infrastructure**: Technical implementations (secret generation, I/O)
- **Application**: Application layer (CLI)

```
┌──────────────────────────────────────┐
│         Application Layer            │
│            (main.rs)                 │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│      Infrastructure Layer            │
│  - SecretGenerator                   │
│  - EnvFileWriter                     │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│         Domain Layer                 │
│  - AppSecret (newtype)               │
│  - SecretError (errors)              │
└──────────────────────────────────────┘
```

---

## Directory Structure

```
ironmonger/
├── src/
│   ├── domain/
│   │   ├── mod.rs              # Exports domain modules
│   │   ├── errors.rs           # Custom errors (SecretError)
│   │   └── secret.rs           # AppSecret newtype
│   │
│   ├── infrastructure/
│   │   ├── mod.rs              # Constants and module exports
│   │   ├── secret_generator.rs # Cryptographic secret generation
│   │   └── env_writer.rs       # .env file persistence
│   │
│   ├── lib.rs                  # Public library
│   └── main.rs                 # CLI entry point
│
├── docs/                       # Documentation in other languages
├── Cargo.toml                  # Dependencies and metadata
├── README.md                   # Usage documentation
└── DOCUMENTATION.md            # This file
```

---

## Modules and Components

### 1. **Domain Layer** (`src/domain/`)

#### `errors.rs`

Defines domain-specific errors using `thiserror`:

```rust
pub enum SecretError {
    NoMacAddress,              // No MAC address found
    EnvFileAccess(io::Error),  // Error accessing .env file
    InvalidFormat,             // Invalid secret format
    Other(String),             // Generic error
}
```

**Custom result type:**
```rust
pub type Result<T> = std::result::Result<T, SecretError>;
```

#### `secret.rs`

Defines the **newtype pattern** for secrets:

```rust
pub struct AppSecret(String);
```

**Public methods:**

| Method | Description | Returns |
|--------|-------------|---------|
| `new(value: String)` | Creates secret with validation | `Result<AppSecret>` |
| `new_unchecked(value: String)` | Creates without validation (internal) | `AppSecret` |
| `as_str(&self)` | Returns as &str | `&str` |
| `is_valid(&self)` | Validates format (64 hex chars) | `bool` |

**Validations:**
- Exactly 64 characters
- Only hexadecimal characters (0-9, a-f)

---

### 2. **Infrastructure Layer** (`src/infrastructure/`)

#### `secret_generator.rs`

Responsible for **cryptographically secure** secret generation.

**Generation Algorithm:**

1. **Entropy Collection:**
   - System MAC address
   - UTC timestamp (nanoseconds + microseconds)
   - Process ID (PID)
   - 32 random bytes (CSPRNG)
   - System hostname

2. **Key Derivation:**
   - **PBKDF2-HMAC-SHA256** with 600,000 iterations
   - Random 32-byte salt
   - 64-byte output

3. **Final Hash:**
   - **SHA3-512** of PBKDF2 result
   - Take first 32 bytes
   - Encode as hexadecimal (64 characters)

**Security Constants:**
```rust
const PBKDF2_ITERATIONS: u32 = 600_000;  // OWASP recommends 600k+ for SHA-256
const SALT_SIZE: usize = 32;             // 256 bits
const ENTROPY_SIZE: usize = 64;          // 512 bits
```

**Public methods:**

| Method | Description | Returns |
|--------|-------------|---------|
| `new()` | Creates new instance | `SecretGenerator` |
| `generate()` | Generates new secret | `Result<AppSecret>` |

**Private methods:**
- `collect_entropy()`: Collects all entropy sources
- `get_mac_address()`: Gets system MAC address
- `get_timestamp()`: High-precision timestamp
- `get_process_id()`: Current process ID
- `get_random_bytes()`: Generates 32 random bytes (CSPRNG)
- `get_hostname()`: Host name
- `generate_salt()`: Generates random 32-byte salt
- `derive_key()`: Derives key using PBKDF2 + SHA3-512

#### `env_writer.rs`

Responsible for **persisting secrets** to `.env` files.

**Features:**
- Creates `.env` file if it doesn't exist
- Updates existing entry or adds new one
- Preserves other environment variables
- Supports custom key names

**Public methods:**

| Method | Description | Returns |
|--------|-------------|---------|
| `new(path, key_name)` | Creates custom writer | `EnvFileWriter` |
| `with_default_path()` | Creates with defaults | `EnvFileWriter` |
| `write(&secret)` | Writes secret to file | `Result<()>` |

**Private methods:**
- `ensure_file_exists()`: Creates file if necessary
- `read_env_lines()`: Reads all file lines
- `update_secret_in_lines()`: Updates or adds secret
- `write_env_lines()`: Writes lines back to file

#### `mod.rs`

Defines shared constants:

```rust
pub const ENV_FILE_PATH: &str = ".env";
pub const SECRET_KEY_NAME: &str = "APP_SECRET";
```

---

### 3. **Application Layer** (`src/main.rs`)

CLI entry point using `clap`.

**Available command:**
```bash
ironmonger create:secret [OPTIONS]
```

**Options:**
- `-n, --name <KEY_NAME>`: Variable name (default: APP_SECRET)
- `-f, --file <FILE_PATH>`: File path (default: .env)

**Execution flow:**
1. Parse CLI arguments
2. Create `SecretGenerator` instance
3. Generate new secret
4. Create `EnvFileWriter` instance
5. Persist secret to file
6. Display confirmation to user

---

## Security

### Entropy Sources

The secret is generated by combining multiple entropy sources:

| Source | Size | Purpose |
|--------|------|---------|
| MAC Address | ~17 bytes | Unique hardware identification |
| Timestamp | ~30 bytes | Nanoseconds + microseconds timestamp |
| Process ID | 8 bytes | Per-execution variation |
| Random Bytes | 32 bytes | Cryptographically secure entropy (CSPRNG) |
| Hostname | Variable | System identification |

**Total**: ~87+ bytes of raw entropy

### Cryptographic Algorithms

#### PBKDF2-HMAC-SHA256
- **Iterations**: 600,000 (per OWASP 2023)
- **Purpose**: Key derivation with brute-force protection
- **Salt**: Unique 32 random bytes per generation

#### SHA3-512
- **Purpose**: Cryptographically strong final hash
- **Output**: 512 bits (we take 256 bits = 64 hex chars)
- **Advantage**: Resistant to length extension attacks

### Security Properties

✅ **Non-deterministic**: Each execution generates different secret (thanks to RNG and timestamp)

✅ **Brute-force resistant**: PBKDF2 with 600k iterations makes attacks computationally expensive

✅ **High entropy level**: ~87+ bytes from multiple sources

✅ **Modern algorithms**: SHA3-512 (Keccak) NIST-approved

✅ **Unique salt**: Prevents rainbow table attacks

### Comparison with Previous Version

| Aspect | Old Version | New Version (Secure) |
|--------|-------------|---------------------|
| Entropy | ~47 bytes | ~87+ bytes |
| Algorithm | Simple SHA-256 | PBKDF2 + SHA3-512 |
| Iterations | 1 | 600,000 |
| Random bytes | 0 | 32 bytes (CSPRNG) |
| Salt | No | Yes (32 bytes) |
| Brute-force resistance | Low | High |

---

## Usage

### Installation

```bash
git clone https://github.com/EchoSistema/ironmonger.git
cd ironmonger
cargo install --path .
```

### Basic Usage

#### Generate APP_SECRET (default)
```bash
ironmonger create:secret
```

**Output:**
```
✓ New APP_SECRET generated and saved to .env
  Secret: a3f5d8c2e9b1f4a7c6e8d3b2f9a1c4e7b3d6f8a2c5e9b1d4f7a3c6e8b2d5f9a1
```

#### Generate custom JWT_SECRET
```bash
ironmonger create:secret -n JWT_SECRET
```

#### Generate in custom file
```bash
ironmonger create:secret -n DATABASE_SECRET -f config/.env.production
```

### Programmatic Usage (Library)

#### As dependency in Cargo.toml
```toml
[dependencies]
ironmonger = "0.1"
```

#### Example: Generate Secret

```rust
use ironmonger::infrastructure::secret_generator::SecretGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generator = SecretGenerator::new();
    let secret = generator.generate()?;

    println!("Generated secret: {}", secret);

    Ok(())
}
```

#### Example: Save to .env

```rust
use ironmonger::infrastructure::{
    secret_generator::SecretGenerator,
    env_writer::EnvFileWriter,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generator = SecretGenerator::new();
    let secret = generator.generate()?;

    let writer = EnvFileWriter::new(Path::new(".env"), "MY_SECRET");
    writer.write(&secret)?;

    println!("Secret saved successfully!");

    Ok(())
}
```

---

## Code Examples

### Example 1: Simple Generation

```rust
use ironmonger::infrastructure::secret_generator::SecretGenerator;

let generator = SecretGenerator::new();
let secret = generator.generate().unwrap();

assert_eq!(secret.as_str().len(), 64);
assert!(secret.is_valid());
```

### Example 2: Multiple Secrets

```rust
use ironmonger::infrastructure::secret_generator::SecretGenerator;

let generator = SecretGenerator::new();

let app_secret = generator.generate()?;
let jwt_secret = generator.generate()?;
let db_secret = generator.generate()?;

// Each secret is unique
assert_ne!(app_secret.as_str(), jwt_secret.as_str());
assert_ne!(jwt_secret.as_str(), db_secret.as_str());
```

### Example 3: Error Handling

```rust
use ironmonger::infrastructure::secret_generator::SecretGenerator;
use ironmonger::domain::errors::SecretError;

let generator = SecretGenerator::new();

match generator.generate() {
    Ok(secret) => {
        println!("Success: {}", secret);
    }
    Err(SecretError::NoMacAddress) => {
        eprintln!("Error: No MAC address found on this system");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

---

## Dependencies

### Production

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.1 | CLI argument parsing |
| `thiserror` | 1.0 | Error handling derivation |
| `hex` | 0.4 | Hexadecimal encoding |
| `sha2` | 0.10 | SHA-256 (PBKDF2) |
| `sha3` | 0.10 | SHA3-512 (Keccak) |
| `pbkdf2` | 0.12 | Key derivation function |
| `rand` | 0.8 | Cryptographically secure RNG |
| `chrono` | 0.4 | High-precision timestamps |
| `mac_address` | 1.1 | System MAC address retrieval |
| `hostname` | 0.4 | System hostname |

### Development

| Crate | Version | Purpose |
|-------|---------|---------|
| `tempfile` | 3.8 | Temporary file testing |

---

## Testing

### Run all tests
```bash
cargo test
```

### Tests with detailed output
```bash
cargo test -- --nocapture
```

### Specific test
```bash
cargo test test_generate_uniqueness
```

### Test Coverage

- **Domain Layer**: 100% (6/6 tests)
- **Infrastructure Layer**: 100% (13/13 tests)
- **Total**: 19 unit tests

---

## Performance

### Generation Time

On a modern system (4-core CPU, 3.5 GHz):

- **Secret generation**: ~600ms (due to PBKDF2 with 600k iterations)
- **.env writing**: <1ms

**Note**: The high generation time is **intentional** for brute-force security.

### Benchmark

```bash
cargo bench
```

---

## Contributing

1. Fork the repository
2. Create a branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m 'feat: add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

### Conventions

- Commits follow [Conventional Commits](https://www.conventionalcommits.org/)
- Code formatted with `cargo fmt`
- Zero warnings from `cargo clippy`
- All tests must pass

---

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

**EchoSistema**
GitHub: [@EchoSistema](https://github.com/EchoSistema)
