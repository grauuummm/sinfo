pub struct SystemInfo {
    pub os_name: String,
    pub kernel_version: String,
    pub desktop_environment: String,
    pub ram_usage: String,
    pub package_count: String,
    pub cpu: String,
    pub init: String, 
}

impl SystemInfo {
    pub fn new() -> Self {
        let os_name = std::fs::read_to_string("/etc/os-release")
            .unwrap_or_else(|_| "unknown distro".to_string())
            .lines()
            .find(|line| line.starts_with("NAME="))
            .map(|line| line.trim_start_matches("NAME=").trim_matches('"').to_string())
            .unwrap_or_else(|| "unknown".to_string())
            .to_lowercase();

        let kernel_version = std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .unwrap_or_else(|_| "unknown kernel".to_string())
            .trim()
            .to_lowercase();

        let desktop_environment = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| "Unknown".to_string())
            .to_lowercase();

        let ram_usage = {
            let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_else(|_| "Unknown RAM".to_string());
            let mut total_memory_kb = 0;
            let mut available_memory_kb = 0;
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    total_memory_kb = line.split_whitespace().nth(1).and_then(|kb| kb.parse::<u64>().ok()).unwrap_or(0);
                }
                if line.starts_with("MemAvailable:") {
                    available_memory_kb = line.split_whitespace().nth(1).and_then(|kb| kb.parse::<u64>().ok()).unwrap_or(0);
                }
            }
            let used_memory_kb = total_memory_kb.saturating_sub(available_memory_kb);
            let used_memory_gb = used_memory_kb as f64 / 1_048_576.0;
            let total_memory_gb = total_memory_kb as f64 / 1_048_576.0;
            format!("{:.2} gb / {:.2} gb", used_memory_gb, total_memory_gb)
        }
        .to_lowercase();

        let package_count = {
            let dpkg_query = std::process::Command::new("dpkg-query")
                .arg("-f")
                .arg("${binary:Package}\n")
                .arg("-W")
                .output();

            match dpkg_query {
                Ok(output) => {
                    let count = String::from_utf8_lossy(&output.stdout);
                    count.lines().count().to_string()
                }
                Err(_) => "0".to_string(),
            }
        }
        .to_lowercase();

        let cpu = {
            let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
            let mut model_name = String::from("unknown cpu");
            let mut cpu_ghz = String::from("unknown ghz");

            for line in cpuinfo.lines() {
                if line.starts_with("model name") && model_name == "unknown cpu" {
                    if let Some(name) = line.splitn(2, ':').nth(1) {
                        let cleaned = name
                            .replace("with Radeon Graphics", "")
                            .replace("with Radeon Vega Graphics", "")
                            .replace("(R)", "")
                            .replace("(TM)", "")
                            .replace("CPU", "")
                            .replace("  ", " ")
                            .trim()
                            .to_string()
                            .to_lowercase();
                        model_name = cleaned;
                    }
                }
                if line.starts_with("cpu MHz") && cpu_ghz == "unknown ghz" {
                    if let Some(mhz) = line.splitn(2, ':').nth(1) {
                        let ghz = mhz.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                        cpu_ghz = format!("{:.2} ghz", ghz);
                    }
                }
            }

            if model_name != "unknown cpu" && cpu_ghz != "unknown ghz" {
                format!("{} @ {}", model_name, cpu_ghz)
            } else {
                model_name
            }
        }
        .to_lowercase();

        let init = {
            // Check what process 1 is
            let comm_path = "/proc/1/comm";
            let exe_path = "/proc/1/exe";
            let init_name = if let Ok(name) = std::fs::read_to_string(comm_path) {
                name.trim().to_lowercase()
            } else if let Ok(path) = std::fs::read_link(exe_path) {
                path.file_name()
                    .map(|n| n.to_string_lossy().to_lowercase())
                    .unwrap_or_else(|| "unknown".to_string())
            } else {
                "unknown".to_string()
            };
            init_name
        };

        SystemInfo {
            os_name,
            kernel_version,
            desktop_environment,
            ram_usage,
            package_count,
            cpu,
            init, 
        }
    }
}

pub fn get_system_info() -> SystemInfo {
    SystemInfo::new()
}