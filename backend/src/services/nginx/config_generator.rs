use crate::{
    core::error::AppError,
    models::app::AppRecord,
    services::{apps_validation, nginx::site_config},
};

pub use site_config::NginxPreview;

pub fn preview_for_app(app: &AppRecord) -> Result<NginxPreview, AppError> {
    let domain = app
        .primary_domain
        .as_deref()
        .ok_or_else(|| AppError::BadRequest("app has no primary domain".to_string()))?;

    apps_validation::validate_domain(Some(domain))?;

    let file_name = site_config::file_name(&app.name);
    Ok(NginxPreview {
        app_name: app.name.clone(),
        domain: domain.to_string(),
        internal_port: app.internal_port,
        available_path: format!("{}/{}", site_config::SITES_AVAILABLE, file_name),
        enabled_path: format!("{}/{}", site_config::SITES_ENABLED, file_name),
        config: render_config(domain, app.internal_port),
    })
}

fn render_config(domain: &str, internal_port: u16) -> String {
    format!(
        "server {{\n\
         \tlisten 80;\n\
         \tlisten [::]:80;\n\
         \tserver_name {domain};\n\n\
         \taccess_log /var/log/nginx/{domain}.access.log;\n\
         \terror_log /var/log/nginx/{domain}.error.log;\n\n\
         \tlocation / {{\n\
         \t\tproxy_pass http://127.0.0.1:{internal_port};\n\
         \t\tproxy_http_version 1.1;\n\
         \t\tproxy_set_header Host $host;\n\
         \t\tproxy_set_header X-Real-IP $remote_addr;\n\
         \t\tproxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n\
         \t\tproxy_set_header X-Forwarded-Proto $scheme;\n\
         \t}}\n\
         }}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_proxies_to_internal_port() {
        let config = render_config("example.com", 3001);
        assert!(config.contains("server_name example.com;"));
        assert!(config.contains("proxy_pass http://127.0.0.1:3001;"));
    }
}
