use crate::models::runtime::{RuntimeInfo, RuntimeStatus};
use crate::utils::command;

struct RuntimeProbe {
    name: &'static str,
    command: &'static str,
    args: &'static [&'static str],
}

pub fn detect_runtimes() -> Vec<RuntimeInfo> {
    probes().iter().map(detect_runtime).collect()
}

fn detect_runtime(probe: &RuntimeProbe) -> RuntimeInfo {
    match command::run_read_only(probe.command, probe.args) {
        Some(output) if output.success => RuntimeInfo {
            name: probe.name,
            command: probe.command,
            status: RuntimeStatus::Installed,
            version: Some(first_line(output.combined())),
        },
        Some(output) => RuntimeInfo {
            name: probe.name,
            command: probe.command,
            status: RuntimeStatus::Error,
            version: Some(first_line(output.combined())),
        },
        None => RuntimeInfo {
            name: probe.name,
            command: probe.command,
            status: RuntimeStatus::NotInstalled,
            version: None,
        },
    }
}

fn probes() -> &'static [RuntimeProbe] {
    &[
        RuntimeProbe {
            name: "PHP",
            command: "php",
            args: &["-v"],
        },
        RuntimeProbe {
            name: "Node.js",
            command: "node",
            args: &["-v"],
        },
        RuntimeProbe {
            name: "Python",
            command: "python3",
            args: &["--version"],
        },
        RuntimeProbe {
            name: "Go",
            command: "go",
            args: &["version"],
        },
        RuntimeProbe {
            name: "Cargo",
            command: "cargo",
            args: &["--version"],
        },
        RuntimeProbe {
            name: "Rust",
            command: "rustc",
            args: &["--version"],
        },
        RuntimeProbe {
            name: "Java",
            command: "java",
            args: &["-version"],
        },
        RuntimeProbe {
            name: ".NET",
            command: "dotnet",
            args: &["--version"],
        },
    ]
}

fn first_line(value: String) -> String {
    value.lines().next().unwrap_or("").trim().to_string()
}
