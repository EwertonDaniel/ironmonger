# Ironmonger

> **Generador de Secrets Criptográficamente Seguro para Aplicaciones**

> **[English](../README.md)** | **[Português](README.pt-BR.md)**

**Ironmonger** es una herramienta CLI en Rust para generar y administrar secrets de aplicación altamente seguros usando algoritmos criptográficos modernos (PBKDF2 + SHA3-512) y múltiples fuentes de entropía.

[![Rust](https://img.shields.io/badge/rust-1.56%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)

---

## Características

- **Extremadamente Seguro**: PBKDF2-HMAC-SHA256 (600k iteraciones) + SHA3-512
- **Alta Entropía**: Combina MAC, timestamp, PID, hostname y CSPRNG
- **Personalizable**: Elige el nombre de variable (APP_SECRET, JWT_SECRET, etc.)
- **Múltiples Archivos**: Soporte para diferentes archivos .env
- **100% Probado**: 14 pruebas de integración, cero advertencias
- **Código Limpio**: Arquitectura limpia siguiendo SOLID y DDD

---

## Instalación

### Requisitos

- Rust >= 1.56
- Cargo

### Via Clone

```bash
git clone https://github.com/EwertonDaniel/ironmonger.git
cd ironmonger
cargo install --path .
```

---

## Uso

### Generar APP_SECRET (predeterminado)

```bash
ironmonger create:secret
```

**Salida:**
```
✓ New APP_SECRET generated and saved to .env
  Secret: a3f5d8c2e9b1f4a7c6e8d3b2f9a1c4e7b3d6f8a2c5e9b1d4f7a3c6e8b2d5f9a1
```

### Generar con Nombre Personalizado

```bash
# JWT Secret
ironmonger create:secret -n JWT_SECRET

# Database Secret
ironmonger create:secret -n DATABASE_SECRET
```

### Generar en Archivo Personalizado

```bash
ironmonger create:secret -n API_KEY -f config/.env.production
```

---

## Seguridad

### Propiedades de Seguridad

**No determinístico**: Cada ejecución genera un secret único

**Resistente a Fuerza Bruta**: PBKDF2 con 600k iteraciones

**Alta Entropía**: ~87+ bytes de múltiples fuentes

**Algoritmos Modernos**: SHA3-512 (aprobado por NIST)

**Salt Único**: Previene ataques de tablas arcoíris

### Comparación de Seguridad

| Aspecto | Versión Simple | Ironmonger (Actual) |
|---------|----------------|---------------------|
| Entropía | ~47 bytes | ~87+ bytes |
| Algoritmo | SHA-256 simple | PBKDF2 + SHA3-512 |
| Iteraciones | 1 | 600.000 |
| Random bytes | 0 | 32 bytes (CSPRNG) |
| Salt | | (32 bytes) |
| Resistencia | Baja | **Extremadamente Alta** |

---

## Documentación

Para documentación técnica completa, consulte:

**[DOCUMENTATION.md](../DOCUMENTATION.md)** - Arquitectura, módulos, ejemplos de código

**También disponible en:** [Español](DOCUMENTATION.es.md) | [Português](DOCUMENTATION.pt-BR.md)

---

## Pruebas

```bash
cargo test
```

### Estadísticas

- **14 pruebas de integración**
- **100% de cobertura** en APIs públicas
- **Cero advertencias** de clippy

---

## Contribuir

1. Fork el repositorio
2. Crea una rama (`git checkout -b feature/nueva-caracteristica`)
3. Commit siguiendo [Conventional Commits](https://www.conventionalcommits.org/)
4. Ejecuta `cargo fmt` y `cargo clippy`
5. Asegúrate de que `cargo test` pase
6. Push a la rama
7. Abre un Pull Request

---

## Licencia

Licencia MIT - ver [LICENSE](../LICENSE) para detalles.

## Autor

**Ewerton Daniel**
- GitHub: [@EwertonDaniel](https://github.com/EwertonDaniel)

---

**Construido con Rust**
