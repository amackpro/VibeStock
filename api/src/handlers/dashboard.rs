use axum::{extract::{Extension, State}, Json};
use uuid::Uuid;

use crate::AppState;
use shared::{AppResult, DashboardStats};

/// GET /api/dashboard/stats — returns 6 inventory KPI aggregates
pub async fn get_stats(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<DashboardStats>> {
    let (
        total_products,
        total_categories,
        total_suppliers,
        low_stock_count,
        out_of_stock_count,
        total_stock_value,
        total_movements_today
    ): (i64, i64, i64, i64, i64, f64, i64) = sqlx::query_as(
        "SELECT 
            (SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND is_active = true) as total_products,
            (SELECT COUNT(*) FROM categories WHERE tenant_id = $1) as total_categories,
            (SELECT COUNT(*) FROM suppliers WHERE tenant_id = $1) as total_suppliers,
            (SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND is_active = true AND quantity_in_stock > 0 AND quantity_in_stock <= reorder_level) as low_stock_count,
            (SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND is_active = true AND quantity_in_stock = 0) as out_of_stock_count,
            (SELECT COALESCE(SUM(unit_price * quantity_in_stock::float8), 0.0) FROM products WHERE tenant_id = $1 AND is_active = true) as total_stock_value,
            (SELECT COUNT(*) FROM stock_movements WHERE tenant_id = $1 AND created_at >= CURRENT_DATE) as total_movements_today"
    )
    .bind(tenant_id)
    .fetch_one(&state.db)
    .await?;

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
