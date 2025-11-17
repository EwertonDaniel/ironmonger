# 🔐 Ironmonger

> **Gerador de Secrets Criptograficamente Seguro para Aplicações**

**Ironmonger** é uma ferramenta CLI em Rust para gerar e gerenciar secrets de aplicação altamente seguros usando algoritmos criptográficos modernos (PBKDF2 + SHA3-512) e múltiplas fontes de entropia.

[![Rust](https://img.shields.io/badge/rust-1.56%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## ✨ Características

- 🔒 **Extremamente Seguro**: PBKDF2-HMAC-SHA256 (600k iterações) + SHA3-512
- 🎲 **Alta Entropia**: Combina MAC, timestamp, PID, hostname e CSPRNG
- 🎯 **Customizável**: Escolha o nome da variável (APP_SECRET, JWT_SECRET, etc.)
- 📁 **Múltiplos Arquivos**: Suporte para diferentes arquivos .env
- 🧪 **100% Testado**: 19 testes unitários, zero warnings
- ⚡ **Clean Code**: Arquitetura limpa seguindo SOLID e DDD

---

## 🏷️ Informações

- **Versão**: 0.1.0
- **Edição Rust**: 2021
- **Licença**: MIT

---

## 🛠️ Instalação

### Requisitos

- Rust >= 1.56
- Cargo

### Via Clone

```bash
git clone https://github.com/EchoSistema/ironmonger.git
cd ironmonger
cargo install --path .
```

### Verificar Instalação

```bash
ironmonger --help
```

---

## 🚀 Uso

### Gerar APP_SECRET (padrão)

```bash
ironmonger create:secret
```

**Saída:**
```
✓ New APP_SECRET generated and saved to .env
  Secret: a3f5d8c2e9b1f4a7c6e8d3b2f9a1c4e7b3d6f8a2c5e9b1d4f7a3c6e8b2d5f9a1
```

### Gerar com Nome Customizado

```bash
# JWT Secret
ironmonger create:secret -n JWT_SECRET

# Database Secret
ironmonger create:secret -n DATABASE_SECRET
```

### Gerar em Arquivo Customizado

```bash
ironmonger create:secret -n API_KEY -f config/.env.production
```

### Ajuda

```bash
ironmonger create:secret --help
```

**Opções disponíveis:**
- `-n, --name <KEY_NAME>`: Nome da variável de ambiente (padrão: APP_SECRET)
- `-f, --file <FILE_PATH>`: Caminho do arquivo .env (padrão: .env)

---

## 🔧 Como Funciona

### 1. Coleta de Entropia

O Ironmonger coleta entropia de múltiplas fontes:

| Fonte | Descrição |
|-------|-----------|
| **MAC Address** | Identificador único do hardware |
| **Timestamp** | UTC em nanosegundos + microsegundos |
| **Process ID** | ID do processo atual |
| **Random Bytes** | 32 bytes do CSPRNG (rand) |
| **Hostname** | Nome do sistema |

**Total**: ~87+ bytes de entropia bruta

### 2. Derivação Criptográfica

```
Entropia (87+ bytes)
        ↓
   PBKDF2-HMAC-SHA256
   (600.000 iterações)
   Salt: 32 bytes aleatórios
        ↓
   Saída: 64 bytes
        ↓
     SHA3-512
        ↓
   Resultado: 32 bytes
        ↓
   Hex Encode
        ↓
   Secret: 64 caracteres hex
```

### 3. Algoritmos Utilizados

- **PBKDF2-HMAC-SHA256**: Key derivation com 600.000 iterações (OWASP 2023)
- **SHA3-512** (Keccak): Hash final criptograficamente forte
- **Salt Único**: 32 bytes aleatórios por geração

### 4. Persistência

- Cria arquivo `.env` se não existir
- Atualiza entrada existente ou adiciona nova
- Preserva outras variáveis de ambiente

---

## 📦 Dependências

### Produção

| Crate | Versão | Propósito |
|-------|--------|-----------|
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

## 🔐 Segurança

### Propriedades de Segurança

✅ **Não-determinístico**: Cada execução gera secret único

✅ **Resistente a Brute-Force**: PBKDF2 com 600k iterações

✅ **Alta Entropia**: ~87+ bytes de múltiplas fontes

✅ **Algoritmos Modernos**: SHA3-512 (NIST-approved)

✅ **Salt Único**: Previne rainbow table attacks

### Comparação de Segurança

| Aspecto | Versão Simples | Ironmonger (Atual) |
|---------|----------------|-------------------|
| Entropia | ~47 bytes | ~87+ bytes |
| Algoritmo | SHA-256 simples | PBKDF2 + SHA3-512 |
| Iterações | 1 | 600.000 |
| Random bytes | 0 | 32 bytes (CSPRNG) |
| Salt | ❌ | ✅ (32 bytes) |
| Resistência | Baixa | Alta |

### Tempo de Geração

- **~600ms** por secret (intencional para segurança)
- O tempo elevado torna ataques de brute-force inviáveis

---

## 📚 Documentação

Para documentação técnica completa, consulte:

📖 **[DOCUMENTATION.md](DOCUMENTATION.md)** - Arquitetura, módulos, exemplos de código

### Tópicos Disponíveis:

- Arquitetura (Clean Architecture + DDD)
- Estrutura de diretórios
- Módulos e componentes detalhados
- Segurança criptográfica
- Exemplos de código
- Como usar como biblioteca
- Testes e benchmarks

---

## 🏗️ Arquitetura

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

**Princípios seguidos:**
- Clean Code (Robert C. Martin)
- SOLID
- Domain-Driven Design (DDD)
- Newtype Pattern
- Error Handling com Result<T, E>

---

## 🧪 Testes

### Executar Testes

```bash
cargo test
```

### Estatísticas

- **19 testes unitários**
- **100% de cobertura** nas camadas críticas
- **Zero warnings** do clippy
- **Formatado** com cargo fmt

### Testes Específicos

```bash
cargo test test_generate_uniqueness
cargo test test_salt_generation
```

---

## 🛤️ Roadmap

- [ ] Comando `rotate-secret` para rotação automática
- [ ] Comando `verify-secret` para validação
- [ ] Modo dry-run (preview sem salvar)
- [ ] Perfis de ambiente (dev, staging, prod)
- [ ] Arquivamento de secrets antigos
- [ ] Suporte a Key Derivation customizado
- [ ] Integração com vaults (HashiCorp Vault, AWS Secrets Manager)

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Fork o repositório
2. Crie uma branch (`git checkout -b feature/nova-feature`)
3. Commit seguindo [Conventional Commits](https://www.conventionalcommits.org/) (`git commit -m 'feat: adiciona X'`)
4. Rode `cargo fmt` e `cargo clippy`
5. Certifique-se de que `cargo test` passa
6. Push para a branch (`git push origin feature/nova-feature`)
7. Abra um Pull Request

### Convenções

- ✅ Código formatado com `cargo fmt`
- ✅ Zero warnings do `cargo clippy`
- ✅ Testes para novas funcionalidades
- ✅ Documentação atualizada
- ✅ Commits semânticos (feat, fix, docs, refactor, test, chore)

---

## 📄 Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

---

## 👨‍💻 Autor

**EchoSistema**

- GitHub: [@EchoSistema](https://github.com/EchoSistema)
- Repository: [ironmonger](https://github.com/EchoSistema/ironmonger)

---

## 🙏 Agradecimentos

- [OWASP](https://owasp.org/) - Guidelines de segurança
- [NIST](https://www.nist.gov/) - Padrões criptográficos
- Rust Community - Ferramentas e bibliotecas incríveis

---

**Desenvolvido com ❤️ em Rust**
