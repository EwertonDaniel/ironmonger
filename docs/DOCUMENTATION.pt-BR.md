# Ironmonger - Documentação Técnica

## Índice

1. [Arquitetura](#arquitetura)
2. [Estrutura de Diretórios](#estrutura-de-diretórios)
3. [Módulos e Componentes](#módulos-e-componentes)
4. [Segurança](#segurança)
5. [Como Usar](#como-usar)
6. [Exemplos de Código](#exemplos-de-código)

---

## Arquitetura

O **Ironmonger** segue os princípios de **Clean Architecture** e **Domain-Driven Design** (DDD), separando claramente:

- **Domain**: Regras de negócio e tipos de domínio
- **Infrastructure**: Implementações técnicas (geração de secrets, I/O)
- **Application**: Camada de aplicação (CLI)

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

## Estrutura de Diretórios

```
ironmonger/
├── src/
│   ├── domain/
│   │   ├── mod.rs              # Exporta módulos do domínio
│   │   ├── errors.rs           # Erros customizados (SecretError)
│   │   └── secret.rs           # AppSecret newtype
│   │
│   ├── infrastructure/
│   │   ├── mod.rs              # Constantes e exporta módulos
│   │   ├── secret_generator.rs # Geração criptográfica de secrets
│   │   └── env_writer.rs       # Persistência em arquivos .env
│   │
│   ├── lib.rs                  # Biblioteca pública
│   └── main.rs                 # CLI entry point
│
├── Cargo.toml                  # Dependências e metadados
├── README.md                   # Documentação de uso
└── DOCUMENTATION.md            # Este arquivo
```

---

## Módulos e Componentes

### 1. **Domain Layer** (`src/domain/`)

#### `errors.rs`

Define os erros específicos do domínio usando `thiserror`:

```rust
pub enum SecretError {
    NoMacAddress,              // Nenhum MAC address encontrado
    EnvFileAccess(io::Error),  // Erro ao acessar arquivo .env
    InvalidFormat,             // Formato de secret inválido
    Other(String),             // Erro genérico
}
```

**Tipo de resultado customizado:**
```rust
pub type Result<T> = std::result::Result<T, SecretError>;
```

#### `secret.rs`

Define o **newtype pattern** para secrets:

```rust
pub struct AppSecret(String);
```

**Métodos públicos:**

| Método | Descrição | Retorno |
|--------|-----------|---------|
| `new(value: String)` | Cria secret com validação | `Result<AppSecret>` |
| `new_unchecked(value: String)` | Cria sem validação (interno) | `AppSecret` |
| `as_str(&self)` | Retorna como &str | `&str` |
| `is_valid(&self)` | Valida formato (64 hex chars) | `bool` |

**Validações:**
- Exatamente 64 caracteres
- Apenas caracteres hexadecimais (0-9, a-f)

---

### 2. **Infrastructure Layer** (`src/infrastructure/`)

#### `secret_generator.rs`

Responsável pela **geração criptograficamente segura** de secrets.

**Algoritmo de Geração:**

1. **Coleta de Entropia:**
   - MAC address do sistema
   - Timestamp UTC (nanosegundos + microsegundos)
   - Process ID (PID)
   - 32 bytes aleatórios (CSPRNG)
   - Hostname do sistema

2. **Derivação de Chave:**
   - **PBKDF2-HMAC-SHA256** com 600.000 iterações
   - Salt aleatório de 32 bytes
   - Saída de 64 bytes

3. **Hash Final:**
   - **SHA3-512** do resultado do PBKDF2
   - Pegamos os primeiros 32 bytes
   - Codificamos em hexadecimal (64 caracteres)

**Constantes de Segurança:**
```rust
const PBKDF2_ITERATIONS: u32 = 600_000;  // OWASP recomenda 600k+ para SHA-256
const SALT_SIZE: usize = 32;             // 256 bits
const ENTROPY_SIZE: usize = 64;          // 512 bits
```

**Métodos públicos:**

| Método | Descrição | Retorno |
|--------|-----------|---------|
| `new()` | Cria nova instância | `SecretGenerator` |
| `generate()` | Gera novo secret | `Result<AppSecret>` |

**Métodos privados:**
- `collect_entropy()`: Coleta todas as fontes de entropia
- `get_mac_address()`: Obtém MAC address do sistema
- `get_timestamp()`: Timestamp de alta precisão
- `get_process_id()`: ID do processo atual
- `get_random_bytes()`: Gera 32 bytes aleatórios (CSPRNG)
- `get_hostname()`: Nome do host
- `generate_salt()`: Gera salt aleatório de 32 bytes
- `derive_key()`: Deriva chave usando PBKDF2 + SHA3-512

#### `env_writer.rs`

Responsável por **persistir secrets** em arquivos `.env`.

**Funcionalidades:**
- Cria arquivo `.env` se não existir
- Atualiza entrada existente ou adiciona nova
- Preserva outras variáveis de ambiente
- Suporta nomes de chave customizados

**Métodos públicos:**

| Método | Descrição | Retorno |
|--------|-----------|---------|
| `new(path, key_name)` | Cria writer customizado | `EnvFileWriter` |
| `with_default_path()` | Cria com defaults | `EnvFileWriter` |
| `write(&secret)` | Escreve secret no arquivo | `Result<()>` |

**Métodos privados:**
- `ensure_file_exists()`: Cria arquivo se necessário
- `read_env_lines()`: Lê todas as linhas do arquivo
- `update_secret_in_lines()`: Atualiza ou adiciona secret
- `write_env_lines()`: Escreve linhas de volta ao arquivo

#### `mod.rs`

Define constantes compartilhadas:

```rust
pub const ENV_FILE_PATH: &str = ".env";
pub const SECRET_KEY_NAME: &str = "APP_SECRET";
```

---

### 3. **Application Layer** (`src/main.rs`)

Entry point da CLI usando `clap`.

**Comando disponível:**
```bash
ironmonger create:secret [OPTIONS]
```

**Opções:**
- `-n, --name <KEY_NAME>`: Nome da variável (padrão: APP_SECRET)
- `-f, --file <FILE_PATH>`: Caminho do arquivo (padrão: .env)

**Fluxo de execução:**
1. Parse dos argumentos CLI
2. Cria instância de `SecretGenerator`
3. Gera novo secret
4. Cria instância de `EnvFileWriter`
5. Persiste secret no arquivo
6. Exibe confirmação ao usuário

---

## Segurança

### Fontes de Entropia

O secret é gerado combinando múltiplas fontes de entropia:

| Fonte | Tamanho | Propósito |
|-------|---------|-----------|
| MAC Address | ~17 bytes | Identificação única do hardware |
| Timestamp | ~30 bytes | Timestamp nanosegundos + microsegundos |
| Process ID | 8 bytes | Variação por execução |
| Random Bytes | 32 bytes | Entropia criptograficamente segura (CSPRNG) |
| Hostname | Variável | Identificação do sistema |

**Total**: ~87+ bytes de entropia bruta

### Algoritmos Criptográficos

#### PBKDF2-HMAC-SHA256
- **Iterações**: 600.000 (conforme OWASP 2023)
- **Propósito**: Key derivation com proteção contra brute-force
- **Salt**: 32 bytes aleatórios únicos por geração

#### SHA3-512
- **Propósito**: Hash final criptograficamente forte
- **Saída**: 512 bits (pegamos 256 bits = 64 hex chars)
- **Vantagem**: Resistente a ataques de extensão de comprimento

### Propriedades de Segurança

**Não-determinístico**: Cada execução gera secret diferente (graças ao RNG e timestamp)

**Resistente a brute-force**: PBKDF2 com 600k iterações torna ataques computacionalmente caros

**Alto nível de entropia**: ~87+ bytes de múltiplas fontes

**Algoritmos modernos**: SHA3-512 (Keccak) aprovado pelo NIST

**Salt único**: Previne ataques de rainbow table

### Comparação com Versão Anterior

| Aspecto | Versão Antiga | Versão Nova (Segura) |
|---------|---------------|----------------------|
| Entropia | ~47 bytes | ~87+ bytes |
| Algoritmo | SHA-256 simples | PBKDF2 + SHA3-512 |
| Iterações | 1 | 600.000 |
| Random bytes | 0 | 32 bytes (CSPRNG) |
| Salt | Não | Sim (32 bytes) |
| Resistência brute-force | Baixa | Alta |

---

## Como Usar

### Instalação

```bash
git clone https://github.com/EwertonDaniel/ironmonger.git
cd ironmonger
cargo install --path .
```

### Uso Básico

#### Gerar APP_SECRET (padrão)
```bash
ironmonger create:secret
```

**Saída:**
```
✓ New APP_SECRET generated and saved to .env
  Secret: a3f5d8c2e9b1f4a7c6e8d3b2f9a1c4e7b3d6f8a2c5e9b1d4f7a3c6e8b2d5f9a1
```

#### Gerar JWT_SECRET customizado
```bash
ironmonger create:secret -n JWT_SECRET
```

#### Gerar em arquivo customizado
```bash
ironmonger create:secret -n DATABASE_SECRET -f config/.env.production
```

### Uso Programático (Biblioteca)

#### Como dependência no Cargo.toml
```toml
[dependencies]
ironmonger = "0.1"
```

#### Exemplo: Gerar Secret

```rust
use ironmonger::infrastructure::secret_generator::SecretGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generator = SecretGenerator::new();
    let secret = generator.generate()?;

    println!("Generated secret: {}", secret);

    Ok(())
}
```

#### Exemplo: Salvar em .env

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

#### Exemplo: Validar Secret

```rust
use ironmonger::domain::secret::AppSecret;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret_str = "a".repeat(64);

    match AppSecret::new(secret_str) {
        Ok(secret) => {
            println!("Valid secret: {}", secret);
            println!("Is valid: {}", secret.is_valid());
        }
        Err(e) => eprintln!("Invalid secret: {}", e),
    }

    Ok(())
}
```

---

## Exemplos de Código

### Exemplo 1: Geração Simples

```rust
use ironmonger::infrastructure::secret_generator::SecretGenerator;

let generator = SecretGenerator::new();
let secret = generator.generate().unwrap();

assert_eq!(secret.as_str().len(), 64);
assert!(secret.is_valid());
```

### Exemplo 2: Múltiplos Secrets

```rust
use ironmonger::infrastructure::secret_generator::SecretGenerator;

let generator = SecretGenerator::new();

let app_secret = generator.generate()?;
let jwt_secret = generator.generate()?;
let db_secret = generator.generate()?;

// Cada secret é único
assert_ne!(app_secret.as_str(), jwt_secret.as_str());
assert_ne!(jwt_secret.as_str(), db_secret.as_str());
```

### Exemplo 3: Tratamento de Erros

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

### Exemplo 4: Writer Customizado

```rust
use ironmonger::infrastructure::env_writer::EnvFileWriter;
use ironmonger::domain::secret::AppSecret;
use std::path::Path;

let secret = AppSecret::new("a".repeat(64)).unwrap();

let writer = EnvFileWriter::new(
    Path::new("config/.env.local"),
    "CUSTOM_SECRET"
);

writer.write(&secret)?;
```

### Exemplo 5: Integração com Dotenv

```rust
use ironmonger::infrastructure::{
    secret_generator::SecretGenerator,
    env_writer::EnvFileWriter,
};
use std::env;

// Gera e salva
let generator = SecretGenerator::new();
let secret = generator.generate()?;

let writer = EnvFileWriter::default();
writer.write(&secret)?;

// Carrega com dotenv
dotenvy::dotenv()?;

let app_secret = env::var("APP_SECRET")?;
println!("Loaded secret: {}", app_secret);
```

---

## Dependências

### Produção

| Crate | Versão | Propósito |
|-------|--------|-----------|
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

### Desenvolvimento

| Crate | Versão | Propósito |
|-------|--------|-----------|
| `tempfile` | 3.8 | Testes com arquivos temporários |

---

## Testes

### Executar todos os testes
```bash
cargo test
```

### Testes com output detalhado
```bash
cargo test -- --nocapture
```

### Teste específico
```bash
cargo test test_generate_uniqueness
```

### Cobertura de Testes

- **Domain Layer**: 100% (6/6 testes)
- **Infrastructure Layer**: 100% (13/13 testes)
- **Total**: 19 testes unitários

---

## Performance

### Tempo de Geração

Em um sistema moderno (CPU 4 cores, 3.5 GHz):

- **Geração de secret**: ~600ms (devido ao PBKDF2 com 600k iterações)
- **Escrita em .env**: <1ms

**Nota**: O tempo elevado de geração é **intencional** para segurança contra brute-force.

### Benchmark

```bash
cargo bench
```

---

## Contribuindo

1. Fork o repositório
2. Crie uma branch (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -m 'feat: adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

### Convenções

- Commits seguem [Conventional Commits](https://www.conventionalcommits.org/)
- Código formatado com `cargo fmt`
- Zero warnings do `cargo clippy`
- Todos os testes devem passar

---

## Licença

MIT License - veja [LICENSE](LICENSE) para detalhes.

## Autor

**Ewerton Daniel**
GitHub: [@EwertonDaniel](https://github.com/EwertonDaniel)
