use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;
use shared::{AppError, AppResult, Supplier, SupplierWithDetails, SupplierFilterParams, CreateSupplierRequest, UpdateSupplierRequest};

/// GET /api/suppliers — list all suppliers ordered by name
pub async fn list_suppliers(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Query(params): Query<SupplierFilterParams>,
) -> AppResult<Json<Vec<SupplierWithDetails>>> {
    let mut conditions = vec!["s.tenant_id = $1".to_string()];
    if params.region_id.is_some()  { conditions.push(format!("r.id = ${}", conditions.len() + 1)); }
    if params.country_id.is_some() { conditions.push(format!("co.id = ${}", conditions.len() + 1)); }
    if params.city_id.is_some()    { conditions.push(format!("ci.id = ${}", conditions.len() + 1)); }

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        "SELECT 
            s.id, s.tenant_id, s.name, s.contact_name, s.email, s.phone, s.address, 
            s.city_id, ci.name as city_name,
            co.id as country_id, co.name as country_name,
            r.id as region_id, r.name as region_name,
            COUNT(DISTINCT p.id) as product_count,
            s.created_at, s.updated_at
         FROM suppliers s
         LEFT JOIN cities ci ON s.city_id = ci.id
         LEFT JOIN countries co ON ci.country_id = co.id
         LEFT JOIN regions r ON co.region_id = r.id
         LEFT JOIN products p ON p.supplier_id = s.id
         WHERE {where_clause}
         GROUP BY s.id, ci.name, co.id, co.name, r.id, r.name
         ORDER BY s.name"
    );

    let mut q = sqlx::query_as::<_, SupplierWithDetails>(&sql).bind(tenant_id);
    
    let mut current_bind = 2;
    if let Some(id) = params.region_id { q = q.bind(id); current_bind += 1; }
    if let Some(id) = params.country_id { q = q.bind(id); current_bind += 1; }
    if let Some(id) = params.city_id    { q = q.bind(id); }

    let suppliers = q.fetch_all(&state.db).await?;
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
