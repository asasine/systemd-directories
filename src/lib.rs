//! The _systemd-directories_ crate is a tiny library to retrieve systemd directories following
//! [systemd.exec(5)](https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RuntimeDirectory=).
//!
//! The library can be used in two ways:
//! - As standalone functions to retrieve the directories.
//! - As a struct to snapshot the environment at the time of creating [`SystemdDirs`]
//!
//! # Examples
//! ## Standalone functions
//! ```
//! use systemd_directories;
//! let runtime_dir = systemd_directories::runtime_dir();
//! ```
//!
//! ## [`SystemdDirs`] Struct
//! ```
//! use systemd_directories::SystemdDirs;
//! let dirs = SystemdDirs::new();
//! let runtime_dir = dirs.runtime_dir();
//! ```

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]

mod standalone;
mod systemd_dirs;
#[cfg(test)]
mod tests;

pub use standalone::{
    cache_dir, cache_dirs, config_dir, config_dirs, logs_dir, logs_dirs, runtime_dir, runtime_dirs,
    state_dir, state_dirs,
};
pub use systemd_dirs::SystemdDirs;
