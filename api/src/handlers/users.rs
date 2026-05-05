use axum::{
    extract::{Extension, Path, State},
    Json,
};
use bcrypt::hash;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{auth::Claims, AppState};
use shared::{AppError, AppResult, User};

/// Require admin role, returning an error if unauthorized.
fn require_admin(claims: &Claims) -> AppResult<()> {
    let role_lower = claims.role.to_lowercase();
    if role_lower != "admin" && !claims.is_global_admin {
        return Err(AppError::Unauthorized("Admin privileges required".into()));
    }
    Ok(())
}

/// GET /api/users
pub async fn list_users(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<Vec<shared::UserWithTenant>>> {
    require_admin(&claims)?;

    let users = if claims.is_global_admin {
        sqlx::query_as::<_, shared::UserWithTenant>(
            "SELECT 
                u.id, u.tenant_id, t.name as tenant_name, u.is_global_admin, u.username, u.email, u.full_name, 
                u.role::text AS role, u.is_active, 
                u.created_at, u.updated_at 
             FROM users u
             LEFT JOIN tenants t ON t.id = u.tenant_id
             ORDER BY u.full_name ASC"
        )
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, shared::UserWithTenant>(
            "SELECT 
                u.id, u.tenant_id, t.name as tenant_name, u.is_global_admin, u.username, u.email, u.full_name, 
                u.role::text AS role, u.is_active, 
                u.created_at, u.updated_at 
             FROM users u
             LEFT JOIN tenants t ON t.id = u.tenant_id
             WHERE u.tenant_id = $1
             ORDER BY u.full_name ASC"
        )
        .bind(tenant_id)
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(users))
}

#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

/// PATCH /api/users/:id/role
pub async fn update_user_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRoleRequest>,
) -> AppResult<Json<User>> {
    require_admin(&claims)?;

    // Validate role (case insensitive)
    let role_lower = payload.role.to_lowercase();
    let valid_roles = ["admin", "manager", "staff"];
    if !valid_roles.contains(&role_lower.as_str()) {
        return Err(AppError::BadRequest("Invalid role provided".into()));
    }

    // Prevent demoting oneself
    if id == claims.sub && role_lower != "admin" {
        return Err(AppError::BadRequest("You cannot demote yourself".into()));
    }

    // Enforce single-admin-per-org: if promoting to admin, ensure no other user in the
    // same tenant already holds that role (excluding the user being updated themselves).
    if role_lower == "admin" {
        let admin_exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users \
             WHERE tenant_id = (SELECT tenant_id FROM users WHERE id = $1) \
               AND role = 'admin'::user_role \
               AND id != $1"
        )
        .bind(id)
        .fetch_one(&state.db)
        .await?;

        if admin_exists > 0 {
            return Err(AppError::Conflict(
                "This organization already has an admin. \
                 Each organization can only have one admin.".into(),
            ));
        }
    }

    let user = if claims.is_global_admin {
        sqlx::query_as::<_, User>(
            "UPDATE users 
             SET role = $1::user_role, updated_at = NOW() 
             WHERE id = $2 
             RETURNING id, tenant_id, is_global_admin, username, email, password_hash, full_name, role::text AS role, is_active, created_at, updated_at"
        )
        .bind(&role_lower)
        .bind(id)
        .fetch_optional(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, User>(
            "UPDATE users 
             SET role = $1::user_role, updated_at = NOW() 
             WHERE id = $2 AND tenant_id = $3
             RETURNING id, tenant_id, is_global_admin, username, email, password_hash, full_name, role::text AS role, is_active, created_at, updated_at"
        )
        .bind(&role_lower)
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&state.db)
        .await?
    };

    let user = user.ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub is_active: bool,
}

/// PATCH /api/users/:id/status
pub async fn toggle_user_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateStatusRequest>,
) -> AppResult<Json<User>> {
    require_admin(&claims)?;

    // Prevent suspending oneself
    if id == claims.sub && !payload.is_active {
        return Err(AppError::BadRequest("You cannot suspend yourself".into()));
    }

    let user = if claims.is_global_admin {
        sqlx::query_as::<_, User>(
            "UPDATE users 
             SET is_active = $1, updated_at = NOW() 
             WHERE id = $2 
             RETURNING id, tenant_id, is_global_admin, username, email, password_hash, full_name, role::text AS role, is_active, created_at, updated_at"
        )
        .bind(payload.is_active)
        .bind(id)
        .fetch_optional(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, User>(
            "UPDATE users 
             SET is_active = $1, updated_at = NOW() 
             WHERE id = $2 AND tenant_id = $3
             RETURNING id, tenant_id, is_global_admin, username, email, password_hash, full_name, role::text AS role, is_active, created_at, updated_at"
        )
        .bind(payload.is_active)
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&state.db)
        .await?
    };

    let user = user.ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user))
}

/// Request body for admin-created users
#[derive(serde::Deserialize, validator::Validate)]
pub struct AdminCreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1, max = 255))]
    pub full_name: String,
    /// One of: "admin", "manager", "staff"  (defaults to "staff")
    pub role: Option<String>,
}

/// POST /api/users
/// Admin-only: create a new user inside the same tenant as the calling admin.
pub async fn create_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant_id): Extension<Uuid>,
    Json(payload): Json<AdminCreateUserRequest>,
) -> AppResult<Json<serde_json::Value>> {
    require_admin(&claims)?;

    if let Err(e) = payload.validate() {
        return Err(AppError::BadRequest(format!("Validation error: {}", e)));
    }

    let role_str = payload.role.as_deref().unwrap_or("staff").to_lowercase();
    let valid_roles = ["admin", "manager", "staff"];
    if !valid_roles.contains(&role_str.as_str()) {
        return Err(AppError::BadRequest(
            "Invalid role. Must be one of: admin, manager, staff".into(),
        ));
    }

    // Enforce single-admin-per-org rule
    if role_str == "admin" {
        let admin_exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE tenant_id = $1 AND role = 'admin'::user_role"
        )
        .bind(tenant_id)
        .fetch_one(&state.db)
        .await?;

        if admin_exists > 0 {
            return Err(AppError::Conflict(
                "This organization already has an admin. \
                 Each organization can only have one admin.".into(),
            ));
        }
    }

    let exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE (username = $1 OR email = $2) AND tenant_id IS NOT NULL"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await?;

    if exists > 0 {
        return Err(AppError::Conflict("Username or email already exists".into()));
    }

    let password_hash = hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Hashing failed: {}", e)))?;

    let user_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO users (id, tenant_id, username, email, password_hash, full_name, role) \
         VALUES ($1, $2, $3, $4, $5, $6, $7::user_role)"
    )
    .bind(user_id)
    .bind(tenant_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.full_name)
    .bind(&role_str)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "message": "User created successfully",
        "user_id": user_id,
        "tenant_id": tenant_id,
        "role": role_str
    })))
}

/// DELETE /api/users/:id
/// Tenant admin: remove a user from their own org (with safety guards).
/// Global admin: remove any user from any org.
pub async fn delete_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant_id): Extension<Uuid>,
    Path(target_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    require_admin(&claims)?;

    // Guard 1: cannot delete yourself
    if target_id == claims.sub {
        return Err(AppError::BadRequest(
            "You cannot delete your own account.".into(),
        ));
    }

    // Fetch the target user to validate ownership and role
    #[derive(sqlx::FromRow)]
    struct TargetUser {
        tenant_id: Option<Uuid>,
        is_global_admin: bool,
        role: Option<String>,
    }

    let target = sqlx::query_as::<_, TargetUser>(
        "SELECT tenant_id, is_global_admin, role::text AS role FROM users WHERE id = $1"
    )
    .bind(target_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Guard 2: tenant admins can only delete users in their own org
    if !claims.is_global_admin {
        match target.tenant_id {
            Some(tid) if tid == tenant_id => {}
            _ => return Err(AppError::Unauthorized(
                "You can only remove users from your own organization.".into(),
            )),
        }
    }

    // Guard 3: nobody can delete a global admin (except another global admin)
    if target.is_global_admin && !claims.is_global_admin {
        return Err(AppError::Unauthorized(
            "Global admin accounts cannot be deleted.".into(),
        ));
    }

    // Guard 4: cannot remove the last admin of a tenant (global admin bypasses this)
    let role_str = target.role.as_deref().unwrap_or("staff");
    if role_str == "admin" && !claims.is_global_admin {
        let admin_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users \
             WHERE tenant_id = $1 AND role = 'admin'::user_role AND is_active = true AND id != $2"
        )
        .bind(target.tenant_id)
        .bind(target_id)
        .fetch_one(&state.db)
        .await?;

        if admin_count == 0 {
            return Err(AppError::BadRequest(
                "Cannot delete the last admin of an organization. \
                 Promote another user to admin first.".into(),
            ));
        }
    }

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(target_id)
        .execute(&state.db)
        .await?;

    Ok(Json(serde_json::json!({ "message": "User deleted successfully" })))
}
