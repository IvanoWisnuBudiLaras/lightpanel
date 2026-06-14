use crate::core::{error::AppError, state::AppState};
use crate::models::app::AppRecord;
use crate::services::{
    deployment::{app_paths, release_manager, start_script, status},
    logs::deploy_log,
};
use crate::utils::{command, fs, path};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::node_package_manager::{NodeCommand, PackageManager};

#[derive(Debug, Deserialize)]
pub struct DeployNodeRequest {
    pub source_path: String,
}

#[derive(Debug, Serialize)]
pub struct NodeDeployResult {
    pub deployment_id: String,
    pub status: String,
    pub release_id: String,
    pub release_path: String,
    pub current_path: String,
    pub deploy_log_path: String,
    pub package_manager: String,
    pub install_command: String,
    pub build_command: String,
    pub start_command: String,
}

pub struct NodeAdapter;

impl NodeAdapter {
    pub fn deploy(
        state: &AppState,
        app: &AppRecord,
        request: DeployNodeRequest,
    ) -> Result<NodeDeployResult, AppError> {
        ensure_node_app(app)?;

        let source_dir = path::validate_source_dir(&request.source_path)?;
        let package_manager = PackageManager::detect(Path::new(&source_dir))?;
        package_manager.ensure_supported_and_installed()?;

        let release_id = status::timestamp_id();
        let release_path = release_manager::create_release_dir(&app.name, &release_id)?;
        let log_path = deploy_log::path_for_app(&app.name)?;
        ensure_source_outside_app_dir(&app.name, &source_dir, &release_path)?;
        ensure_release_workdir(&app.name, &release_path)?;

        deploy_log::append(&log_path, "created node release directory")?;
        fs::copy_dir_contents(&source_dir, &release_path)?;
        deploy_log::append(&log_path, "copied node source files")?;

        let install_command = package_manager.install_command();
        let build_command = package_manager.build_command()?;
        let start_command = package_manager.start_command();

        run_step(&install_command, &release_path, &log_path)?;
        run_step(&build_command, &release_path, &log_path)?;

        fs::write_file(path::app_env_file(&app.name)?, &env_content(app))?;
        fs::write_executable_file(
            format!("{release_path}/start.sh"),
            &start_script::render_node_start_script(app, start_command.program, start_command.args)?,
        )?;
        deploy_log::append(&log_path, "wrote env and node start script")?;

        release_manager::switch_current(&app.name, &release_id)?;
        deploy_log::append(&log_path, "switched current symlink")?;

        let deployment = status::record_success(state, app, &release_path, &log_path)?;
        Ok(NodeDeployResult {
            deployment_id: deployment.id,
            status: deployment.status,
            release_id,
            release_path,
            current_path: path::current_release_dir(&app.name)?,
            deploy_log_path: log_path,
            package_manager: package_manager.name().to_string(),
            install_command: command_label(&install_command),
            build_command: command_label(&build_command),
            start_command: command_label(&start_command),
        })
    }
}

fn run_step(command: &NodeCommand, release_path: &str, log_path: &str) -> Result<(), AppError> {
    deploy_log::append(
        log_path,
        &format!("running: {} {}", command.program, command.args.join(" ")),
    )?;
    let output = command::run_deploy_command(command.program, command.args, Path::new(release_path))?;
    deploy_log::append(log_path, &output.combined())?;

    if output.success {
        Ok(())
    } else {
        Err(AppError::BadRequest(format!("node command failed: {}", output.combined())))
    }
}

fn ensure_node_app(app: &AppRecord) -> Result<(), AppError> {
    if app.runtime == "node" {
        Ok(())
    } else {
        Err(AppError::BadRequest("app runtime must be node".to_string()))
    }
}

fn env_content(app: &AppRecord) -> String {
    format!("LIGHTPANEL_INTERNAL_PORT={}\n", app.internal_port)
}

fn command_label(command: &NodeCommand) -> String {
    format!("{} {}", command.program, command.args.join(" "))
}

fn ensure_release_workdir(app_name: &str, release_path: &str) -> Result<(), AppError> {
    let releases_dir = app_paths::releases_dir(app_name)?;

    if Path::new(release_path).starts_with(releases_dir) {
        Ok(())
    } else {
        Err(AppError::BadRequest(
            "working directory must be inside app releases".to_string(),
        ))
    }
}

fn ensure_source_outside_app_dir(
    app_name: &str,
    source_dir: &str,
    release_path: &str,
) -> Result<(), AppError> {
    let app_dir: PathBuf = app_paths::app_dir(app_name)?;

    if Path::new(source_dir).starts_with(&app_dir) {
        return Err(AppError::BadRequest(
            "source path must be outside the app directory".to_string(),
        ));
    }

    if Path::new(release_path).starts_with(source_dir) {
        return Err(AppError::BadRequest(
            "release path must not be inside source path".to_string(),
        ));
    }

    Ok(())
}
