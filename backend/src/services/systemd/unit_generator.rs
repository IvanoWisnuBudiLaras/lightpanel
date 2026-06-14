use crate::core::error::AppError;
use crate::models::app::AppRecord;
use crate::services::deployment::start_script;
use crate::utils::path;
use serde::Serialize;

use super::unit::AppUnit;

#[derive(Debug, Serialize)]
pub struct SystemdPreview {
    pub service_name: String,
    pub unit_file: String,
    pub start_script_path: String,
    pub start_script: String,
}

pub fn preview_for_app(app: &AppRecord) -> Result<SystemdPreview, AppError> {
    validate_app_name(&app.name)?;

    let working_directory = path::current_release_dir(&app.name)?;
    let environment_file = path::app_env_file(&app.name)?;
    let start_script_path = path::start_script_path(&app.name)?;

    let unit = AppUnit {
        service_name: service_name(&app.name),
        description: format!("LightPanel app {}", app.name),
        working_directory,
        environment_file,
        exec_start: start_script_path.clone(),
    };

    Ok(SystemdPreview {
        service_name: unit.service_name.clone(),
        unit_file: render_unit(&unit),
        start_script_path,
        start_script: start_script::render_start_script(app)?,
    })
}

pub fn service_name(app_name: &str) -> String {
    format!("lightpanel-app-{app_name}.service")
}

fn render_unit(unit: &AppUnit) -> String {
    format!(
        "[Unit]\n\
         Description={description}\n\
         After=network.target\n\n\
         [Service]\n\
         Type=simple\n\
         WorkingDirectory={working_directory}\n\
         EnvironmentFile={environment_file}\n\
         ExecStart={exec_start}\n\
         Restart=on-failure\n\
         RestartSec=5\n\
         NoNewPrivileges=true\n\
         PrivateTmp=true\n\n\
         [Install]\n\
         WantedBy=multi-user.target\n",
        description = &unit.description,
        working_directory = &unit.working_directory,
        environment_file = &unit.environment_file,
        exec_start = &unit.exec_start,
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_name_uses_lightpanel_prefix() {
        assert_eq!(
            service_name("demo-app"),
            "lightpanel-app-demo-app.service"
        );
    }
}
