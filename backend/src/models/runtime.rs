use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RuntimeInfo {
    pub name: &'static str,
    pub command: &'static str,
    pub status: RuntimeStatus,
    pub version: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeStatus {
    Installed,
    NotInstalled,
    Error,
}
