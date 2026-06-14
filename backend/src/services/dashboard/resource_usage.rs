use crate::{core::error::AppError, utils::command};
use serde::Serialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Serialize)]
pub struct ResourceUsage {
    pub uptime_seconds: u64,
    pub uptime_label: String,
    pub load: LoadAverage,
    pub memory: MemoryUsage,
    pub disk: DiskUsage,
}

#[derive(Debug, Serialize)]
pub struct LoadAverage {
    pub one: String,
    pub five: String,
    pub fifteen: String,
}

#[derive(Debug, Serialize)]
pub struct MemoryUsage {
    pub total_mb: u64,
    pub used_mb: u64,
    pub available_mb: u64,
    pub used_percent: u8,
}

#[derive(Debug, Serialize)]
pub struct DiskUsage {
    pub filesystem: String,
    pub mount: String,
    pub total_gb: f64,
    pub used_gb: f64,
    pub available_gb: f64,
    pub used_percent: u8,
}

pub fn read_resources() -> Result<ResourceUsage, AppError> {
    let uptime_seconds = read_uptime_seconds()?;

    Ok(ResourceUsage {
        uptime_seconds,
        uptime_label: format_uptime(uptime_seconds),
        load: read_load()?,
        memory: read_memory()?,
        disk: read_disk()?,
    })
}

fn read_uptime_seconds() -> Result<u64, AppError> {
    let content = fs::read_to_string("/proc/uptime")?;
    let first = content.split_whitespace().next().unwrap_or("0");
    Ok(first.split('.').next().unwrap_or("0").parse().unwrap_or(0))
}

fn read_load() -> Result<LoadAverage, AppError> {
    let content = fs::read_to_string("/proc/loadavg")?;
    let mut parts = content.split_whitespace();
    Ok(LoadAverage {
        one: parts.next().unwrap_or("0.00").to_string(),
        five: parts.next().unwrap_or("0.00").to_string(),
        fifteen: parts.next().unwrap_or("0.00").to_string(),
    })
}

fn read_memory() -> Result<MemoryUsage, AppError> {
    let values = parse_meminfo(&fs::read_to_string("/proc/meminfo")?);
    let total = values.get("MemTotal").copied().unwrap_or(0);
    let available = values.get("MemAvailable").copied().unwrap_or(0);
    let used = total.saturating_sub(available);

    Ok(MemoryUsage {
        total_mb: kb_to_mb(total),
        used_mb: kb_to_mb(used),
        available_mb: kb_to_mb(available),
        used_percent: percent(used, total),
    })
}

fn read_disk() -> Result<DiskUsage, AppError> {
    let output = command::run_read_only("df", &["-P", "/"])
        .ok_or_else(|| AppError::Internal("df command is not available".to_string()))?;
    let line = output.stdout.lines().nth(1).unwrap_or("");
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 6 {
        return Err(AppError::Internal("unable to parse disk usage".to_string()));
    }

    let total = parts[1].parse::<u64>().unwrap_or(0);
    let used = parts[2].parse::<u64>().unwrap_or(0);
    let available = parts[3].parse::<u64>().unwrap_or(0);

    Ok(DiskUsage {
        filesystem: parts[0].to_string(),
        mount: parts[5].to_string(),
        total_gb: kb_to_gb(total),
        used_gb: kb_to_gb(used),
        available_gb: kb_to_gb(available),
        used_percent: percent(used, total),
    })
}

fn parse_meminfo(content: &str) -> HashMap<&str, u64> {
    content
        .lines()
        .filter_map(|line| {
            let (key, rest) = line.split_once(':')?;
            let value = rest.split_whitespace().next()?.parse().ok()?;
            Some((key, value))
        })
        .collect()
}

fn kb_to_mb(kb: u64) -> u64 {
    kb / 1024
}

fn kb_to_gb(kb: u64) -> f64 {
    ((kb as f64 / 1024.0 / 1024.0) * 10.0).round() / 10.0
}

fn percent(used: u64, total: u64) -> u8 {
    if total == 0 {
        0
    } else {
        ((used.saturating_mul(100)) / total).min(100) as u8
    }
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let minutes = (seconds % 3_600) / 60;
    format!("{days}d {hours}h {minutes}m")
}
