# ironmonger

**ironmonger** is a Rust command-line tool for securely generating and persisting an application secret (`APP_SECRET`) in a local `.env` file. It combines your system’s MAC address with a high-resolution (microsecond) timestamp, hashes the data using SHA-256, and writes the resulting hex string into the environment file.

---

## 🏷️ Version & Metadata

* **Version:** 0.1.0
* **Rust Edition:** 2021 citeturn1file0

---

## 🛠️ Installation

1. Ensure you have Rust (>= 1.56) and Cargo installed.
2. Clone the repository:

   ```bash
   git clone https://github.com/EchoSistema/ironmonger.git
   cd ironmonger
   ```
3. Build and install:

   ```bash
   cargo install --path .
   ```
4. Verify installation:

   ```bash
   ironmonger --help
   ```

After installation, the `ironmonger` executable will be available in your `PATH`.

---

## 🚀 Usage

Generate and save a new `APP_SECRET`:

```bash
ironmonger create:secret
```

**What happens:**

* Creates a `.env` file if one does not exist.
* Inserts or updates the `APP_SECRET` key with a newly generated SHA-256 hex string.
* Prints the generated secret:

````
New APP_SECRET generated and saved: <secret_value>
``` citeturn1file1

Clap also provides built-in help and version flags:
```bash
ironmonger --help
ironmonger --version
````

---

## 🔧 How It Works

1. **Secret Generation**:

    * Retrieves the system MAC address using `mac_address` citeturn1file2.
    * Captures the current UTC time in microseconds via `chrono` citeturn1file2.
    * Feeds both into a SHA-256 hasher (`sha2`) and encodes the result in hex (`hex`) citeturn1file2.
2. **Environment Management**:

    * Reads existing `.env` lines (`dotenvy`–style) and replaces any `APP_SECRET=` entry, or appends one if not present.
    * Writes the updated lines back to `.env`.
    * Uses `anyhow` for streamlined error handling citeturn1file2.

---

## 📦 Dependencies

* **clap** (4.1) – CLI argument parsing citeturn1file0
* **dotenvy** (0.15) – `.env` file creation and reading citeturn1file0
* **mac\_address** (1.1) – Fetch system MAC address citeturn1file2
* **chrono** (0.4) – High-precision timestamps citeturn1file2
* **sha2** (0.10) & **hex** (0.4) – SHA-256 hashing & hex encoding citeturn1file2
* **anyhow** (1.0) – Error handling citeturn1file2
* **regex** (1.11) – (Reserved for future use) citeturn1file0

---

## 🛤️ Roadmap

* **Additional Commands**: e.g., `rotate-secret`, `verify-secret`, etc.
* **Dry-Run Mode**: Preview new secrets without writing to disk.
* **Environment Profiles**: Support for multiple `.env` files (development, staging, production).
* **Secret Archiving**: Maintain a history of previous secrets for rollback.

---

## 🤝 Contributing

1. Fork the repo.
2. Create a new branch (`git checkout -b feature/foo`).
3. Commit your changes (`git commit -m "feat: add foo feature"`).
4. Push to the branch (`git push origin feature/foo`).
5. Open a Pull Request.

Please adhere to the existing code style and include tests where applicable.

---

## 📄 License

This project is licensed under the MIT License.

© 2025 EchoSistema
