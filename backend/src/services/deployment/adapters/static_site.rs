use crate::core::{error::AppError, state::AppState};
use crate::models::app::AppRecord;
use crate::services::{
    deployment::{app_paths, release_manager, start_script, status},
    logs::deploy_log,
};
use crate::utils::{fs, path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DeployStaticRequest {
    pub source_path: String,
}

#[derive(Debug, Serialize)]
pub struct StaticDeployResult {
    pub deployment_id: String,
    pub status: String,
    pub release_id: String,
    pub release_path: String,
    pub current_path: String,
    pub deploy_log_path: String,
}

pub struct StaticSiteAdapter;

impl StaticSiteAdapter {
    pub fn deploy(
        state: &AppState,
        app: &AppRecord,
        request: DeployStaticRequest,
    ) -> Result<StaticDeployResult, AppError> {
        ensure_static_app(app)?;

        let source_dir = path::validate_source_dir(&request.source_path)?;
        let release_id = status::timestamp_id();
        let release_path = release_manager::create_release_dir(&app.name, &release_id)?;
        let deploy_log_path = deploy_log::path_for_app(&app.name)?;

        ensure_source_outside_app_dir(&app.name, &source_dir, &release_path)?;
        deploy_log::append(&deploy_log_path, "created release directory")?;
        fs::copy_dir_contents(&source_dir, &release_path)?;
        deploy_log::append(&deploy_log_path, "copied static source files")?;

        fs::write_file(path::app_env_file(&app.name)?, &env_content(app))?;
        fs::write_executable_file(
            format!("{release_path}/start.sh"),
            &start_script::render_start_script(app)?,
        )?;
        deploy_log::append(&deploy_log_path, "wrote env and start script")?;

        release_manager::switch_current(&app.name, &release_id)?;
        deploy_log::append(&deploy_log_path, "switched current symlink")?;

        let deployment = status::record_success(state, app, &release_path, &deploy_log_path)?;

        Ok(StaticDeployResult {
            deployment_id: deployment.id,
            status: deployment.status,
            release_id,
            release_path,
            current_path: path::current_release_dir(&app.name)?,
            deploy_log_path,
        })
    }
}

fn ensure_static_app(app: &AppRecord) -> Result<(), AppError> {
    if app.runtime == "static" {
        Ok(())
    } else {
        Err(AppError::BadRequest("app runtime must be static".to_string()))
    }
}

fn env_content(app: &AppRecord) -> String {
    format!("LIGHTPANEL_INTERNAL_PORT={}\n", app.internal_port)
}

fn ensure_source_outside_app_dir(
    app_name: &str,
    source_dir: &str,
    release_path: &str,
) -> Result<(), AppError> {
    let app_dir = app_paths::app_dir(app_name)?;

    if std::path::Path::new(source_dir).starts_with(&app_dir) {
        return Err(AppError::BadRequest(
            "source path must be outside the app directory".to_string(),
        ));
    }

    if std::path::Path::new(release_path).starts_with(source_dir) {
        return Err(AppError::BadRequest(
            "release path must not be inside source path".to_string(),
        ));
    }

    Ok(())
}
