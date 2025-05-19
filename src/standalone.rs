//! Standalone functions to get systemd directories.

use std::env;
use std::path::{Path, PathBuf};

/// A struct to hold colon-separated paths.
struct ColonSeparatedPaths {
    /// The colon-separated paths.
    paths: String,
}

impl ColonSeparatedPaths {
    /// Returns a new [`ColonSeparatedPaths`] struct with the given paths.
    fn new(paths: String) -> Self {
        Self { paths }
    }

    /// Returns a new [`ColonSeparatedPaths`] struct with the environment variable `env_key`.
    fn from_env_key(env_key: &str) -> Self {
        Self::new(env::var(env_key).unwrap_or_default())
    }

    /// Returns an iterator over the paths.
    fn iter(&self) -> impl Iterator<Item = &Path> {
        self.paths
            .split(':')
            .map(Path::new)
            .filter(|p| !p.as_os_str().is_empty())
    }
}

impl From<ColonSeparatedPaths> for Vec<PathBuf> {
    /// Returns all paths as a vector of [`PathBuf`].
    fn from(paths: ColonSeparatedPaths) -> Self {
        paths.iter().map(PathBuf::from).collect()
    }
}

impl From<ColonSeparatedPaths> for Option<PathBuf> {
    /// Returns the first path as an [`Option<PathBuf>`].
    fn from(paths: ColonSeparatedPaths) -> Self {
        paths.iter().next().map(PathBuf::from)
    }
}

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
    ColonSeparatedPaths::from_env_key("RUNTIME_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("RUNTIME_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("STATE_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("STATE_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("CACHE_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("CACHE_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("LOGS_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("LOGS_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("CONFIGURATION_DIRECTORY").into()
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
    ColonSeparatedPaths::from_env_key("CONFIGURATION_DIRECTORY").into()
}
