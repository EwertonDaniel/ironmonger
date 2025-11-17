# Ironmonger - Documentación Técnica

> **[English](../DOCUMENTATION.md)** | **[Português](DOCUMENTATION.pt-BR.md)**

## Índice

1. [Arquitectura](#arquitectura)
2. [Estructura de Directorios](#estructura-de-directorios)
3. [Módulos y Componentes](#módulos-y-componentes)
4. [Seguridad](#seguridad)
5. [Uso](#uso)
6. [Ejemplos de Código](#ejemplos-de-código)

---

## Arquitectura

**Ironmonger** sigue los principios de **Clean Architecture** y **Domain-Driven Design** (DDD), separando claramente:

- **Domain**: Reglas de negocio y tipos de dominio
- **Infrastructure**: Implementaciones técnicas (generación de secrets, I/O)
- **Application**: Capa de aplicación (CLI)

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

## Estructura de Directorios

```
ironmonger/
├── src/
│   ├── domain/
│   │   ├── mod.rs              # Exporta módulos de dominio
│   │   ├── errors.rs           # Errores personalizados (SecretError)
│   │   └── secret.rs           # AppSecret newtype
│   │
│   ├── infrastructure/
│   │   ├── mod.rs              # Constantes y exportación de módulos
│   │   ├── secret_generator.rs # Generación criptográfica de secrets
│   │   └── env_writer.rs       # Persistencia en archivos .env
│   │
│   ├── lib.rs                  # Biblioteca pública
│   └── main.rs                 # Punto de entrada CLI
│
├── docs/                       # Documentación en otros idiomas
├── Cargo.toml                  # Dependencias y metadatos
├── README.md                   # Documentación de uso
└── DOCUMENTATION.md            # Este archivo
```

---

## Seguridad

### Fuentes de Entropía

El secret se genera combinando múltiples fuentes de entropía:

| Fuente | Tamaño | Propósito |
|--------|--------|-----------|
| Dirección MAC | ~17 bytes | Identificación única del hardware |
| Timestamp | ~30 bytes | Timestamp nanosegundos + microsegundos |
| Process ID | 8 bytes | Variación por ejecución |
| Random Bytes | 32 bytes | Entropía criptográficamente segura (CSPRNG) |
| Hostname | Variable | Identificación del sistema |

**Total**: ~87+ bytes de entropía bruta

### Algoritmos Criptográficos

#### PBKDF2-HMAC-SHA256
- **Iteraciones**: 600.000 (según OWASP 2023)
- **Propósito**: Derivación de claves con protección contra fuerza bruta
- **Salt**: 32 bytes aleatorios únicos por generación

#### SHA3-512
- **Propósito**: Hash final criptográficamente fuerte
- **Salida**: 512 bits (tomamos 256 bits = 64 caracteres hex)
- **Ventaja**: Resistente a ataques de extensión de longitud

### Propiedades de Seguridad

**No determinístico**: Cada ejecución genera un secret diferente

**Resistente a fuerza bruta**: PBKDF2 con 600k iteraciones hace los ataques computacionalmente costosos

**Alto nivel de entropía**: ~87+ bytes de múltiples fuentes

**Algoritmos modernos**: SHA3-512 (Keccak) aprobado por NIST

**Salt único**: Previene ataques de tablas arcoíris

---

## Uso

### Instalación

```bash
git clone https://github.com/EwertonDaniel/ironmonger.git
cd ironmonger
cargo install --path .
```

### Uso Básico

#### Generar APP_SECRET (predeterminado)
```bash
ironmonger create:secret
```

**Salida:**
```
✓ New APP_SECRET generated and saved to .env
  Secret: a3f5d8c2e9b1f4a7c6e8d3b2f9a1c4e7b3d6f8a2c5e9b1d4f7a3c6e8b2d5f9a1
```

#### Generar JWT_SECRET personalizado
```bash
ironmonger create:secret -n JWT_SECRET
```

#### Generar en archivo personalizado
```bash
ironmonger create:secret -n DATABASE_SECRET -f config/.env.production
```

### Uso Programático (Biblioteca)

```rust
use ironmonger::infrastructure::secret_generator::SecretGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generator = SecretGenerator::new();
    let secret = generator.generate()?;
    println!("Secret generado: {}", secret);
    Ok(())
}
```

---

## Pruebas

### Ejecutar todas las pruebas
```bash
cargo test
```

### Cobertura de Pruebas

- **Domain Layer**: 100% (6/6 pruebas)
- **Infrastructure Layer**: 100% (13/13 pruebas)
- **Total**: 19 pruebas unitarias

---

## Rendimiento

### Tiempo de Generación

En un sistema moderno (CPU 4 cores, 3.5 GHz):

- **Generación de secret**: ~600ms (debido a PBKDF2 con 600k iteraciones)
- **Escritura en .env**: <1ms

**Nota**: El tiempo elevado de generación es **intencional** para seguridad contra fuerza bruta.

---

## Contribuir

1. Fork el repositorio
2. Crea una rama (`git checkout -b feature/nueva-caracteristica`)
3. Commit tus cambios (`git commit -m 'feat: añade nueva característica'`)
4. Push a la rama (`git push origin feature/nueva-caracteristica`)
5. Abre un Pull Request

### Convenciones

- Los commits siguen [Conventional Commits](https://www.conventionalcommits.org/)
- Código formateado con `cargo fmt`
- Cero advertencias de `cargo clippy`
- Todas las pruebas deben pasar

---

## Licencia

Licencia MIT - ver [LICENSE](../LICENSE) para detalles.

## Autor

**Ewerton Daniel**
GitHub: [@EwertonDaniel](https://github.com/EwertonDaniel)
