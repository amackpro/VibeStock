use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{auth::Claims, AppState};
use shared::{
    AppError, AppResult, CreateStockMovementRequest, MovementType,
    PaginationParams, PaginatedResponse, StockMovementWithDetails, WsEvent,
};

pub async fn list_movements(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<StockMovementWithDetails>>> {
    let limit = params.limit();
    let offset = params.offset();
    let page = params.page.unwrap_or(1);

    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM stock_movements")
        .fetch_one(&state.db)
        .await?
        .unwrap_or(0);

    let movements = sqlx::query_as!(
        StockMovementWithDetails,
        r#"SELECT sm.id, sm.product_id, p.name AS product_name, p.sku AS product_sku,
                  sm.movement_type AS "movement_type: MovementType",
                  sm.quantity, sm.reference, sm.notes,
                  sm.performed_by, u.full_name AS performed_by_name,
                  sm.created_at
           FROM stock_movements sm
           JOIN products p ON p.id = sm.product_id
           JOIN users u ON u.id = sm.performed_by
           ORDER BY sm.created_at DESC
           LIMIT $1 OFFSET $2"#,
        limit, offset
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(PaginatedResponse::new(movements, total, page, limit)))
}

pub async fn create_movement(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStockMovementRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let movement_type: MovementType = match payload.movement_type.as_str() {
        "in"         => MovementType::In,
        "out"        => MovementType::Out,
        "adjustment" => MovementType::Adjustment,
        "return"     => MovementType::Return,
        other => return Err(AppError::BadRequest(format!("Unknown movement type: {}", other))),
    };

    // Fetch product to validate stock and get name
    let product = sqlx::query!(
        "SELECT id, name, quantity_in_stock, reorder_level FROM products WHERE id = $1 AND is_active = true",
        payload.product_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Product {} not found", payload.product_id)))?;

    // Calculate new quantity
    let delta: i32 = match movement_type {
        MovementType::In | MovementType::Return  =>  payload.quantity,
        MovementType::Out                        => -payload.quantity,
        MovementType::Adjustment                 =>  payload.quantity - product.quantity_in_stock,
    };

    let new_qty = product.quantity_in_stock + delta;
    if new_qty < 0 {
        return Err(AppError::BadRequest(format!(
            "Insufficient stock. Current: {}, Requested: {}",
            product.quantity_in_stock, payload.quantity
        )));
    }

    // Insert movement
    let id = Uuid::new_v4();
    let mt_str = format!("{}", movement_type);
    sqlx::query!(
        r#"INSERT INTO stock_movements (id, product_id, movement_type, quantity, reference, notes, performed_by)
           VALUES ($1, $2, $3::movement_type, $4, $5, $6, $7)"#,
        id, payload.product_id, mt_str as _, payload.quantity,
        payload.reference, payload.notes, claims.sub
    )
    .execute(&state.db)
    .await?;

    // Update product stock
    sqlx::query!(
        "UPDATE products SET quantity_in_stock = $1, updated_at = now() WHERE id = $2",
        new_qty, payload.product_id
    )
    .execute(&state.db)
    .await?;

    // Broadcast WebSocket event
    let _ = state.ws_tx.send(WsEvent::StockUpdated {
        product_id: payload.product_id,
        product_name: product.name.clone(),
        new_quantity: new_qty,
    });

    // Broadcast low-stock warning if needed
    if new_qty <= product.reorder_level {
        let _ = state.ws_tx.send(WsEvent::LowStock {
            product_id: payload.product_id,
            product_name: product.name,
            quantity: new_qty,
            reorder_level: product.reorder_level,
        });
    }

    tracing::info!("Stock movement {} created by {}", id, claims.username);
    Ok(Json(serde_json::json!({
        "message": "Stock movement recorded",
        "id": id,
        "new_quantity": new_qty
    })))
}
