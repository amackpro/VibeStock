use axum::{
    extract::{Path, Query, State, Extension},
    Json,
};
use uuid::Uuid;

use crate::{auth::Claims, AppState};
use shared::{
    AppError, AppResult, CreateProductRequest, UpdateProductRequest,
    PaginationParams, PaginatedResponse, ProductWithDetails, WsEvent,
};

pub async fn list_products(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<ProductWithDetails>>> {
    let search = params.search.as_deref().unwrap_or("").to_string();
    let pattern = format!("%{}%", search);
    let limit = params.limit();
    let offset = params.offset();
    let page = params.page.unwrap_or(1);

    let total = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM products p WHERE p.is_active = true AND (p.name ILIKE $1 OR p.sku ILIKE $1)",
        pattern
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0);

    let products = sqlx::query_as!(
        ProductWithDetails,
        r#"SELECT p.id, p.name, p.description, p.sku, p.barcode,
                  p.category_id, c.name AS category_name,
                  p.supplier_id, s.name AS supplier_name,
                  p.unit_price, p.cost_price,
                  p.quantity_in_stock, p.reorder_level,
                  p.unit_of_measure, p.is_active, p.image_url,
                  p.created_at, p.updated_at
           FROM products p
           LEFT JOIN categories c ON c.id = p.category_id
           LEFT JOIN suppliers s ON s.id = p.supplier_id
           WHERE p.is_active = true AND (p.name ILIKE $1 OR p.sku ILIKE $1)
           ORDER BY p.name ASC
           LIMIT $2 OFFSET $3"#,
        pattern, limit, offset
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(PaginatedResponse::new(products, total, page, limit)))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ProductWithDetails>> {
    let product = sqlx::query_as!(
        ProductWithDetails,
        r#"SELECT p.id, p.name, p.description, p.sku, p.barcode,
                  p.category_id, c.name AS category_name,
                  p.supplier_id, s.name AS supplier_name,
                  p.unit_price, p.cost_price,
                  p.quantity_in_stock, p.reorder_level,
                  p.unit_of_measure, p.is_active, p.image_url,
                  p.created_at, p.updated_at
           FROM products p
           LEFT JOIN categories c ON c.id = p.category_id
           LEFT JOIN suppliers s ON s.id = p.supplier_id
           WHERE p.id = $1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))?;

    Ok(Json(product))
}

pub async fn get_product_by_barcode(
    State(state): State<AppState>,
    Path(barcode): Path<String>,
) -> AppResult<Json<ProductWithDetails>> {
    let product = sqlx::query_as!(
        ProductWithDetails,
        r#"SELECT p.id, p.name, p.description, p.sku, p.barcode,
                  p.category_id, c.name AS category_name,
                  p.supplier_id, s.name AS supplier_name,
                  p.unit_price, p.cost_price,
                  p.quantity_in_stock, p.reorder_level,
                  p.unit_of_measure, p.is_active, p.image_url,
                  p.created_at, p.updated_at
           FROM products p
           LEFT JOIN categories c ON c.id = p.category_id
           LEFT JOIN suppliers s ON s.id = p.supplier_id
           WHERE p.barcode = $1 AND p.is_active = true"#,
        barcode
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("No product with barcode {}", barcode)))?;

    Ok(Json(product))
}

pub async fn create_product(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    // Check duplicate SKU
    let exists = sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE sku = $1", payload.sku)
        .fetch_one(&state.db)
        .await?
        .unwrap_or(0) > 0;
    if exists {
        return Err(AppError::Conflict(format!("SKU '{}' already exists", payload.sku)));
    }

    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO products
           (id, name, description, sku, barcode, category_id, supplier_id,
            unit_price, cost_price, quantity_in_stock, reorder_level, unit_of_measure, image_url)
           VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)"#,
        id, payload.name, payload.description, payload.sku, payload.barcode,
        payload.category_id, payload.supplier_id, payload.unit_price, payload.cost_price,
        payload.quantity_in_stock, payload.reorder_level, payload.unit_of_measure, payload.image_url
    )
    .execute(&state.db)
    .await?;

    tracing::info!("Product created: {} by {}", id, claims.username);
    Ok(Json(serde_json::json!({ "message": "Product created", "id": id })))
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query!(
        r#"UPDATE products SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            barcode = COALESCE($3, barcode),
            category_id = COALESCE($4, category_id),
            supplier_id = COALESCE($5, supplier_id),
            unit_price = COALESCE($6, unit_price),
            cost_price = COALESCE($7, cost_price),
            reorder_level = COALESCE($8, reorder_level),
            unit_of_measure = COALESCE($9, unit_of_measure),
            is_active = COALESCE($10, is_active),
            image_url = COALESCE($11, image_url),
            updated_at = now()
           WHERE id = $12"#,
        payload.name, payload.description, payload.barcode,
        payload.category_id, payload.supplier_id, payload.unit_price,
        payload.cost_price, payload.reorder_level, payload.unit_of_measure,
        payload.is_active, payload.image_url, id
    )
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Product {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Product updated" })))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query!(
        "UPDATE products SET is_active = false, updated_at = now() WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Product {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Product deleted" })))
}
