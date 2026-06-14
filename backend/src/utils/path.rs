use crate::core::error::AppError;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

const APPS_BASE: &str = "/opt/lightpanel/data/apps";

pub fn current_release_dir(app_name: &str) -> Result<String, AppError> {
    safe_app_path(app_name, &["current"])
}

pub fn app_dir(app_name: &str) -> Result<PathBuf, AppError> {
    safe_app_path_buf(app_name, &[])
}

pub fn releases_dir(app_name: &str) -> Result<PathBuf, AppError> {
    safe_app_path_buf(app_name, &["releases"])
}

pub fn release_dir(app_name: &str, release_id: &str) -> Result<PathBuf, AppError> {
    validate_release_id(release_id)?;
    safe_app_path_buf(app_name, &["releases", release_id])
}

pub fn current_link(app_name: &str) -> Result<PathBuf, AppError> {
    safe_app_path_buf(app_name, &["current"])
}

pub fn app_env_file(app_name: &str) -> Result<String, AppError> {
    safe_app_path(app_name, &[".env"])
}

pub fn deploy_log_path(app_name: &str) -> Result<PathBuf, AppError> {
    safe_app_path_buf(app_name, &["deploy.log"])
}

pub fn start_script_path(app_name: &str) -> Result<String, AppError> {
    safe_app_path(app_name, &["current", "start.sh"])
}

pub fn validate_source_dir(source_path: &str) -> Result<String, AppError> {
    if source_path.trim().is_empty() {
        return Err(AppError::BadRequest("source path is required".to_string()));
    }

    let path = Path::new(source_path);
    if has_unsafe_component(path) {
        return Err(AppError::BadRequest("unsafe source path".to_string()));
    }

    if fs::symlink_metadata(path)?.file_type().is_symlink() {
        return Err(AppError::BadRequest("source path must be a directory".to_string()));
    }

    let canonical = path.canonicalize()?;
    if !canonical.is_dir() {
        return Err(AppError::BadRequest("source path must be a directory".to_string()));
    }

    Ok(canonical.to_string_lossy().to_string())
}

fn safe_app_path(app_name: &str, segments: &[&str]) -> Result<String, AppError> {
    Ok(safe_app_path_buf(app_name, segments)?
        .to_string_lossy()
        .to_string())
}

fn safe_app_path_buf(app_name: &str, segments: &[&str]) -> Result<PathBuf, AppError> {
    validate_app_name(app_name)?;

    let mut path = PathBuf::from(APPS_BASE);
    path.push(app_name);

    for segment in segments {
        path.push(segment);
    }

    ensure_inside_base(&path)?;
    Ok(path)
}

fn validate_app_name(app_name: &str) -> Result<(), AppError> {
    let valid = !app_name.is_empty()
        && app_name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');

    if valid {
        Ok(())
    } else {
        Err(AppError::BadRequest("invalid app name".to_string()))
    }
}

fn validate_release_id(release_id: &str) -> Result<(), AppError> {
    let valid = !release_id.is_empty() && release_id.bytes().all(|byte| byte.is_ascii_digit());

    if valid {
        Ok(())
    } else {
        Err(AppError::BadRequest("invalid release id".to_string()))
    }
}

fn ensure_inside_base(path: &Path) -> Result<(), AppError> {
    let base = Path::new(APPS_BASE);

    if has_unsafe_component(path) || !path.starts_with(base) {
        return Err(AppError::BadRequest("unsafe app path".to_string()));
    }

    Ok(())
}

fn has_unsafe_component(path: &Path) -> bool {
    path.components()
        .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_current_release_path() {
        let path = current_release_dir("demo").unwrap();
        assert_eq!(path, "/opt/lightpanel/data/apps/demo/current");
    }

    #[test]
    fn rejects_traversal_name() {
        assert!(current_release_dir("../demo").is_err());
    }
}
