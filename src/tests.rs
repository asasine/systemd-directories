use super::*;
use std::env;
use std::path::{Path, PathBuf};

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
