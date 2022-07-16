use std::collections::HashMap;
use std::fs;
use std::env;
use std::process;

pub fn user() -> String {
    env::var("USER")
        .unwrap_or("n/a".to_string())
        .trim()
        .to_string()
}

pub fn host() -> String {
    match fs::read_to_string("/etc/hostname") {
        Ok(s) => s.trim().to_string(),
        Err(_) => String::from("n/a")
    }
}

pub fn os() -> String {
    match process::Command::new("uname")
        .arg("-o")
        .output() {
        Ok(s) => String::from_utf8(s.stdout)
            .unwrap_or("n/a".to_string())
            .trim()
            .to_string(),
        Err(_) => String::from("n/a")
    }
}

pub fn distro() -> String {
    match fs::read_to_string("/etc/os-release") {
        Ok(s) => s
        .trim()
        .split("\n")
        .map(|s| s.split_at(s.find("=").unwrap()))
        .map(|(k, v)| (k.to_string(), v[1..].to_string()))
        .collect::<HashMap<String, String>>()
        .get("ID").unwrap_or(&String::from("n/a"))
        .trim_matches('"')
        .to_string(),
        Err(_) => String::from("n/a")
    }
}

pub fn kernel() -> String {
    match process::Command::new("uname")
        .arg("-r")
        .output() {
        Ok(s) => String::from_utf8(s.stdout)
            .unwrap_or("n/a".to_string())
            .trim()
            .to_string(),
        Err(_) => String::from("n/a")
    }
}

pub fn pkgs() -> String {
    match process::Command::new("ls")
        .arg("-A")
        .arg("/var/lib/pacman/local")
        .output() {
        Ok(s) => String::from_utf8(s.stdout)
            .unwrap_or("".to_string())
            .lines()
            .count()
            .to_string(),
        Err(_) => String::from("n/a")
    }
}

pub fn mem() -> String {
    match fs::read_to_string("/proc/meminfo") {
        Ok(s) => {
            format!("{}mb", s
                .trim()
                .split("\n")
                .map(|s| s.split_at(s.find(":").unwrap()))
                .map(|(k, v)| (k.trim().to_string(), v[1..].trim().to_string()))
                .collect::<HashMap<String, String>>()
                .get("Active").unwrap_or(&String::from("n/a"))
                .split(' ')
                .next().unwrap().parse::<i32>().unwrap() / 1000)
        },
        Err(_) => String::from("n/a")
    }
}

pub fn uptime() -> String {
    match process::Command::new("uptime")
        .arg("-p")
        .output() {
        Ok(s) => String::from_utf8(s.stdout)
            .unwrap_or("n/a".to_string())
            .trim()
            .split(',')
            .next().unwrap_or("n/a")
            .to_string(),
        Err(_) => String::from("n/a")
    }
}
