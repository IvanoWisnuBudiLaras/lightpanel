use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppDomain {
    pub id: String,
    pub app_id: String,
    pub domain: String,
    pub is_primary: bool,
    pub created_at: String,
}
