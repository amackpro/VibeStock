use axum::{extract::State, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::{auth::create_token, AppState};
use shared::{AppError, AppResult, LoginRequest, LoginResponse, User};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, username, email, password_hash, full_name,
                  role AS "role: _", is_active, created_at, updated_at
           FROM users WHERE username = $1 AND is_active = true"#,
        payload.username
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username or password".into()))?;

    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::Internal("Password verification failed".into()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid username or password".into()));
    }

    let role_str = format!("{:?}", user.role).to_lowercase();
    let token = create_token(
        user.id,
        &user.username,
        &role_str,
        &state.config.jwt_secret,
        state.config.jwt_expiry_hours,
    )?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id,
        username: user.username,
        full_name: user.full_name,
        role: role_str,
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    let username = payload["username"].as_str().ok_or(AppError::BadRequest("username required".into()))?;
    let email    = payload["email"].as_str().ok_or(AppError::BadRequest("email required".into()))?;
    let password = payload["password"].as_str().ok_or(AppError::BadRequest("password required".into()))?;
    let full_name = payload["full_name"].as_str().ok_or(AppError::BadRequest("full_name required".into()))?;

    let existing = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE username = $1 OR email = $2",
        username, email
    )
    .fetch_one(&state.db)
    .await?;

    if existing.unwrap_or(0) > 0 {
        return Err(AppError::Conflict("Username or email already exists".into()));
    }

    let hash = hash(password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Hashing failed: {}", e)))?;

    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO users (id, username, email, password_hash, full_name, role)
           VALUES ($1, $2, $3, $4, $5, 'staff')"#,
        id, username, email, hash, full_name
    )
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({ "message": "User registered successfully", "user_id": id })))
}

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok", "service": "VibeStock API", "version": "0.1.0" }))
}
