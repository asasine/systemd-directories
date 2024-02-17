fn main() {
    println!("runtime dir: {:?}", systemd_directories::runtime_dirs());
    println!("state dir: {:?}", systemd_directories::state_dirs());
    println!("cache dir: {:?}", systemd_directories::cache_dirs());
    println!("log dir: {:?}", systemd_directories::logs_dirs());
    println!("config dir: {:?}", systemd_directories::config_dirs());
}
