use axum::{
    extract::{Extension, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{auth::Claims, AppState};
use shared::{
    AppError, AppResult, CreateStockMovementRequest,
    PaginatedResponse, PaginationParams, StockMovementWithDetails, WsEvent,
};

/// GET /api/movements — paginated stock movement history
pub async fn list_movements(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<StockMovementWithDetails>>> {
    let limit  = params.limit();
    let offset = params.offset();
    let page   = params.page.unwrap_or(1);

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM stock_movements WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_one(&state.db)
    .await?;

    let movements = sqlx::query_as::<_, StockMovementWithDetails>(
        "SELECT sm.id, sm.tenant_id,
                sm.product_id,
                p.name        AS product_name,
                p.sku         AS product_sku,
                sm.movement_type::text AS movement_type,
                sm.quantity,
                sm.reference,
                sm.notes,
                sm.performed_by,
                u.full_name   AS performed_by_name,
                sm.created_at
         FROM stock_movements sm
         JOIN products p ON p.id = sm.product_id
         JOIN users u    ON u.id = sm.performed_by
         WHERE sm.tenant_id = $1
         ORDER BY sm.created_at DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(tenant_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(PaginatedResponse::new(movements, total, page, limit)))
}

/// POST /api/movements — atomically records movement and updates stock quantity
pub async fn create_movement(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStockMovementRequest>,
) -> AppResult<Json<serde_json::Value>> {

    // ── Validate movement type ────────────────────────────────────────────────
    match payload.movement_type.as_str() {
        "in" | "out" | "adjustment" | "return" => {},
        other => return Err(AppError::BadRequest(
            format!("Unknown movement type: '{}'. Use: in, out, adjustment, return", other)
        )),
    }

    // ── Fetch current product state ───────────────────────────────────────────
    #[derive(sqlx::FromRow)]
    struct ProductRow { id: Uuid, name: String, quantity_in_stock: i32, reorder_level: i32 }

    let product = sqlx::query_as::<_, ProductRow>(
        "SELECT id, name, quantity_in_stock, reorder_level \
         FROM products WHERE id = $1 AND tenant_id = $2 AND is_active = true"
    )
    .bind(payload.product_id)
    .bind(tenant_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Product {} not found", payload.product_id)))?;

    // ── Calculate stock delta ─────────────────────────────────────────────────
    let delta: i32 = match payload.movement_type.as_str() {
        "in" | "return" => payload.quantity,
        "out"           => -payload.quantity,
        "adjustment"    => payload.quantity - product.quantity_in_stock,
        _               => unreachable!(),
    };

    let new_qty = product.quantity_in_stock + delta;
    if new_qty < 0 {
        return Err(AppError::BadRequest(format!(
            "Insufficient stock. Current: {}, Requested reduction: {}",
            product.quantity_in_stock, payload.quantity
        )));
    }

    // ── Insert stock movement ─────────────────────────────────────────────────
    let id = Uuid::new_v4();
    let mt = payload.movement_type.as_str();
    sqlx::query(
        "INSERT INTO stock_movements \
         (id, tenant_id, product_id, movement_type, quantity, reference, notes, performed_by) \
         VALUES ($1, $2, $3, $4::movement_type, $5, $6, $7, $8)"
    )
    .bind(id)
    .bind(tenant_id)
    .bind(payload.product_id)
    .bind(mt)
    .bind(payload.quantity)
    .bind(&payload.reference)
    .bind(&payload.notes)
    .bind(claims.sub)
    .execute(&state.db)
    .await?;

    // ── Update product stock ──────────────────────────────────────────────────
    sqlx::query(
        "UPDATE products SET quantity_in_stock = $1, updated_at = now() WHERE id = $2"
    )
    .bind(new_qty)
    .bind(payload.product_id)
    .execute(&state.db)
    .await?;

    // ── Broadcast WebSocket events ────────────────────────────────────────────
    let _ = state.ws_tx.send(WsEvent::StockUpdated {
        product_id:   payload.product_id,
        product_name: product.name.clone(),
        new_quantity: new_qty,
    });

    if new_qty <= product.reorder_level {
        let _ = state.ws_tx.send(shared::WsEvent::LowStock {
            product_id:    payload.product_id,
            product_name:  product.name,
            quantity:      new_qty,
            reorder_level: product.reorder_level,
        });
    }

    let _ = state.ws_tx.send(shared::WsEvent::NewMovement {
        product_id: payload.product_id,
        movement_type: payload.movement_type.clone(),
        quantity: payload.quantity,
    });

    tracing::info!("Movement {} recorded by {}", id, claims.username);
    Ok(Json(serde_json::json!({
        "message":      "Stock movement recorded",
        "id":           id,
        "new_quantity": new_qty
    })))
}
