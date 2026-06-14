use crate::core::error::AppError;
use std::{
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const COMMAND_TIMEOUT: Duration = Duration::from_secs(2);
const POLL_INTERVAL: Duration = Duration::from_millis(25);

pub struct CommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    pub fn combined(&self) -> String {
        if self.stdout.is_empty() {
            self.stderr.clone()
        } else {
            self.stdout.clone()
        }
    }
}

pub fn run_read_only(program: &str, args: &[&str]) -> Option<CommandOutput> {
    if !is_allowlisted(program, args) {
        return None;
    }

    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    let started_at = Instant::now();

    loop {
        if child.try_wait().ok()?.is_some() {
            let output = child.wait_with_output().ok()?;
            return Some(CommandOutput {
                success: output.status.success(),
                stdout: clean_output(output.stdout),
                stderr: clean_output(output.stderr),
            });
        }

        if started_at.elapsed() >= COMMAND_TIMEOUT {
            let _ = child.kill();
            let output = child.wait_with_output().ok()?;
            return Some(CommandOutput {
                success: false,
                stdout: clean_output(output.stdout),
                stderr: "command timed out".to_string(),
            });
        }

        thread::sleep(POLL_INTERVAL);
    }
}

pub fn run_deploy_command(
    program: &str,
    args: &[&str],
    working_dir: &Path,
) -> Result<CommandOutput, AppError> {
    if !is_deploy_allowlisted(program, args) {
        return Err(AppError::BadRequest("command is not allowlisted".to_string()));
    }

    let output = Command::new(program)
        .args(args)
        .current_dir(working_dir)
        .output()
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                AppError::BadRequest(format!("{program} is not installed"))
            } else {
                AppError::from(error)
            }
        })?;

    Ok(CommandOutput {
        success: output.status.success(),
        stdout: clean_output(output.stdout),
        stderr: clean_output(output.stderr),
    })
}

pub fn is_program_installed(program: &str) -> bool {
    run_read_only(program, &["--version"])
        .map(|output| output.success)
        .unwrap_or(false)
}

fn is_allowlisted(program: &str, args: &[&str]) -> bool {
    const ALLOWLIST: &[(&str, &[&str])] = &[
        ("php", &["-v"]),
        ("node", &["-v"]),
        ("python3", &["--version"]),
        ("go", &["version"]),
        ("cargo", &["--version"]),
        ("rustc", &["--version"]),
        ("java", &["-version"]),
        ("dotnet", &["--version"]),
        ("npm", &["--version"]),
        ("pnpm", &["--version"]),
        ("yarn", &["--version"]),
        ("uname", &["-r"]),
        ("uname", &["-m"]),
        ("df", &["-P", "/"]),
    ];

    ALLOWLIST
        .iter()
        .any(|(allowed_program, allowed_args)| {
            program == *allowed_program && args == *allowed_args
        })
}

fn is_deploy_allowlisted(program: &str, args: &[&str]) -> bool {
    const ALLOWLIST: &[(&str, &[&str])] = &[
        ("npm", &["install"]),
        ("npm", &["run", "build"]),
        ("npm", &["run", "start"]),
        ("pnpm", &["install"]),
        ("pnpm", &["build"]),
        ("pnpm", &["start"]),
    ];

    ALLOWLIST
        .iter()
        .any(|(allowed_program, allowed_args)| {
            program == *allowed_program && args == *allowed_args
        })
}

fn clean_output(bytes: Vec<u8>) -> String {
    String::from_utf8_lossy(&bytes).trim().to_string()
}
