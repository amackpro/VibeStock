use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{auth::Claims, AppState};
use shared::{AppError, AppResult, PaginatedResponse, PaginationParams, ProductWithDetails};

/// GET /api/products — paginated, searchable product list with category/supplier join
pub async fn list_products(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<ProductWithDetails>>> {
    let search  = format!("%{}%", params.search.as_deref().unwrap_or(""));
    let limit   = params.limit();
    let offset  = params.offset();
    let page    = params.page.unwrap_or(1);

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products p WHERE p.tenant_id = $1 AND p.is_active = true \
         AND (p.name ILIKE $2 OR p.sku ILIKE $2)"
    )
    .bind(tenant_id)
    .bind(&search)
    .fetch_one(&state.db)
    .await?;

    let products = sqlx::query_as::<_, ProductWithDetails>(
        "SELECT p.id, p.tenant_id, p.name, p.description, p.sku, p.barcode,
                p.category_id, c.name AS category_name,
                p.supplier_id, s.name AS supplier_name,
                p.unit_price, p.cost_price,
                p.quantity_in_stock, p.reorder_level,
                p.unit_of_measure, p.is_active, p.image_url,
                p.created_at, p.updated_at
         FROM products p
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN suppliers  s ON s.id = p.supplier_id
         WHERE p.tenant_id = $1 AND p.is_active = true AND (p.name ILIKE $2 OR p.sku ILIKE $2)
         ORDER BY p.name ASC
         LIMIT $3 OFFSET $4"
    )
    .bind(tenant_id)
    .bind(&search)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(PaginatedResponse::new(products, total, page, limit)))
}

/// GET /api/products/:id
pub async fn get_product(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ProductWithDetails>> {
    let product = sqlx::query_as::<_, ProductWithDetails>(
        "SELECT p.id, p.tenant_id, p.name, p.description, p.sku, p.barcode,
                p.category_id, c.name AS category_name,
                p.supplier_id, s.name AS supplier_name,
                p.unit_price, p.cost_price,
                p.quantity_in_stock, p.reorder_level,
                p.unit_of_measure, p.is_active, p.image_url,
                p.created_at, p.updated_at
         FROM products p
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN suppliers  s ON s.id = p.supplier_id
         WHERE p.id = $1 AND p.tenant_id = $2"
    )
    .bind(id)
    .bind(tenant_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))?;

    Ok(Json(product))
}

/// GET /api/products/barcode/:barcode — used by Android scanner
pub async fn get_product_by_barcode(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(barcode): Path<String>,
) -> AppResult<Json<ProductWithDetails>> {
    let product = sqlx::query_as::<_, ProductWithDetails>(
        "SELECT p.id, p.tenant_id, p.name, p.description, p.sku, p.barcode,
                p.category_id, c.name AS category_name,
                p.supplier_id, s.name AS supplier_name,
                p.unit_price, p.cost_price,
                p.quantity_in_stock, p.reorder_level,
                p.unit_of_measure, p.is_active, p.image_url,
                p.created_at, p.updated_at
         FROM products p
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN suppliers  s ON s.id = p.supplier_id
         WHERE p.barcode = $1 AND p.tenant_id = $2 AND p.is_active = true"
    )
    .bind(&barcode)
    .bind(tenant_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("No product with barcode {}", barcode)))?;

    Ok(Json(product))
}

/// POST /api/products
pub async fn create_product(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<shared::CreateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND sku = $2"
    )
    .bind(tenant_id)
    .bind(&payload.sku)
    .fetch_one(&state.db)
    .await?;

    if exists > 0 {
        return Err(AppError::Conflict(format!("SKU '{}' already exists", payload.sku)));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO products
         (id, tenant_id, name, description, sku, barcode, category_id, supplier_id,
          unit_price, cost_price, quantity_in_stock, reorder_level, unit_of_measure, image_url)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)"
    )
    .bind(id)
    .bind(tenant_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.sku)
    .bind(&payload.barcode)
    .bind(payload.category_id)
    .bind(payload.supplier_id)
    .bind(payload.unit_price)
    .bind(payload.cost_price)
    .bind(payload.quantity_in_stock)
    .bind(payload.reorder_level)
    .bind(&payload.unit_of_measure)
    .bind(&payload.image_url)
    .execute(&state.db)
    .await?;

    tracing::info!("Product created: {} by {}", id, claims.username);
    Ok(Json(serde_json::json!({ "message": "Product created", "id": id })))
}

/// PUT /api/products/:id
pub async fn update_product(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(payload): Json<shared::UpdateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "UPDATE products SET
         name              = COALESCE($1,  name),
         description       = COALESCE($2,  description),
         barcode           = COALESCE($3,  barcode),
         category_id       = COALESCE($4,  category_id),
         supplier_id       = COALESCE($5,  supplier_id),
         unit_price        = COALESCE($6,  unit_price),
         cost_price        = COALESCE($7,  cost_price),
         reorder_level     = COALESCE($8,  reorder_level),
         unit_of_measure   = COALESCE($9,  unit_of_measure),
         is_active         = COALESCE($10, is_active),
         image_url         = COALESCE($11, image_url),
         updated_at        = now()
         WHERE id = $12 AND tenant_id = $13"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.barcode)
    .bind(payload.category_id)
    .bind(payload.supplier_id)
    .bind(payload.unit_price)
    .bind(payload.cost_price)
    .bind(payload.reorder_level)
    .bind(&payload.unit_of_measure)
    .bind(payload.is_active)
    .bind(&payload.image_url)
    .bind(id)
    .bind(tenant_id)
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Product {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Product updated" })))
}

/// DELETE /api/products/:id — soft delete
pub async fn delete_product(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "UPDATE products SET is_active = false, updated_at = now() WHERE id = $1 AND tenant_id = $2"
    )
    .bind(id)
    .bind(tenant_id)
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Product {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Product deleted" })))
}
