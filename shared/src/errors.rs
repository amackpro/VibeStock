use thiserror::Error;

/// Unified application error type shared across crates.
///
/// When the `axum-errors` feature is enabled (by the `api` crate),
/// this type automatically implements `axum::IntoResponse` with
/// structured JSON error bodies.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type AppResult<T> = Result<T, AppError>;

// ── axum IntoResponse — only compiled when the api crate enables axum-errors ──
#[cfg(feature = "axum-errors")]
mod axum_impl {
    use super::AppError;
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use serde_json::json;

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            let (status, message) = match &self {
                AppError::NotFound(msg)     => (StatusCode::NOT_FOUND,            msg.clone()),
                AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED,         msg.clone()),
                AppError::Forbidden(msg)    => (StatusCode::FORBIDDEN,            msg.clone()),
                AppError::BadRequest(msg)   => (StatusCode::BAD_REQUEST,          msg.clone()),
                AppError::Conflict(msg)     => (StatusCode::CONFLICT,             msg.clone()),
                AppError::Validation(msg)   => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
                AppError::Database(e) => {
                    tracing::error!("Database error: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".to_string())
                }
                AppError::Internal(msg) => {
                    tracing::error!("Internal error: {}", msg);
                    (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
                }
            };

            let body = Json(json!({ "error": message, "status": status.as_u16() }));
            (status, body).into_response()
        }
    }
}
