//! The [`SystemdDirs`] struct is a snapshot of the environment at the time of its creation.

use std::path::{Path, PathBuf};

use crate::{cache_dirs, config_dirs, logs_dirs, runtime_dirs, state_dirs};

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
    fn as_paths<'a>(&'a self, dirs: &'a [PathBuf]) -> impl Iterator<Item = &'a Path> + 'a {
        dirs.iter().map(|p| p.as_path())
    }
}

impl Default for SystemdDirs {
    fn default() -> Self {
        Self::new()
    }
}
