use axum::{extract::State, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::{auth::create_token, AppState};
use shared::{AppError, AppResult, LoginRequest, LoginResponse};

/// POST /api/auth/login
/// Verifies credentials and returns a signed JWT.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {

    #[derive(sqlx::FromRow)]
    struct LoginRow {
        id:            Uuid,
        username:      String,
        password_hash: String,
        full_name:     String,
        role:          Option<String>,
    }

    let row = sqlx::query_as::<_, LoginRow>(
        "SELECT id, username, password_hash, full_name, role::text AS role \
         FROM users WHERE username = $1 AND is_active = true"
    )
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username or password".into()))?;

    let valid = verify(&payload.password, &row.password_hash)
        .map_err(|_| AppError::Internal("Password verification failed".into()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid username or password".into()));
    }

    let role_str = row.role.unwrap_or_else(|| "staff".into());

    let token = create_token(
        row.id,
        &row.username,
        &role_str,
        &state.config.jwt_secret,
        state.config.jwt_expiry_hours,
    )?;

    Ok(Json(LoginResponse {
        token,
        user_id:  row.id,
        username:  row.username,
        full_name: row.full_name,
        role:      role_str,
    }))
}


/// POST /api/auth/register
/// Creates a new staff user account.
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    let username  = payload["username"].as_str()
        .ok_or(AppError::BadRequest("username required".into()))?;
    let email     = payload["email"].as_str()
        .ok_or(AppError::BadRequest("email required".into()))?;
    let password  = payload["password"].as_str()
        .ok_or(AppError::BadRequest("password required".into()))?;
    let full_name = payload["full_name"].as_str()
        .ok_or(AppError::BadRequest("full_name required".into()))?;

    let exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE username = $1 OR email = $2"
    )
    .bind(username)
    .bind(email)
    .fetch_one(&state.db)
    .await?;

    if exists > 0 {
        return Err(AppError::Conflict("Username or email already exists".into()));
    }

    let hash = hash(password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Hashing failed: {}", e)))?;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, full_name, role) \
         VALUES ($1, $2, $3, $4, $5, 'staff')"
    )
    .bind(id)
    .bind(username)
    .bind(email)
    .bind(hash)
    .bind(full_name)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({ "message": "User registered successfully", "user_id": id })))
}

/// GET /api/health
pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status":  "ok",
        "service": "VibeStock API",
        "version": "0.1.0"
    }))
}
