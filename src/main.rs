// src/main.rs
mod sysinfo;

use sysinfo::{get_system_info, SystemInfo};

fn main() {
    let system_info: SystemInfo = get_system_info();

    println!("distro: {}", system_info.os_name);
    println!("krnl: {}", system_info.kernel_version);
    println!("de: {}", system_info.desktop_environment);
    println!("ram: {}", system_info.ram_usage);
    println!("pkgs: {}", system_info.package_count);
    println!("cpu: {}", system_info.cpu);
    println!("init: {}", system_info.init);
}