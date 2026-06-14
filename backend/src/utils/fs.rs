use crate::core::error::AppError;
use std::{
    fs::{self, Permissions},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

pub fn copy_dir_contents(source: &str, destination: &str) -> Result<(), AppError> {
    let source = Path::new(source);
    let destination = Path::new(destination);

    copy_dir_paths(source, destination)
}

fn copy_dir_paths(source: &Path, destination: &Path) -> Result<(), AppError> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        copy_entry(&entry.path(), &target)?;
    }

    Ok(())
}

pub fn write_file(path: String, content: &str) -> Result<(), AppError> {
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;
    Ok(())
}

pub fn write_executable_file(path: String, content: &str) -> Result<(), AppError> {
    let path = PathBuf::from(path);
    write_file(path.to_string_lossy().to_string(), content)?;
    fs::set_permissions(path, Permissions::from_mode(0o755))?;
    Ok(())
}

fn copy_entry(source: &Path, destination: &Path) -> Result<(), AppError> {
    let metadata = fs::symlink_metadata(source)?;

    if metadata.file_type().is_symlink() {
        return Err(AppError::BadRequest("source symlinks are not allowed".to_string()));
    }

    if metadata.is_dir() {
        fs::create_dir_all(destination)?;
        copy_dir_paths(source, destination)
    } else if metadata.is_file() {
        fs::copy(source, destination)?;
        Ok(())
    } else {
        Err(AppError::BadRequest("unsupported source file type".to_string()))
    }
}
