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

use std::env;
use std::path::{Path, PathBuf};

/// Returns all runtime directories as defined by `RuntimeDirectory` in the unit file.
///
/// If the environment variable `RUNTIME_DIRECTORY` is not set, it returns an empty vector.
/// If it is set, it returns all paths in the colon-separated list.
/// To get just the first path, use [`runtime_dir`].
///
/// # Examples
/// ```
/// let runtime_dirs = systemd_directories::runtime_dirs();
/// ```
pub fn runtime_dirs() -> Vec<PathBuf> {
    env::var("RUNTIME_DIRECTORY")
        .ok()
        .map(|dirs| dirs.split(':').map(PathBuf::from).collect())
        .unwrap_or_default()
}

/// Returns the first runtime directory as defined by `RuntimeDirectory` in the unit file.
///
/// If the environment variable `RUNTIME_DIRECTORY` is not set, it returns [`None`].
/// If it is set, it returns the first path in the colon-separated list.
/// To get all paths, use [`runtime_dirs`].
///
/// # Examples
/// ```
/// # use std::env;
/// # env::set_var("RUNTIME_DIRECTORY", "/run/foo");
/// let runtime_dir = systemd_directories::runtime_dir().unwrap_or_default();
/// ```
///
/// The function returns [`Option<PathBuf>`] which can be used in `if let` statements to handle the case where the environment variable is not set.
///
/// ```
/// if let Some(runtime_dir) = systemd_directories::runtime_dir() {
///     // --snip--
/// }
/// ```
pub fn runtime_dir() -> Option<PathBuf> {
    runtime_dirs().first().cloned()
}

/// Returns all state directories as defined by `StateDirectory` in the unit file.
///
/// If the environment variable `STATE_DIRECTORY` is not set, it returns an empty vector.
/// If it is set, it returns all paths in the colon-separated list.
/// To get just the first path, use [`state_dir`].
///
/// # Examples
/// ```
/// let state_dirs = systemd_directories::state_dirs();
/// ```
pub fn state_dirs() -> Vec<PathBuf> {
    env::var("STATE_DIRECTORY")
        .ok()
        .map(|dirs| dirs.split(':').map(PathBuf::from).collect())
        .unwrap_or_default()
}

/// Returns the first state directory as defined by `StateDirectory` in the unit file.
///
/// If the environment variable `STATE_DIRECTORY` is not set, it returns [`None`].
/// If it is set, it returns the first path in the colon-separated list.
/// To get all paths, use [`state_dirs`].
///
/// # Examples
/// ```
/// # use std::env;
/// # env::set_var("STATE_DIRECTORY", "/var/lib/foo");
/// let state_dir = systemd_directories::state_dir().unwrap_or_default();
/// ```
///
/// The function returns [`Option<PathBuf>`] which can be used in `if let` statements to handle the case where the environment variable is not set.
///
/// ```
/// if let Some(state_dir) = systemd_directories::state_dir() {
///     // --snip--
/// }
/// ```
pub fn state_dir() -> Option<PathBuf> {
    state_dirs().first().cloned()
}

/// Returns all cache directories as defined by `CacheDirectory` in the unit file.
///
/// If the environment variable `CACHE_DIRECTORY` is not set, it returns an empty vector.
/// If it is set, it returns all paths in the colon-separated list.
/// To get just the first path, use [`cache_dir`].
///
/// # Examples
/// ```
/// let cache_dirs = systemd_directories::cache_dirs();
/// ```
pub fn cache_dirs() -> Vec<PathBuf> {
    env::var("CACHE_DIRECTORY")
        .ok()
        .map(|dirs| dirs.split(':').map(PathBuf::from).collect())
        .unwrap_or_default()
}

/// Returns the first cache directory as defined by `CacheDirectory` in the unit file.
///
/// If the environment variable `CACHE_DIRECTORY` is not set, it returns [`None`].
/// If it is set, it returns the first path in the colon-separated list.
/// To get all paths, use [`cache_dirs`].
///
/// # Examples
/// ```
/// # use std::env;
/// # env::set_var("CACHE_DIRECTORY", "/var/cache/foo");
/// let cache_dir = systemd_directories::cache_dir().unwrap_or_default();
/// ```
///
/// The function returns [`Option<PathBuf>`] which can be used in `if let` statements to handle the case where the environment variable is not set.
///
/// ```
/// if let Some(cache_dir) = systemd_directories::cache_dir() {
///     // --snip--
/// }
/// ```
pub fn cache_dir() -> Option<PathBuf> {
    cache_dirs().first().cloned()
}

/// Returns all logs directories as defined by `LogsDirectory` in the unit file.
///
/// If the environment variable `LOGS_DIRECTORY` is not set, it returns an empty vector.
/// If it is set, it returns all paths in the colon-separated list.
/// To get just the first path, use [`logs_dir`].
///
/// # Examples
/// ```
/// let logs_dirs = systemd_directories::logs_dirs();
/// ```
pub fn logs_dirs() -> Vec<PathBuf> {
    env::var("LOGS_DIRECTORY")
        .ok()
        .map(|dirs| dirs.split(':').map(PathBuf::from).collect())
        .unwrap_or_default()
}

/// Returns the first logs directory as defined by `LogsDirectory` in the unit file.
///
/// If the environment variable `LOGS_DIRECTORY` is not set, it returns [`None`].
/// If it is set, it returns the first path in the colon-separated list.
/// To get all paths, use [`logs_dirs`].
///
/// # Examples
/// ```
/// # use std::env;
/// # env::set_var("LOGS_DIRECTORY", "/var/log/foo");
/// let logs_dir = systemd_directories::logs_dir().unwrap_or_default();
/// ```
///
/// The function returns [`Option<PathBuf>`] which can be used in `if let` statements to handle the case where the environment variable is not set.
///
/// ```
/// if let Some(logs_dir) = systemd_directories::logs_dir() {
///     // --snip--
/// }
/// ```
pub fn logs_dir() -> Option<PathBuf> {
    logs_dirs().first().cloned()
}

/// Returns all configuration directories as defined by `ConfigurationDirectory` in the unit file.
///
/// If the environment variable `CONFIGURATION_DIRECTORY` is not set, it returns an empty vector.
/// If it is set, it returns all paths in the colon-separated list.
/// To get just the first path, use [`config_dir`].
///
/// # Examples
/// ```
/// let config_dirs = systemd_directories::config_dirs();
/// ```
pub fn config_dirs() -> Vec<PathBuf> {
    env::var("CONFIGURATION_DIRECTORY")
        .ok()
        .map(|dirs| dirs.split(':').map(PathBuf::from).collect())
        .unwrap_or_default()
}

/// Returns the first configuration directory as defined by `ConfigurationDirectory` in the unit file.
///
/// If the environment variable `CONFIGURATION_DIRECTORY` is not set, it returns [`None`].
/// If it is set, it returns the first path in the colon-separated list.
/// To get all paths, use [`config_dirs`].
///
/// # Examples
/// ```
/// # use std::env;
/// # env::set_var("CONFIGURATION_DIRECTORY", "/etc/foo");
/// let config_dir = systemd_directories::config_dir().unwrap_or_default();
/// ```
///
/// The function returns [`Option<PathBuf>`] which can be used in `if let` statements to handle the case where the environment variable is not set.
///
/// ```
/// if let Some(config_dir) = systemd_directories::config_dir() {
///     // --snip--
/// }
/// ```
pub fn config_dir() -> Option<PathBuf> {
    config_dirs().first().cloned()
}

/// A struct to snapshot the environment at the time of creation.
///
/// The [`SystemdDirs`] methods return [`Path`] objects as immutable references to the paths as available when
/// [`Self::new`] was called. This differs from the standalone functions, which return [`PathBuf`] objects.
#[derive(Debug, Clone)]
pub struct SystemdDirs {
    /// All runtime directories when the struct was created.
    runtime_dirs: Vec<PathBuf>,

    /// All state directories when the struct was created.
    state_dirs: Vec<PathBuf>,

    /// All cache directories when the struct was created.
    cache_dirs: Vec<PathBuf>,

    /// All logs directories when the struct was created.
    logs_dirs: Vec<PathBuf>,

    /// All configuration directories when the struct was created.
    config_dirs: Vec<PathBuf>,
}

impl SystemdDirs {
    /// Returns a new [`SystemdDirs`] struct with a snapshot of the current environment.
    ///
    /// # Examples
    /// ```
    /// use systemd_directories::SystemdDirs;
    /// let dirs = SystemdDirs::new();
    /// ```
    pub fn new() -> Self {
        // TODO: Use env::vars to snapshot the environment or is separate env::var calls sufficient?
        Self {
            runtime_dirs: runtime_dirs(),
            state_dirs: state_dirs(),
            cache_dirs: cache_dirs(),
            logs_dirs: logs_dirs(),
            config_dirs: config_dirs(),
        }
    }

    /// Returns all runtime directories as defined by `RuntimeDirectory` in the unit file.
    ///
    /// If the environment variable `RUNTIME_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns an empty vector.
    /// If it was set, it returns all paths in the colon-separated list.
    /// To get just the first path, use [`Self::runtime_dir`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let runtime_dirs = dirs.runtime_dirs();
    /// ```
    pub fn runtime_dirs(&self) -> Vec<&Path> {
        self.as_paths(&self.runtime_dirs).collect()
    }

    /// Returns the first runtime directory as defined by `RuntimeDirectory` in the unit file.
    ///
    /// If the environment variable `RUNTIME_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns [`None`].
    /// If it was set, it returns the first path in the colon-separated list.
    /// To get all paths, use [`Self::runtime_dirs`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let runtime_dir = dirs.runtime_dir();
    pub fn runtime_dir(&self) -> Option<&Path> {
        self.as_paths(&self.runtime_dirs).next()
    }

    /// Returns all state directories as defined by `StateDirectory` in the unit file.
    ///
    /// If the environment variable `STATE_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns an empty vector.
    /// If it was set, it returns all paths in the colon-separated list.
    /// To get just the first path, use [`Self::state_dir`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let state_dirs = dirs.state_dirs();
    /// ```
    pub fn state_dirs(&self) -> Vec<&Path> {
        self.as_paths(&self.state_dirs).collect()
    }

    /// Returns the first state directory as defined by `StateDirectory` in the unit file.
    ///
    /// If the environment variable `STATE_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns [`None`].
    /// If it was set, it returns the first path in the colon-separated list.
    /// To get all paths, use [`Self::state_dirs`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let state_dir = dirs.state_dir();
    /// ```
    pub fn state_dir(&self) -> Option<&Path> {
        self.as_paths(&self.state_dirs).next()
    }

    /// Returns all cache directories as defined by `CacheDirectory` in the unit file.
    ///
    /// If the environment variable `CACHE_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns an empty vector.
    /// If it was set, it returns all paths in the colon-separated list.
    /// To get just the first path, use [`Self::cache_dir`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let cache_dirs = dirs.cache_dirs();
    /// ```
    pub fn cache_dirs(&self) -> Vec<&Path> {
        self.as_paths(&self.cache_dirs).collect()
    }

    /// Returns the first cache directory as defined by `CacheDirectory` in the unit file.
    ///
    /// If the environment variable `CACHE_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns [`None`].
    /// If it was set, it returns the first path in the colon-separated list.
    /// To get all paths, use [`Self::cache_dirs`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let cache_dir = dirs.cache_dir();
    /// ```
    pub fn cache_dir(&self) -> Option<&Path> {
        self.as_paths(&self.cache_dirs).next()
    }

    /// Returns all logs directories as defined by `LogsDirectory` in the unit file.
    ///
    /// If the environment variable `LOGS_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns an empty vector.
    /// If it was set, it returns all paths in the colon-separated list.
    /// To get just the first path, use [`Self::logs_dir`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let logs_dirs = dirs.logs_dirs();
    /// ```
    pub fn logs_dirs(&self) -> Vec<&Path> {
        self.as_paths(&self.logs_dirs).collect()
    }

    /// Returns the first logs directory as defined by `LogsDirectory` in the unit file.
    ///
    /// If the environment variable `LOGS_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns [`None`].
    /// If it was set, it returns the first path in the colon-separated list.
    /// To get all paths, use [`Self::logs_dirs`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let logs_dir = dirs.logs_dir();
    /// ```
    pub fn logs_dir(&self) -> Option<&Path> {
        self.as_paths(&self.logs_dirs).next()
    }

    /// Returns all configuration directories as defined by `ConfigurationDirectory` in the unit file.
    ///
    /// If the environment variable `CONFIGURATION_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns an empty vector.
    /// If it was set, it returns all paths in the colon-separated list.
    /// To get just the first path, use [`Self::config_dir`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let config_dirs = dirs.config_dirs();
    /// ```
    pub fn config_dirs(&self) -> Vec<&Path> {
        self.as_paths(&self.config_dirs).collect()
    }

    /// Returns the first configuration directory as defined by `ConfigurationDirectory` in the unit file.
    ///
    /// If the environment variable `CONFIGURATION_DIRECTORY` was not set when [`SystemdDirs`] was created, it returns [`None`].
    /// If it was set, it returns the first path in the colon-separated list.
    /// To get all paths, use [`Self::config_dirs`].
    ///
    /// # Examples
    /// ```
    /// let dirs = systemd_directories::SystemdDirs::new();
    /// let config_dir = dirs.config_dir();
    /// ```
    pub fn config_dir(&self) -> Option<&Path> {
        self.as_paths(&self.config_dirs).next()
    }

    /// Helper to map a slice of `PathBuf` to an iterator of `&Path`.
    fn as_paths<'a>(&'a self, dirs: &'a [PathBuf]) -> impl Iterator<Item = &Path> + 'a {
        dirs.iter().map(|p| p.as_path())
    }
}

impl Default for SystemdDirs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    fn test_set(
        env_key: &str,
        dirs: &[&str],
        standalone_single: fn() -> Option<PathBuf>,
        standalone_all: fn() -> Vec<PathBuf>,
        method_single: fn(&SystemdDirs) -> Option<&Path>,
        method_all: fn(&SystemdDirs) -> Vec<&Path>,
    ) {
        env::set_var(env_key, dirs.join(":"));
        assert_eq!(standalone_single(), Some(PathBuf::from(dirs[0])));
        assert_eq!(
            standalone_all(),
            dirs.iter().map(PathBuf::from).collect::<Vec<PathBuf>>()
        );
        let systemd_dirs = SystemdDirs::new();
        assert_eq!(method_single(&systemd_dirs), Some(Path::new(dirs[0])));
        assert_eq!(
            method_all(&systemd_dirs),
            dirs.iter().map(Path::new).collect::<Vec<&Path>>()
        );
    }

    fn test_unset(
        env_key: &str,
        standalone_single: fn() -> Option<PathBuf>,
        standalone_all: fn() -> Vec<PathBuf>,
        method_single: fn(&SystemdDirs) -> Option<&Path>,
        method_all: fn(&SystemdDirs) -> Vec<&Path>,
    ) {
        env::remove_var(env_key);
        assert_eq!(standalone_single(), None);
        assert!(standalone_all().is_empty());
        let systemd_dirs = SystemdDirs::new();
        assert_eq!(method_single(&systemd_dirs), None);
        assert!(method_all(&systemd_dirs).is_empty());
    }

    #[test]
    fn test_runtime_directory() {
        test_set(
            "RUNTIME_DIRECTORY",
            &["/run/foo", "/run/bar"],
            runtime_dir,
            runtime_dirs,
            SystemdDirs::runtime_dir,
            SystemdDirs::runtime_dirs,
        );

        test_unset(
            "RUNTIME_DIRECTORY",
            runtime_dir,
            runtime_dirs,
            SystemdDirs::runtime_dir,
            SystemdDirs::runtime_dirs,
        );
    }

    #[test]
    fn test_state_directory() {
        test_set(
            "STATE_DIRECTORY",
            &["/var/lib/foo", "/var/lib/bar"],
            state_dir,
            state_dirs,
            SystemdDirs::state_dir,
            SystemdDirs::state_dirs,
        );

        test_unset(
            "STATE_DIRECTORY",
            state_dir,
            state_dirs,
            SystemdDirs::state_dir,
            SystemdDirs::state_dirs,
        );
    }

    #[test]
    fn test_cache_directory() {
        test_set(
            "CACHE_DIRECTORY",
            &["/var/cache/foo", "/var/cache/bar"],
            cache_dir,
            cache_dirs,
            SystemdDirs::cache_dir,
            SystemdDirs::cache_dirs,
        );

        test_unset(
            "CACHE_DIRECTORY",
            cache_dir,
            cache_dirs,
            SystemdDirs::cache_dir,
            SystemdDirs::cache_dirs,
        );
    }

    #[test]
    fn test_logs_directory() {
        test_set(
            "LOGS_DIRECTORY",
            &["/var/log/foo", "/var/log/bar"],
            logs_dir,
            logs_dirs,
            SystemdDirs::logs_dir,
            SystemdDirs::logs_dirs,
        );

        test_unset(
            "LOGS_DIRECTORY",
            logs_dir,
            logs_dirs,
            SystemdDirs::logs_dir,
            SystemdDirs::logs_dirs,
        );
    }

    #[test]
    fn test_configuration_directory() {
        test_set(
            "CONFIGURATION_DIRECTORY",
            &["/etc/foo", "/etc/bar"],
            config_dir,
            config_dirs,
            SystemdDirs::config_dir,
            SystemdDirs::config_dirs,
        );

        test_unset(
            "CONFIGURATION_DIRECTORY",
            config_dir,
            config_dirs,
            SystemdDirs::config_dir,
            SystemdDirs::config_dirs,
        );
    }
}
