use crate::core::error::AppError;
use serde::Serialize;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const NGINX_ERROR_LOG: &str = "/var/log/nginx/error.log";
const PANEL_DEPLOY_LOG: &str = "/opt/lightpanel/deploy.log";

#[derive(Debug, Serialize)]
pub struct LogSnapshot {
    pub source: String,
    pub lines: Vec<String>,
    pub warning: Option<String>,
}

pub fn read_nginx_error_log(max_lines: usize) -> Result<LogSnapshot, AppError> {
    tail_file(NGINX_ERROR_LOG, max_lines)
}

pub fn read_panel_deploy_log(max_lines: usize) -> Result<LogSnapshot, AppError> {
    tail_file(PANEL_DEPLOY_LOG, max_lines)
}

pub fn tail_file(path: impl AsRef<Path>, max_lines: usize) -> Result<LogSnapshot, AppError> {
    let path = path.as_ref();
    let limit = max_lines.clamp(1, 500);

    if !path.exists() {
        return Ok(LogSnapshot {
            source: path.to_string_lossy().to_string(),
            lines: Vec::new(),
            warning: Some("log file does not exist yet".to_string()),
        });
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = VecDeque::with_capacity(limit);

    for line in reader.lines() {
        let line = line?;
        if lines.len() == limit {
            lines.pop_front();
        }
        lines.push_back(line);
    }

    Ok(LogSnapshot {
        source: path.to_string_lossy().to_string(),
        lines: lines.into_iter().collect(),
        warning: None,
    })
}
