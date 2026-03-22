# hozon — Encrypted Device Backup

Full device backup with age encryption and BLAKE3 manifest via fudajiku. Consumes AdbTransport, ArchiveReader, StorageBackend traits.

## Build & Test

```bash
cargo build
cargo test
cargo run -- backup
cargo run -- restore
```

## Conventions

- Edition 2024, Rust 1.91.0+, MIT, clippy pedantic
- Release: codegen-units=1, lto=true, opt-level="z", strip=true
