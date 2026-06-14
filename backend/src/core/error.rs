use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Conflict(String),
    Internal(String),
    NotFound(String),
}

#[derive(Serialize)]
struct ErrorBody {
    success: bool,
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "bad_request",
            Self::Conflict(_) => "conflict",
            Self::Internal(_) => "internal_error",
            Self::NotFound(_) => "not_found",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::BadRequest(message) => message.clone(),
            Self::Conflict(message) => message.clone(),
            Self::Internal(message) => message.clone(),
            Self::NotFound(message) => message.clone(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = ErrorBody {
            success: false,
            error: ErrorDetail {
                code: self.code(),
                message: self.message(),
            },
        };

        (status, Json(body)).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::Internal(error.to_string())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Internal(error.to_string())
    }
}
