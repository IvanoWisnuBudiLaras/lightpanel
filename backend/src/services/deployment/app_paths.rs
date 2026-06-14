use crate::core::error::AppError;
use crate::utils::path;
use std::path::PathBuf;

pub fn app_dir(app_name: &str) -> Result<PathBuf, AppError> {
    path::app_dir(app_name)
}

pub fn releases_dir(app_name: &str) -> Result<PathBuf, AppError> {
    path::releases_dir(app_name)
}

pub fn release_dir(app_name: &str, release_id: &str) -> Result<PathBuf, AppError> {
    path::release_dir(app_name, release_id)
}

pub fn current_link(app_name: &str) -> Result<PathBuf, AppError> {
    path::current_link(app_name)
}

pub fn deploy_log_path(app_name: &str) -> Result<PathBuf, AppError> {
    path::deploy_log_path(app_name)
}
