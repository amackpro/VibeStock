use axum::{
    extract::{Request, State},
    http::HeaderMap,
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
    pub sub: Uuid,           // user id
    pub username: String,
    pub role: String,
    pub tenant_id: Uuid,
    pub is_global_admin: bool,
    pub exp: i64,
    pub iat: i64,
}

pub fn create_token(
    user_id: Uuid,
    username: &str,
    role: &str,
    tenant_id: Uuid,
    is_global_admin: bool,
    secret: &str,
    expiry_hours: i64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        username: username.to_owned(),
        role: role.to_owned(),
        tenant_id,
        is_global_admin,
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

pub fn extract_tenant_id(headers: &HeaderMap) -> Option<Uuid> {
    headers
        .get("X-Tenant-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
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
    
    let header_tenant_id = extract_tenant_id(req.headers());
    
    let effective_tenant_id = if claims.is_global_admin {
        header_tenant_id.unwrap_or(claims.tenant_id)
    } else {
        claims.tenant_id
    };
    
    req.extensions_mut().insert(claims);
    req.extensions_mut().insert(effective_tenant_id);
    Ok(next.run(req).await)
}

// ─── RBAC Middleware ──────────────────────────────────────────────────────────

pub async fn require_admin(
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| AppError::Unauthorized("Unauthorized context".into()))?;
        
    if claims.role != "admin" && !claims.is_global_admin {
        return Err(AppError::Forbidden("Requires admin privileges".into()));
    }
    
    Ok(next.run(req).await)
}

pub async fn require_manager(
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| AppError::Unauthorized("Unauthorized context".into()))?;
        
    if claims.role != "admin" && claims.role != "manager" && !claims.is_global_admin {
        return Err(AppError::Forbidden("Requires manager privileges".into()));
    }
    
    Ok(next.run(req).await)
}
