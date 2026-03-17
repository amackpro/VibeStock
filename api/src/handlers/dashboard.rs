use axum::{extract::{Extension, State}, Json};
use uuid::Uuid;

use crate::AppState;
use shared::{AppResult, DashboardStats};

/// GET /api/dashboard/stats — returns 6 inventory KPI aggregates
pub async fn get_stats(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<DashboardStats>> {
    let total_products: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND is_active = true"
    ).bind(tenant_id).fetch_one(&state.db).await?;

    let total_categories: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM categories WHERE tenant_id = $1"
    ).bind(tenant_id).fetch_one(&state.db).await?;

    let total_suppliers: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM suppliers WHERE tenant_id = $1"
    ).bind(tenant_id).fetch_one(&state.db).await?;

    let low_stock_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products \
         WHERE tenant_id = $1 AND is_active = true AND quantity_in_stock > 0 AND quantity_in_stock <= reorder_level"
    ).bind(tenant_id).fetch_one(&state.db).await?;

    let out_of_stock_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND is_active = true AND quantity_in_stock = 0"
    ).bind(tenant_id).fetch_one(&state.db).await?;

    let total_stock_value: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(unit_price * quantity_in_stock::float8), 0.0) \
         FROM products WHERE tenant_id = $1 AND is_active = true"
    ).bind(tenant_id).fetch_one(&state.db).await?;

    let total_movements_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM stock_movements WHERE tenant_id = $1 AND created_at >= CURRENT_DATE"
    ).bind(tenant_id).fetch_one(&state.db).await?;

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
