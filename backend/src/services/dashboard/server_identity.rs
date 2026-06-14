use crate::core::error::AppError;
use crate::models::server_identity::ServerIdentity;
use crate::utils::command;
use std::{collections::HashMap, fs};

pub type Identity = ServerIdentity;

pub fn read_identity() -> Result<Identity, AppError> {
    let os_release = read_os_release();

    Ok(ServerIdentity {
        hostname: read_hostname(),
        os_name: value_or_unknown(&os_release, "PRETTY_NAME"),
        os_version: value_or_unknown(&os_release, "VERSION_ID"),
        kernel: command::run_read_only("uname", &["-r"])
            .map(|output| output.stdout)
            .unwrap_or_else(|| "unknown".to_string()),
        architecture: command::run_read_only("uname", &["-m"])
            .map(|output| output.stdout)
            .unwrap_or_else(|| "unknown".to_string()),
    })
}

fn read_hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

fn read_os_release() -> HashMap<String, String> {
    let Ok(contents) = fs::read_to_string("/etc/os-release") else {
        return HashMap::new();
    };

    contents
        .lines()
        .filter_map(parse_os_release_line)
        .collect()
}

fn parse_os_release_line(line: &str) -> Option<(String, String)> {
    let (key, value) = line.split_once('=')?;
    Some((key.to_string(), value.trim_matches('"').to_string()))
}

fn value_or_unknown(values: &HashMap<String, String>, key: &str) -> String {
    values
        .get(key)
        .cloned()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}
