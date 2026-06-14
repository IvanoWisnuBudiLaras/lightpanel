use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Deployment {
    pub id: String,
    pub app_id: String,
    pub status: String,
    pub release_path: Option<String>,
    pub log_path: Option<String>,
    pub created_at: String,
}
