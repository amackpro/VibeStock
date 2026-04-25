use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;
use shared::{
    AppError, AppResult, CreateTenantRequest, Tenant, TenantWithStats, UpdateTenantRequest,
};

pub async fn list_tenants(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<TenantWithStats>>> {
    let tenants = sqlx::query_as::<_, TenantWithStats>(
        "SELECT 
            t.id, t.name, t.slug, t.owner_user_id, t.is_active, t.created_at,
            COUNT(DISTINCT u.id) as total_users,
            COUNT(DISTINCT p.id) as total_products,
            COUNT(DISTINCT s.id) as total_suppliers
         FROM tenants t
         LEFT JOIN users u ON u.tenant_id = t.id
         LEFT JOIN products p ON p.tenant_id = t.id
         LEFT JOIN suppliers s ON s.tenant_id = t.id
         GROUP BY t.id
         ORDER BY t.created_at DESC"
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(tenants))
}

pub async fn get_tenant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<TenantWithStats>> {
    let tenant = sqlx::query_as::<_, TenantWithStats>(
        "SELECT 
            t.id, t.name, t.slug, t.owner_user_id, t.is_active, t.created_at,
            COUNT(DISTINCT u.id) as total_users,
            COUNT(DISTINCT p.id) as total_products,
            COUNT(DISTINCT s.id) as total_suppliers
         FROM tenants t
         LEFT JOIN users u ON u.tenant_id = t.id
         LEFT JOIN products p ON p.tenant_id = t.id
         LEFT JOIN suppliers s ON s.tenant_id = t.id
         WHERE t.id = $1
         GROUP BY t.id"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Tenant {} not found", id)))?;
    Ok(Json(tenant))
}

pub async fn create_tenant(
    State(state): State<AppState>,
    Json(payload): Json<CreateTenantRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO tenants (id, name, slug) VALUES ($1, $2, $3)"
    )
    .bind(id)
    .bind(&payload.name)
    .bind(&payload.slug)
    .execute(&state.db)
    .await?;
    Ok(Json(serde_json::json!({ "message": "Tenant created", "id": id })))
}

pub async fn update_tenant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTenantRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "UPDATE tenants SET \
         name = COALESCE($1::text, name), \
         is_active = COALESCE($2::bool, is_active), \
         updated_at = now() \
         WHERE id = $3"
    )
    .bind(&payload.name)
    .bind(&payload.is_active)
    .bind(id)
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Tenant {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Tenant updated" })))
}

pub async fn delete_tenant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query("DELETE FROM tenants WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Tenant {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Tenant deleted" })))
}

pub async fn get_tenant_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<Tenant>> {
    let tenant = sqlx::query_as::<_, Tenant>(
        "SELECT id, name, slug, owner_user_id, is_active, created_at, updated_at \
         FROM tenants WHERE slug = $1"
    )
    .bind(slug)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Tenant not found".to_string()))?;
    Ok(Json(tenant))
}
