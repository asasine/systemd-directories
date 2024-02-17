fn main() {
    println!("runtime dir: {:?}", systemd_directories::runtime_dir());
    println!("state dir: {:?}", systemd_directories::state_dir());
    println!("cache dir: {:?}", systemd_directories::cache_dir());
    println!("log dir: {:?}", systemd_directories::logs_dir());
    println!("config dir: {:?}", systemd_directories::config_dir());
}
