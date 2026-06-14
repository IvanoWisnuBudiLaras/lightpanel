use serde::Serialize;

pub const SITES_AVAILABLE: &str = "/etc/nginx/sites-available";
pub const SITES_ENABLED: &str = "/etc/nginx/sites-enabled";

#[derive(Debug, Serialize)]
pub struct NginxPreview {
    pub app_name: String,
    pub domain: String,
    pub internal_port: u16,
    pub available_path: String,
    pub enabled_path: String,
    pub config: String,
}

pub fn file_name(app_name: &str) -> String {
    format!("lightpanel-{app_name}.conf")
}
