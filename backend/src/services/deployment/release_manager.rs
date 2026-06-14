use crate::core::error::AppError;
use crate::services::deployment::app_paths;
use std::{
    fs,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

pub fn create_release_dir(app_name: &str, release_id: &str) -> Result<String, AppError> {
    fs::create_dir_all(app_paths::releases_dir(app_name)?)?;
    let release_dir = app_paths::release_dir(app_name, release_id)?;

    if release_dir.exists() {
        return Err(AppError::Conflict("release already exists".to_string()));
    }

    fs::create_dir_all(&release_dir)?;
    Ok(release_dir.to_string_lossy().to_string())
}

pub fn switch_current(app_name: &str, release_id: &str) -> Result<(), AppError> {
    let current = app_paths::current_link(app_name)?;
    let tmp_current = current.with_extension("next");
    let release_dir = app_paths::release_dir(app_name, release_id)?;

    if !release_dir.is_dir() {
        return Err(AppError::NotFound("release directory not found".to_string()));
    }

    remove_existing_tmp_link(&tmp_current)?;
    symlink(relative_release_target(release_id), &tmp_current)?;
    replace_current_link(&current, &tmp_current)?;

    Ok(())
}

fn relative_release_target(release_id: &str) -> PathBuf {
    PathBuf::from("releases").join(release_id)
}

fn replace_current_link(current: &Path, tmp_current: &Path) -> Result<(), AppError> {
    if current.exists() || current.symlink_metadata().is_ok() {
        let metadata = current.symlink_metadata()?;
        if !metadata.file_type().is_symlink() {
            return Err(AppError::Conflict("current path is not a symlink".to_string()));
        }
        fs::remove_file(current)?;
    }

    fs::rename(tmp_current, current)?;
    Ok(())
}

fn remove_existing_tmp_link(tmp_current: &Path) -> Result<(), AppError> {
    if tmp_current.exists() || tmp_current.symlink_metadata().is_ok() {
        fs::remove_file(tmp_current)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_target_points_to_release() {
        assert_eq!(
            relative_release_target("123").to_string_lossy(),
            "releases/123"
        );
    }
}
