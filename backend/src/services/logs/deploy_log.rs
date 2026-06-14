use crate::core::error::AppError;
use crate::services::deployment::app_paths;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use super::journal::{self, LogSnapshot};

pub fn path_for_app(app_name: &str) -> Result<String, AppError> {
    Ok(app_paths::deploy_log_path(app_name)?
        .to_string_lossy()
        .to_string())
}

pub fn append(path: &str, message: &str) -> Result<(), AppError> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{message}")?;
    Ok(())
}

pub fn read_for_app(app_name: &str, max_lines: usize) -> Result<LogSnapshot, AppError> {
    journal::tail_file(app_paths::deploy_log_path(app_name)?, max_lines)
}
