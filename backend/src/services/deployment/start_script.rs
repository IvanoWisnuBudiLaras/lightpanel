use crate::core::error::AppError;
use crate::models::app::AppRecord;

pub fn render_start_script(app: &AppRecord) -> Result<String, AppError> {
    let command = command_for_runtime(app)?;

    Ok(format!(
        "#!/usr/bin/env sh\n\
         set -eu\n\
         export LIGHTPANEL_INTERNAL_PORT=\"${{LIGHTPANEL_INTERNAL_PORT:-{port}}}\"\n\
         cd \"$(dirname \"$0\")\"\n\
         exec {command}\n",
        port = app.internal_port
    ))
}

pub fn render_node_start_script(
    app: &AppRecord,
    program: &str,
    args: &[&str],
) -> Result<String, AppError> {
    if app.runtime != "node" {
        return Err(AppError::BadRequest("app runtime must be node".to_string()));
    }

    if !is_allowed_node_start(program, args) {
        return Err(AppError::BadRequest(
            "node start command is not allowlisted".to_string(),
        ));
    }

    Ok(format!(
        "#!/usr/bin/env sh\n\
         set -eu\n\
         export LIGHTPANEL_INTERNAL_PORT=\"${{LIGHTPANEL_INTERNAL_PORT:-{port}}}\"\n\
         cd \"$(dirname \"$0\")\"\n\
         exec {program} {args}\n",
        port = app.internal_port,
        args = args.join(" ")
    ))
}

fn is_allowed_node_start(program: &str, args: &[&str]) -> bool {
    (program == "npm" && args == ["run", "start"].as_slice())
        || (program == "pnpm" && args == ["start"].as_slice())
}

fn command_for_runtime(app: &AppRecord) -> Result<&'static str, AppError> {
    match app.runtime.as_str() {
        "static" => Ok("python3 -m http.server \"$LIGHTPANEL_INTERNAL_PORT\""),
        "node" => Ok("npm run start"),
        "php" => Ok("php -S 127.0.0.1:\"$LIGHTPANEL_INTERNAL_PORT\""),
        "python" => Ok("python3 app.py"),
        "rust" => Ok("./app"),
        "go" => Ok("./app"),
        "java" => Ok("java -jar app.jar"),
        "dotnet" => Ok("dotnet app.dll"),
        _ => Err(AppError::BadRequest("unsupported runtime".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_script_uses_internal_port_env() {
        let app = AppRecord {
            id: "1".to_string(),
            name: "demo".to_string(),
            runtime: "static".to_string(),
            internal_port: 8080,
            status: "created".to_string(),
            primary_domain: None,
            created_at: "0".to_string(),
            updated_at: "0".to_string(),
            deleted_at: None,
        };

        let script = render_start_script(&app).unwrap();
        assert!(script.contains("python3 -m http.server"));
        assert!(script.contains("8080"));
    }
}
