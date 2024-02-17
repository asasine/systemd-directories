use systemd_directories::SystemdDirs;

fn main() {
    // NOTE: These will possibly be empty if not running under systemd
    let dirs = SystemdDirs::new();
    println!("runtime dir: {:?}", dirs.runtime_dir());
    println!("state dir: {:?}", dirs.state_dir());
    println!("cache dir: {:?}", dirs.cache_dir());
    println!("log dir: {:?}", dirs.logs_dir());
    println!("config dir: {:?}", dirs.config_dir());
}
