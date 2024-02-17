use systemd_directories::SystemdDirs;

fn main() {
    // NOTE: These will possibly be empty if not running under systemd
    let dirs = SystemdDirs::new();
    println!("runtime dir: {:?}", dirs.runtime_dirs());
    println!("state dir: {:?}", dirs.state_dirs());
    println!("cache dir: {:?}", dirs.cache_dirs());
    println!("log dir: {:?}", dirs.logs_dirs());
    println!("config dir: {:?}", dirs.config_dirs());
}
