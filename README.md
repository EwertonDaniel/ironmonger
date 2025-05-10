# ironmonger

**ironmonger** is a lightweight Rust-based command-line tool designed to securely generate and manage your application's `APP_SECRET`. By combining unique system identifiers (MAC address, external IP) with a high-resolution timestamp, ironmonger produces a robust SHA-256–based secret and ensures it’s stored in a local `.env` file.

---

## 🚀 Getting Started

### Prerequisites

* Rust toolchain (1.56+)
* Internet access (for external IP lookup)

### Installation

```bash
# Clone the repo
git clone https://github.com/EchoSistema/ironmonger.git
cd ironmonger

# Build and install locally
cargo install --path .
```

After installation, the `ironmonger` binary will be available in your `PATH`.

---

## 🛠️ Usage

Generate and store a new `APP_SECRET` in your `.env` file:

```bash
ironmonger create-secret
```

* If `.env` does not exist, it will be created.
* If `APP_SECRET` already exists, it will be overwritten with the new value.

---

## ✅ Features

* **Generate secret**: Combines MAC address, external IP, and microsecond timestamp into a SHA-256 hash.
* **Env management**: Creates or updates `.env` with the `APP_SECRET` entry.

-
-
-

---

## 📅 Roadmap

The project is evolving! Here are some planned enhancements:

1. **Secret rotation**: Archive previous secrets before overwriting.
2. **Dry-run mode**: Preview generated secret without writing to disk.
3. **Secret verification**: Validate existing `APP_SECRET` against current system info.
4. \[Add your feature here]

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!
Please check [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## 📝 License

This project is licensed under the MIT License.

---

© 2025- EchoSistema
