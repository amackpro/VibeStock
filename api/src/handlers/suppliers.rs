use axum::{
    extract::{Extension, Path, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;
use shared::{AppError, AppResult, Supplier, CreateSupplierRequest, UpdateSupplierRequest};

/// GET /api/suppliers — list all suppliers ordered by name
pub async fn list_suppliers(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<Vec<Supplier>>> {
    let suppliers = sqlx::query_as::<_, Supplier>(
        "SELECT id, tenant_id, name, contact_name, email, phone, address, city_id, created_at, updated_at \
         FROM suppliers WHERE tenant_id = $1 ORDER BY name"
    )
    .bind(tenant_id)
    .fetch_all(&state.db)
    .await?;
    Ok(Json(suppliers))
}

/// GET /api/suppliers/:id
pub async fn get_supplier(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Supplier>> {
    let supplier = sqlx::query_as::<_, Supplier>(
        "SELECT id, tenant_id, name, contact_name, email, phone, address, city_id, created_at, updated_at \
         FROM suppliers WHERE id = $1 AND tenant_id = $2"
    )
    .bind(id)
    .bind(tenant_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Supplier {} not found", id)))?;
    Ok(Json(supplier))
}

/// POST /api/suppliers
pub async fn create_supplier(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Json(payload): Json<CreateSupplierRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO suppliers (id, tenant_id, name, contact_name, email, phone, address, city_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(id)
    .bind(tenant_id)
    .bind(&payload.name)
    .bind(&payload.contact_name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(&payload.address)
    .bind(&payload.city_id)
    .execute(&state.db)
    .await?;
    Ok(Json(serde_json::json!({ "message": "Supplier created", "id": id })))
}

/// PUT /api/suppliers/:id
pub async fn update_supplier(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSupplierRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "UPDATE suppliers SET
         name         = COALESCE($1, name),
         contact_name = COALESCE($2, contact_name),
         email        = COALESCE($3, email),
         phone        = COALESCE($4, phone),
         address      = COALESCE($5, address),
         city_id      = COALESCE($6, city_id),
         updated_at   = now()
         WHERE id = $7 AND tenant_id = $8"
    )
    .bind(&payload.name)
    .bind(&payload.contact_name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(&payload.address)
    .bind(&payload.city_id)
    .bind(id)
    .bind(tenant_id)
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Supplier {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Supplier updated" })))
}

/// DELETE /api/suppliers/:id
pub async fn delete_supplier(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query("DELETE FROM suppliers WHERE id = $1 AND tenant_id = $2")
        .bind(id)
        .bind(tenant_id)
        .execute(&state.db)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Supplier {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Supplier deleted" })))
}
