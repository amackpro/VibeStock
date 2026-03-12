use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use shared::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,        // user id
    pub username: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn create_token(
    user_id: Uuid,
    username: &str,
    role: &str,
    secret: &str,
    expiry_hours: i64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        username: username.to_owned(),
        role: role.to_owned(),
        iat: now.timestamp(),
        exp: (now + Duration::hours(expiry_hours)).timestamp(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| AppError::Internal(format!("Token creation failed: {}", e)))
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
}

pub fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_owned())
}

// ─── Auth Middleware ──────────────────────────────────────────────────────────

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token(req.headers())
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;
    let claims = verify_token(&token, &state.config.jwt_secret)?;
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
