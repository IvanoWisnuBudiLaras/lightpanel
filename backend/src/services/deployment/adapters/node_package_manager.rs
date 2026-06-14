use crate::core::error::AppError;
use crate::utils::command;
use std::path::Path;

pub struct NodeCommand {
    pub program: &'static str,
    pub args: &'static [&'static str],
}

#[derive(Clone, Copy)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManager {
    pub fn detect(source_dir: &Path) -> Result<Self, AppError> {
        if source_dir.join("package-lock.json").is_file() {
            return Ok(Self::Npm);
        }
        if source_dir.join("pnpm-lock.yaml").is_file() {
            return Ok(Self::Pnpm);
        }
        if source_dir.join("yarn.lock").is_file() {
            return Ok(Self::Yarn);
        }

        Err(AppError::BadRequest("node lockfile not found".to_string()))
    }

    pub fn ensure_supported_and_installed(self) -> Result<(), AppError> {
        if !command::is_program_installed(self.binary()) {
            return Err(AppError::BadRequest(format!(
                "{} is not installed",
                self.binary()
            )));
        }

        if matches!(self, Self::Yarn) {
            return Err(AppError::BadRequest(
                "yarn.lock detected, but yarn deploy commands are not allowlisted yet".to_string(),
            ));
        }

        Ok(())
    }

    pub fn install_command(self) -> NodeCommand {
        match self {
            Self::Npm => NodeCommand {
                program: "npm",
                args: &["install"],
            },
            Self::Pnpm => NodeCommand {
                program: "pnpm",
                args: &["install"],
            },
            Self::Yarn => unreachable!(),
        }
    }

    pub fn build_command(self) -> Result<NodeCommand, AppError> {
        match self {
            Self::Npm => Ok(NodeCommand {
                program: "npm",
                args: &["run", "build"],
            }),
            Self::Pnpm => Ok(NodeCommand {
                program: "pnpm",
                args: &["build"],
            }),
            Self::Yarn => Err(AppError::BadRequest("yarn build is not allowlisted".to_string())),
        }
    }

    pub fn start_command(self) -> NodeCommand {
        match self {
            Self::Npm => NodeCommand {
                program: "npm",
                args: &["run", "start"],
            },
            Self::Pnpm => NodeCommand {
                program: "pnpm",
                args: &["start"],
            },
            Self::Yarn => unreachable!(),
        }
    }

    pub fn name(self) -> &'static str {
        self.binary()
    }

    fn binary(self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pnpm => "pnpm",
            Self::Yarn => "yarn",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npm_commands_are_allowlisted_defaults() {
        let install = PackageManager::Npm.install_command();
        let start = PackageManager::Npm.start_command();

        assert_eq!(install.program, "npm");
        assert_eq!(install.args, &["install"]);
        assert_eq!(start.args, &["run", "start"]);
    }

    #[test]
    fn pnpm_commands_are_allowlisted_defaults() {
        let build = PackageManager::Pnpm.build_command().unwrap();

        assert_eq!(build.program, "pnpm");
        assert_eq!(build.args, &["build"]);
    }
}
