use std::collections::HashMap;
use std::fs;
use std::env;

const N_A: &str = "n/a";

pub fn user() -> String {
    match env::var("USER") {
        Ok(s) => s.trim().to_string(),
        Err(_) => N_A.to_string()
    }
}

pub fn host() -> String {
    match fs::read_to_string("/etc/hostname") {
        Ok(s) => s.trim().to_string(),
        Err(_) => N_A.to_string()
    }
}

pub fn os() -> String {
    match fs::read_to_string("/proc/sys/kernel/ostype") {
        Ok(s) => s.trim()
            .to_lowercase()
            .to_string(),
        Err(_) => N_A.to_string()
    }
}

pub fn distro() -> String {
    match fs::read_to_string("/etc/os-release") {
        Ok(s) => s
        .lines()
        .find(|l| l.starts_with("ID"))
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap_or(N_A)
        .to_string(),
        Err(_) => N_A.to_string()
    }
}

pub fn kernel() -> String {
    match fs::read_to_string("/proc/sys/kernel/osrelease") {
        Ok(s) => s.split('-')
            .next()
            .unwrap_or(N_A)
            .to_string(),
        Err(_) => N_A.to_string()
    }
}

pub fn pkgs() -> String {
    match fs::read_dir("/var/lib/pacman/local") {
        Ok(d) => d.count().to_string(),
        Err(_) => N_A.to_string()
    }
}

pub fn mem() -> String {
    match fs::read_to_string("/proc/meminfo") {
        Ok(s) => {
            let map = s
                    .trim()
                    .split("\n")
                    .map(|s| s.split_at(s.find(":").unwrap()))
                    .map(|(k, v)| (k.trim().to_string(), v[1..].trim().to_string()))
                    .collect::<HashMap<String, String>>();
            let total = map
                    .get("MemTotal").unwrap()
                    .split(' ')
                    .next().unwrap().parse::<i32>().unwrap();
            let free = map
                    .get("MemFree").unwrap()
                    .split(' ')
                    .next().unwrap().parse::<i32>().unwrap();
            let buffed = map
                    .get("Buffers").unwrap()
                    .split(' ')
                    .next().unwrap().parse::<i32>().unwrap();
            let cached = map
                    .get("Cached").unwrap()
                    .split(' ')
                    .next().unwrap().parse::<i32>().unwrap();
            let sreclaim = map
                    .get("SReclaimable").unwrap()
                    .split(' ')
                    .next().unwrap().parse::<i32>().unwrap();
            let shmem = map
                    .get("Shmem").unwrap()
                    .split(' ')
                    .next().unwrap().parse::<i32>().unwrap();

            let total_used = total - free;
            let total_cached = cached + sreclaim + shmem;
            let used = total_used - buffed - total_cached;
            let used_mb = used / 1024;
            let percent_used = 100.0 * (used as f32 / total as f32);
            format!("{}mb {:.0}%", used_mb, percent_used)
        },
        Err(_) => N_A.to_string()
    }
}

pub fn uptime() -> String {
    match fs::read_to_string("/proc/uptime") {
        Ok(s) => {
            match s
                .split('.')
                .next()
                .unwrap()
                .parse::<i32>() {
                    Ok(sec) =>  {
                        let hours = sec / 3600;
                        let min = (sec % 3600) / 60;
                        format!("{:.0}h {:.0}m", hours, min)
                    },
                    Err(_) => N_A.to_string()
                }
        },
        Err(_) => N_A.to_string()
    }
}
