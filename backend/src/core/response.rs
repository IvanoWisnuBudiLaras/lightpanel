use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub data: T,
}

pub fn ok<T>(data: T) -> Json<ApiResponse<T>>
where
    T: Serialize,
{
    Json(ApiResponse {
        success: true,
        data,
    })
}
