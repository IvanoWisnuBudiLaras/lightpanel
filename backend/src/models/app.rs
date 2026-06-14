use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AppRecord {
    pub id: String,
    pub name: String,
    pub runtime: String,
    pub internal_port: u16,
    pub status: String,
    pub primary_domain: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
    pub runtime: String,
    pub internal_port: u16,
    pub primary_domain: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAppRequest {
    pub name: String,
    pub runtime: String,
    pub internal_port: u16,
    pub primary_domain: Option<String>,
}
