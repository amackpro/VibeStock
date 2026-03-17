use axum::{extract::{Extension, Query, State}, Json};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use shared::{AppResult, ProductWithDetails, StockMovementWithDetails};

#[derive(Debug, Deserialize)]
pub struct ReportParams {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct InventoryReportItem {
    pub sku: String,
    pub name: String,
    pub category: Option<String>,
    pub supplier: Option<String>,
    pub quantity_in_stock: i32,
    pub reorder_level: i32,
    pub unit_price: f64,
    pub total_value: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct LowStockReportItem {
    pub sku: String,
    pub name: String,
    pub category: Option<String>,
    pub supplier: Option<String>,
    pub quantity_in_stock: i32,
    pub reorder_level: i32,
    pub unit_price: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct MovementReportItem {
    pub date: String,
    pub product_sku: String,
    pub product_name: String,
    pub movement_type: String,
    pub quantity: i32,
    pub reference: Option<String>,
    pub performed_by: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ValuationReportItem {
    pub category: Option<String>,
    pub total_quantity: i32,
    pub total_cost_value: f64,
    pub total_retail_value: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct InventoryReport {
    pub generated_at: String,
    pub report_type: String,
    pub items: Vec<InventoryReportItem>,
    pub total_items: i64,
    pub total_value: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct LowStockReport {
    pub generated_at: String,
    pub report_type: String,
    pub items: Vec<LowStockReportItem>,
    pub total_items: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct MovementReport {
    pub generated_at: String,
    pub report_type: String,
    pub items: Vec<MovementReportItem>,
    pub total_items: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct ValuationReport {
    pub generated_at: String,
    pub report_type: String,
    pub items: Vec<ValuationReportItem>,
    pub total_quantity: i32,
    pub total_cost_value: f64,
    pub total_retail_value: f64,
}

/// GET /api/reports/inventory — Full inventory report
pub async fn get_inventory_report(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<InventoryReport>> {
    let products: Vec<ProductWithDetails> = sqlx::query_as(
        "SELECT p.id, p.tenant_id, p.name, p.description, p.sku, p.barcode,
                p.category_id, c.name as category_name,
                p.supplier_id, s.name as supplier_name,
                p.unit_price, p.cost_price, p.quantity_in_stock, p.reorder_level,
                p.unit_of_measure, p.is_active, p.image_url, p.created_at, p.updated_at
         FROM products p
         LEFT JOIN categories c ON p.category_id = c.id
         LEFT JOIN suppliers s ON p.supplier_id = s.id
         WHERE p.tenant_id = $1 AND p.is_active = true
         ORDER BY p.name"
    ).bind(tenant_id).fetch_all(&state.db).await?;

    let items: Vec<InventoryReportItem> = products.into_iter().map(|p| {
        InventoryReportItem {
            sku: p.sku,
            name: p.name,
            category: p.category_name,
            supplier: p.supplier_name,
            quantity_in_stock: p.quantity_in_stock,
            reorder_level: p.reorder_level,
            unit_price: p.unit_price,
            total_value: p.unit_price * p.quantity_in_stock as f64,
        }
    }).collect();

    let total_items = items.len() as i64;
    let total_value: f64 = items.iter().map(|i| i.total_value).sum();

    Ok(Json(InventoryReport {
        generated_at: Utc::now().to_rfc3339(),
        report_type: "inventory".to_string(),
        items,
        total_items,
        total_value,
    }))
}

/// GET /api/reports/low-stock — Low stock items report
pub async fn get_low_stock_report(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<LowStockReport>> {
    let products: Vec<ProductWithDetails> = sqlx::query_as(
        "SELECT p.id, p.tenant_id, p.name, p.description, p.sku, p.barcode,
                p.category_id, c.name as category_name,
                p.supplier_id, s.name as supplier_name,
                p.unit_price, p.cost_price, p.quantity_in_stock, p.reorder_level,
                p.unit_of_measure, p.is_active, p.image_url, p.created_at, p.updated_at
         FROM products p
         LEFT JOIN categories c ON p.category_id = c.id
         LEFT JOIN suppliers s ON p.supplier_id = s.id
         WHERE p.tenant_id = $1 AND p.is_active = true 
           AND p.quantity_in_stock <= p.reorder_level
         ORDER BY p.quantity_in_stock ASC, p.name"
    ).bind(tenant_id).fetch_all(&state.db).await?;

    let items: Vec<LowStockReportItem> = products.into_iter().map(|p| {
        LowStockReportItem {
            sku: p.sku,
            name: p.name,
            category: p.category_name,
            supplier: p.supplier_name,
            quantity_in_stock: p.quantity_in_stock,
            reorder_level: p.reorder_level,
            unit_price: p.unit_price,
        }
    }).collect();

    let total_items = items.len() as i64;

    Ok(Json(LowStockReport {
        generated_at: Utc::now().to_rfc3339(),
        report_type: "low_stock".to_string(),
        items,
        total_items,
    }))
}

/// GET /api/reports/movements — Stock movements report
pub async fn get_movements_report(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Query(params): Query<ReportParams>,
) -> AppResult<Json<MovementReport>> {
    let movements: Vec<StockMovementWithDetails> = sqlx::query_as(
        "SELECT m.id, m.tenant_id, m.product_id, p.name as product_name, p.sku as product_sku,
                m.movement_type::text as movement_type, m.quantity, m.reference, m.notes,
                m.performed_by, u.full_name as performed_by_name, m.created_at
         FROM stock_movements m
         JOIN products p ON m.product_id = p.id
         JOIN users u ON m.performed_by = u.id
         WHERE m.tenant_id = $1
         ORDER BY m.created_at DESC LIMIT 1000"
    ).bind(tenant_id).fetch_all(&state.db).await?;

    let items: Vec<MovementReportItem> = movements.into_iter().map(|m| {
        MovementReportItem {
            date: m.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            product_sku: m.product_sku,
            product_name: m.product_name,
            movement_type: m.movement_type,
            quantity: m.quantity,
            reference: m.reference,
            performed_by: m.performed_by_name,
        }
    }).collect();

    let total_items = items.len() as i64;

    Ok(Json(MovementReport {
        generated_at: Utc::now().to_rfc3339(),
        report_type: "movements".to_string(),
        items,
        total_items,
    }))
}

/// GET /api/reports/valuation — Inventory valuation report
pub async fn get_valuation_report(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<ValuationReport>> {
    let items: Vec<(Option<String>, i32, f64, f64)> = sqlx::query_as(
        "SELECT c.name as category_name,
                SUM(p.quantity_in_stock) as total_quantity,
                COALESCE(SUM(p.cost_price * p.quantity_in_stock::float8), 0.0) as total_cost_value,
                COALESCE(SUM(p.unit_price * p.quantity_in_stock::float8), 0.0) as total_retail_value
         FROM products p
         LEFT JOIN categories c ON p.category_id = c.id
         WHERE p.tenant_id = $1 AND p.is_active = true
         GROUP BY c.name
         ORDER BY total_retail_value DESC"
    ).bind(tenant_id).fetch_all(&state.db).await?;

    let report_items: Vec<ValuationReportItem> = items.into_iter().map(|(category, qty, cost, retail)| {
        ValuationReportItem {
            category,
            total_quantity: qty,
            total_cost_value: cost,
            total_retail_value: retail,
        }
    }).collect();

    let total_quantity: i32 = report_items.iter().map(|i| i.total_quantity).sum();
    let total_cost_value: f64 = report_items.iter().map(|i| i.total_cost_value).sum();
    let total_retail_value: f64 = report_items.iter().map(|i| i.total_retail_value).sum();

    Ok(Json(ValuationReport {
        generated_at: Utc::now().to_rfc3339(),
        report_type: "valuation".to_string(),
        items: report_items,
        total_quantity,
        total_cost_value,
        total_retail_value,
    }))
}
