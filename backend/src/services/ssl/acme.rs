use crate::{
    core::error::AppError,
    models::app::AppRecord,
    services::apps_validation,
    utils::path,
};
use serde::Serialize;

const ACME_PATH: &str = "/root/.acme.sh/acme.sh";
const SSL_BASE: &str = "/opt/lightpanel/data/ssl";

#[derive(Debug, Serialize)]
pub struct SslPreview {
    pub app_name: String,
    pub domain: String,
    pub mode: String,
    pub webroot: String,
    pub certificate_dir: String,
    pub commands: Vec<CommandPreview>,
}

#[derive(Debug, Serialize)]
pub struct CommandPreview {
    pub program: String,
    pub args: Vec<String>,
}

pub fn preview_for_app(app: &AppRecord) -> Result<SslPreview, AppError> {
    let domain = app
        .primary_domain
        .as_deref()
        .ok_or_else(|| AppError::BadRequest("app has no primary domain".to_string()))?;

    apps_validation::validate_domain(Some(domain))?;

    let webroot = path::current_release_dir(&app.name)?;
    let certificate_dir = format!("{SSL_BASE}/{domain}");

    Ok(SslPreview {
        app_name: app.name.clone(),
        domain: domain.to_string(),
        mode: "preview_only".to_string(),
        webroot: webroot.clone(),
        certificate_dir: certificate_dir.clone(),
        commands: vec![
            issue_command(domain, &webroot),
            install_command(domain, &certificate_dir),
        ],
    })
}

fn issue_command(domain: &str, webroot: &str) -> CommandPreview {
    CommandPreview {
        program: ACME_PATH.to_string(),
        args: vec![
            "--issue".to_string(),
            "-d".to_string(),
            domain.to_string(),
            "-w".to_string(),
            webroot.to_string(),
        ],
    }
}

fn install_command(domain: &str, cert_dir: &str) -> CommandPreview {
    CommandPreview {
        program: ACME_PATH.to_string(),
        args: vec![
            "--install-cert".to_string(),
            "-d".to_string(),
            domain.to_string(),
            "--key-file".to_string(),
            format!("{cert_dir}/privkey.key"),
            "--fullchain-file".to_string(),
            format!("{cert_dir}/fullchain.cer"),
            "--reloadcmd".to_string(),
            "systemctl reload nginx".to_string(),
        ],
    }
}
