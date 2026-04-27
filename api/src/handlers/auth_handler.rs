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

    let mode = payload.mode.as_deref().unwrap_or("new");

    let (tenant_id, role_to_assign, is_active) = match mode {
        // ── JOIN EXISTING ────────────────────────────────────────────────────
        "join" => {
            let slug = payload.tenant_slug.as_deref().ok_or_else(|| {
                AppError::BadRequest("tenant_slug is required when joining an organization.".into())
            })?;

            #[derive(sqlx::FromRow)]
            struct TenantRow { id: Uuid }

            let tenant = sqlx::query_as::<_, TenantRow>(
                "SELECT id FROM tenants WHERE slug = $1 AND is_active = true"
            )
            .bind(slug)
            .fetch_optional(&state.db)
            .await?
            .ok_or_else(|| AppError::NotFound(
                "No active organization found with that name. Check the name and try again.".into()
            ))?;

            // Join as inactive staff — the org admin must activate the account
            (tenant.id, "staff", false)
        }

        // ── CREATE NEW ───────────────────────────────────────────────────────
        _ => {
            let tenant_name = payload.tenant_name.as_deref().ok_or_else(|| {
                AppError::BadRequest("Organization nam