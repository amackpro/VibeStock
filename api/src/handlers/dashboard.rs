use axum::{extract::State, Json};

use crate::AppState;
use shared::{AppResult, DashboardStats};

pub async fn get_stats(
    State(state): State<AppState>,
) -> AppResult<Json<DashboardStats>> {
    let total_products = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM products WHERE is_active = true"
    ).fetch_one(&state.db).await?.unwrap_or(0);

    let total_categories = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM categories"
    ).fetch_one(&state.db).await?.unwrap_or(0);

    let total_suppliers = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM suppliers"
    ).fetch_one(&state.db).await?.unwrap_or(0);

    let low_stock_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM products WHERE is_active = true AND quantity_in_stock > 0 AND quantity_in_stock <= reorder_level"
    ).fetch_one(&state.db).await?.unwrap_or(0);

    let out_of_stock_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM products WHERE is_active = true AND quantity_in_stock = 0"
    ).fetch_one(&state.db).await?.unwrap_or(0);

    let total_stock_value = sqlx::query_scalar!(
        "SELECT COALESCE(SUM(unit_price * quantity_in_stock::float8), 0.0) FROM products WHERE is_active = true"
    ).fetch_one(&state.db).await?.unwrap_or(0.0);

    let total_movements_today = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM stock_movements WHERE created_at >= CURRENT_DATE"
    ).fetch_one(&state.db).await?.unwrap_or(0);

    Ok(Json(DashboardStats {
        total_products,
        total_categories,
        total_suppliers,
        low_stock_count,
        out_of_stock_count,
        total_stock_value,
        total_movements_today,
    }))
}
