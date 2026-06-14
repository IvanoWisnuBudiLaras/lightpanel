use crate::core::error::AppError;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn validate_app_input(
    name: &str,
    runtime: &str,
    internal_port: u16,
) -> Result<(), AppError> {
    if !is_valid_app_name(name) {
        return Err(AppError::BadRequest(
            "app name must use lowercase letters, numbers, and dash only".to_string(),
        ));
    }

    if !is_valid_runtime(runtime) {
        return Err(AppError::BadRequest("unsupported runtime".to_string()));
    }

    if internal_port == 0 {
        return Err(AppError::BadRequest(
            "internal port must be between 1 and 65535".to_string(),
        ));
    }

    Ok(())
}

pub fn now_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

pub fn validate_domain(domain: Option<&str>) -> Result<(), AppError> {
    let Some(domain) = domain.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(());
    };

    if is_valid_domain(domain) {
        return Ok(());
    }

    Err(AppError::BadRequest(
        "domain must be a valid hostname".to_string(),
    ))
}

fn is_valid_app_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn is_valid_domain(domain: &str) -> bool {
    domain.len() <= 253
        && domain.contains('.')
        && domain
            .split('.')
            .all(|label| is_valid_domain_label(label))
}

fn is_valid_domain_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 63
        && !label.starts_with('-')
        && !label.ends_with('-')
        && label
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn is_valid_runtime(runtime: &str) -> bool {
    matches!(
        runtime,
        "static" | "node" | "php" | "python" | "rust" | "go" | "java" | "dotnet"
    )
}
