
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// Absolute default; should cover Debian, Ubuntu and RHEL/Centos variants.
const DEFAULT_GROUPS: &str = "sudo:wheel:admin";

fn parse_os_release() -> Option<HashMap<String, String>> {
    let path = PathBuf::from("/etc/os-release");
    if !path.is_file() {
        return None;
    }

    let fd = File::open(path).ok()?;
    let mut map = HashMap::new();
    for l in BufReader::new(fd).lines() {
        if let Ok(line) = l {
            let s: Vec<String> = line
                .trim()
                .split("=")
                .map(String::from)
                .collect();
            map.insert(s[0].clone(), s[1].clone());
        }
    }

    Some(map)
}


fn get_os_priv_groups() -> Option<String> {
    let osr = parse_os_release()?;
    let id: &str = osr.get("ID")
        .map(|v| v.as_str())?;
    Some(match id {
        "debian" => "sudo",
        "ubuntu" => "sudo:admin",
        "amzn" => "wheel",
        "fedora" | "rhel" => "wheel",
        _ => DEFAULT_GROUPS,
    }.to_string())
}


fn main() {
    // Allow override with existing var.
    if option_env!("DAU_PRIV_GROUPS").is_some() {
        return;
    }

    println!("cargo:rustc-env=DAU_PRIV_GROUPS={}", get_os_priv_groups().unwrap_or(DEFAULT_GROUPS.to_string()));
}
