use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;
use shared::{AppError, AppResult, Supplier, CreateSupplierRequest, UpdateSupplierRequest};

pub async fn list_suppliers(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Supplier>>> {
    let suppliers = sqlx::query_as!(
        Supplier,
        "SELECT id, name, contact_name, email, phone, address, created_at, updated_at FROM suppliers ORDER BY name"
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(suppliers))
}

pub async fn get_supplier(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Supplier>> {
    let supplier = sqlx::query_as!(
        Supplier,
        "SELECT id, name, contact_name, email, phone, address, created_at, updated_at FROM suppliers WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Supplier {} not found", id)))?;
    Ok(Json(supplier))
}

pub async fn create_supplier(
    State(state): State<AppState>,
    Json(payload): Json<CreateSupplierRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO suppliers (id, name, contact_name, email, phone, address) VALUES ($1,$2,$3,$4,$5,$6)",
        id, payload.name, payload.contact_name, payload.email, payload.phone, payload.address
    )
    .execute(&state.db)
    .await?;
    Ok(Json(serde_json::json!({ "message": "Supplier created", "id": id })))
}

pub async fn update_supplier(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSupplierRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query!(
        r#"UPDATE suppliers SET
           name = COALESCE($1, name),
           contact_name = COALESCE($2, contact_name),
           email = COALESCE($3, email),
           phone = COALESCE($4, phone),
           address = COALESCE($5, address),
           updated_at = now()
           WHERE id = $6"#,
        payload.name, payload.contact_name, payload.email, payload.phone, payload.address, id
    )
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 { return Err(AppError::NotFound(format!("Supplier {} not found", id))); }
    Ok(Json(serde_json::json!({ "message": "Supplier updated" })))
}

pub async fn delete_supplier(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    sqlx::query!("DELETE FROM suppliers WHERE id = $1", id)
        .execute(&state.db)
        .await?;
    Ok(Json(serde_json::json!({ "message": "Supplier deleted" })))
}
