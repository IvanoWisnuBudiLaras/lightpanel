use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ServerIdentity {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel: String,
    pub architecture: String,
}
