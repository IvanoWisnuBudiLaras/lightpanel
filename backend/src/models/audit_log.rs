use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuditLog {
    pub id: String,
    pub actor: String,
    pub action: String,
    pub target: String,
    pub details: Option<String>,
    pub created_at: String,
}
