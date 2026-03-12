use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::Claims, AppState};
use shared::{AppError, AppResult, User};

/// Require admin role, returning an error if unauthorized.
fn require_admin(claims: &Claims) -> AppResult<()> {
    if claims.role != "admin" {
        return Err(AppError::Unauthorized("Admin privileges required".into()));
    }
    Ok(())
}

/// GET /api/users
pub async fn list_users(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<Json<Vec<User>>> {
    require_admin(&claims)?;

    let users = sqlx::query_as::<_, User>(
        "SELECT 
            id, username, email, full_name, 
            role::text AS role, is_active, 
            created_at, updated_at 
         FROM users 
         ORDER BY full_name ASC"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(users))
}

#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

/// PATCH /api/users/:id/role
pub async fn update_user_role(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateRoleRequest>,
) -> AppResult<Json<User>> {
    require_admin(&claims)?;

    // Validate role
    let valid_roles = ["admin", "manager", "staff"];
    if !valid_roles.contains(&payload.role.as_str()) {
        return Err(AppError::BadRequest("Invalid role provided".into()));
    }

    // Prevent demoting the last admin or oneself (optional but good practice)
    if id == claims.sub && payload.role != "admin" {
        return Err(AppError::BadRequest("You cannot demote yourself".into()));
    }

    let user = sqlx::query_as::<_, User>(
        "UPDATE users 
         SET role = $1::user_role, updated_at = NOW() 
         WHERE id = $2 
         RETURNING id, username, email, full_name, role::text AS role, is_active, created_at, updated_at"
    )
    .bind(&payload.role)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub is_active: bool,
}

/// PATCH /api/users/:id/status
pub async fn toggle_user_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateStatusRequest>,
) -> AppResult<Json<User>> {
    require_admin(&claims)?;

    // Prevent suspending oneself
    if id == claims.sub && !payload.is_active {
        return Err(AppError::BadRequest("You cannot suspend yourself".into()));
    }

    let user = sqlx::query_as::<_, User>(
        "UPDATE users 
         SET is_active = $1, updated_at = NOW() 
         WHERE id = $2 
         RETURNING id, username, email, full_name, role::text AS role, is_active, created_at, updated_at"
    )
    .bind(payload.is_active)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(user))
}
