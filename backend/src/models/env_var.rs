use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppEnvVar {
    pub id: String,
    pub app_id: String,
    pub key: String,
    pub value: String,
    pub created_at: String,
    pub updated_at: String,
}
