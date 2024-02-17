# systemd-directories
A tiny library to retrieve systemd directories following [systemd.exec(5)](https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RuntimeDirectory=).

This library targets Linux when systemd is used as the execution environment.

[![Crates.io Version](https://img.shields.io/crates/v/systemd-directories?logo=rust)](https://crates.io/crates/systemd-directories)
[![Crates.io Documentation](https://docs.rs/systemd-directories/badge.svg)](https://docs.rs/systemd-directories)
[![CI](https://img.shields.io/github/actions/workflow/status/asasine/systemd-directories/rust.yaml?branch=main&logo=github&label=CI)](https://github.com/asasine/systemd-directories/actions/workflows/rust.yaml?query=branch%3Amain)
[![Crates.io Downloads](https://img.shields.io/crates/d/systemd-directories)](https://crates.io/crates/systemd-directories)


# Examples
## Standalone functions
```rust
use systemd_directories;
let runtime_dir = systemd_directories::runtime_dir();
println!("runtime directory: {:?}", runtime_dir);
```

## `SystemdDirs` Struct
```rust
use systemd_directories::SystemdDirs;
let dirs = SystemdDirs::new();
let runtime_dir = dirs.runtime_dir();
println!("runtime directory: {:?}", runtime_dir);
```
