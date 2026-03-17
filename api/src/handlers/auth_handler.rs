use axum::{extract::State, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use validator::Validate;

use crate::{auth::create_token, AppState};
use shared::{AppError, AppResult, LoginRequest, LoginResponse, RegisterRequest, TenantInfo};

/// POST /api/auth/login
/// Verifies credentials and returns a signed JWT with tenant info.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {

    #[derive(sqlx::FromRow)]
    struct LoginRow {
        id:               Uuid,
        tenant_id:        Uuid,
        is_global_admin:  bool,
        username:         String,
        password_hash:    String,
        full_name:        String,
        role:             Option<String>,
        tenant_name:      Option<String>,
    }

    let row = sqlx::query_as::<_, LoginRow>(
        "SELECT u.id, u.tenant_id, u.is_global_admin, u.username, u.password_hash, 
                u.full_name, u.role::text AS role, t.name as tenant_name
         FROM users u
         LEFT JOIN tenants t ON t.id = u.tenant_id
         WHERE u.username = $1 AND u.is_active = true"
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
    let tenant_name = row.tenant_name.unwrap_or_else(|| "Unknown".to_string());

    let token = create_token(
        row.id,
        &row.username,
        &role_str,
        row.tenant_id,
        row.is_global_admin,
        &state.config.jwt_secret,
        state.config.jwt_expiry_hours,
    )?;

    let accessible_tenants = if row.is_global_admin {
        let tenants = sqlx::query_as::<_, TenantInfo>(
            "SELECT id, name, slug FROM tenants WHERE is_active = true ORDER BY name"
        )
        .fetch_all(&state.db)
        .await?;
        tenants
    } else {
        vec![TenantInfo {
            id: row.tenant_id,
            name: tenant_name.clone(),
            slug: "".to_string(),
        }]
    };

    Ok(Json(LoginResponse {
        token,
        user_id:  row.id,
        username:  row.username,
        full_name: row.full_name,
        role:      role_str,
        tenant_id: row.tenant_id,
        tenant_name,
        is_global_admin: row.is_global_admin,
        accessible_tenants,
    }))
}


/// POST /api/auth/register
/// Creates a new user account. If no tenant_id provided, creates a new tenant.
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    if let Err(e) = payload.validate() {
        return Err(AppError::BadRequest(format!("Validation error: {}", e)));
    }

    let exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE username = $1 OR email = $2"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await?;

    if exists > 0 {
        return Err(AppError::Conflict("Username or email already exists".into()));
    }

    let tenant_id = if let Some(tid) = payload.tenant_id {
        tid
    } else if let Some(ref tenant_name) = payload.tenant_name {
        let slug = tenant_name.to_lowercase().replace(' ', "-");
        let tid = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO tenants (id, name, slug) VALUES ($1, $2, $3)"
        )
        .bind(tid)
        .bind(tenant_name)
        .bind(&slug)
        .execute(&state.db)
        .await?;
        tid
    } else {
        return Err(AppError::BadRequest("Either tenant_id or tenant_name is required".into()));
    };

    let hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Hashing failed: {}", e)))?;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO users (id, tenant_id, username, email, password_hash, full_name, role) \
         VALUES ($1, $2, $3, $4, $5, $6, 'staff')"
    )
    .bind(id)
    .bind(tenant_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(hash)
    .bind(&payload.full_name)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({ "message": "User registered successfully", "user_id": id, "tenant_id": tenant_id })))
}

/// GET /api/health
pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status":  "ok",
        "service": "VibeStock API",
        "version": "0.1.0"
    }))
}
