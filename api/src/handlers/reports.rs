use axum::extract::{Extension, Query, State};
use axum::Json;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use shared::{AppResult, ProductWithDetails, StockMovementWithDetails};

#[derive(Debug, Deserialize)]
pub struct ReportParams {
    #[serde(default)]
    pub category_id: Option<Uuid>,
    #[serde(default)]
    pub city_id: Option<Uuid>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    // Frontend sends date_from/date_to
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    // Region/Country/Category filters for frontend
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
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

/// GET /api/reports/inventory
pub async fn get_inventory_report(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Query(params): Query<ReportParams>,
) -> AppResult<Json<InventoryReport>> {
    // Build dynamic WHERE clause
    let mut conditions = vec![
        "p.tenant_id = $1".to_string(),
        "p.is_active = true".to_string(),
    ];
    let mut param_idx = 2;

    if params.category_id.is_some() { 
        conditions.push(format!("p.category_id = ${}", param_idx));
        param_idx += 1;
    }
    if params.city_id.is_some() { 
        conditions.push(format!("s.city_id = ${}", param_idx));
        param_idx += 1;
    }

    // Add region/country/category text filters
    let mut joins = String::new();
    
    if params.region.is_some() || params.country.is_some() {
        joins.push_str(" LEFT JOIN cities ci ON s.city_id = ci.id");
        joins.push_str(" LEFT JOIN countries co ON ci.country_id = co.id");
        joins.push_str(" LEFT JOIN regions r ON co.region_id = r.id");
    }
    if params.category.is_some() || params.category_id.is_some() {
        // categories already joined
    }

    if let Some(ref region) = params.region {
        if !region.is_empty() {
            conditions.push(format!("r.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }
    if let Some(ref country) = params.country {
        if !country.is_empty() {
            conditions.push(format!("co.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }
    if let Some(ref cat) = params.category {
        if !cat.is_empty() {
            conditions.push(format!("c.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        "SELECT p.id, p.tenant_id, p.name, p.description, p.sku, p.barcode,
                p.category_id, c.name as category_name,
                p.supplier_id, s.name as supplier_name,
                p.unit_price, p.cost_price, p.quantity_in_stock, p.reorder_level,
                p.unit_of_measure, p.is_active, p.image_url, p.created_at, p.updated_at
         FROM products p
         LEFT JOIN categories c ON p.category_id = c.id
         LEFT JOIN suppliers  s ON p.supplier_id  = s.id
         {}
         WHERE {}
         ORDER BY p.name",
        joins, where_clause
    );

    let mut q = sqlx::query_as::<_, ProductWithDetails>(&sql).bind(tenant_id);
    if let Some(cid) = params.category_id { q = q.bind(cid); }
    if let Some(city) = params.city_id { q = q.bind(city); }
    if let Some(ref region) = params.region {
        if !region.is_empty() { q = q.bind(format!("%{}%", region)); }
    }
    if let Some(ref country) = params.country {
        if !country.is_empty() { q = q.bind(format!("%{}%", country)); }
    }
    if let Some(ref cat) = params.category {
        if !cat.is_empty() { q = q.bind(format!("%{}%", cat)); }
    }

    let products: Vec<ProductWithDetails> = q.fetch_all(&state.db).await?;

    let items: Vec<InventoryReportItem> = products.into_iter().map(|p| InventoryReportItem {
        total_value: p.unit_price * p.quantity_in_stock as f64,
        sku: p.sku, name: p.name, category: p.category_name, supplier: p.supplier_name,
        quantity_in_stock: p.quantity_in_stock, reorder_level: p.reorder_level,
        unit_price: p.unit_price,
    }).collect();

    let total_items = items.len() as i64;
    let total_value: f64 = items.iter().map(|i| i.total_value).sum();

    Ok(Json(InventoryReport {
        generated_at: Utc::now().to_rfc3339(),
        report_type: "inventory".to_string(),
        items, total_items, total_value,
    }))
}

/// GET /api/reports/low-stock
pub async fn get_low_stock_report(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Query(params): Query<ReportParams>,
) -> AppResult<Json<LowStockReport>> {
    let mut conditions = vec![
        "p.tenant_id = $1".to_string(),
        "p.is_active = true".to_string(),
        "p.quantity_in_stock <= p.reorder_level".to_string(),
    ];
    let mut param_idx = 2;

    if params.category_id.is_some() { 
        conditions.push(format!("p.category_id = ${}", param_idx));
        param_idx += 1;
    }
    if params.city_id.is_some() { 
        conditions.push(format!("s.city_id = ${}", param_idx));
        param_idx += 1;
    }

    let mut joins = String::new();
    if params.region.is_some() || params.country.is_some() {
        joins.push_str(" LEFT JOIN cities ci ON s.city_id = ci.id");
        joins.push_str(" LEFT JOIN countries co ON ci.country_id = co.id");
        joins.push_str(" LEFT JOIN regions r ON co.region_id = r.id");
    }

    if let Some(ref region) = params.region {
        if !region.is_empty() {
            conditions.push(format!("r.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }
    if let Some(ref country) = params.country {
        if !country.is_empty() {
            conditions.push(format!("co.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }
    if let Some(ref cat) = params.category {
        if !cat.is_empty() {
            conditions.push(format!("c.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        "SELECT p.id, p.tenant_id, p.name, p.description, p.sku, p.barcode,
                p.category_id, c.name as category_name,
                p.supplier_id, s.name as supplier_name,
                p.unit_price, p.cost_price, p.quantity_in_stock, p.reorder_level,
                p.unit_of_measure, p.is_active, p.image_url, p.created_at, p.updated_at
         FROM products p
         LEFT JOIN categories c ON p.category_id = c.id
         LEFT JOIN suppliers  s ON p.supplier_id  = s.id
         {}
         WHERE {}
         ORDER BY p.quantity_in_stock ASC, p.name",
        joins, where_clause
    );

    let mut q = sqlx::query_as::<_, ProductWithDetails>(&sql).bind(tenant_id);
    if let Some(cid) = params.category_id { q = q.bind(cid); }
    if let Some(city) = params.city_id { q = q.bind(city); }
    if let Some(ref region) = params.region {
        if !region.is_empty() { q = q.bind(format!("%{}%", region)); }
    }
    if let Some(ref country) = params.country {
        if !country.is_empty() { q = q.bind(format!("%{}%", country)); }
    }
    if let Some(ref cat) = params.category {
        if !cat.is_empty() { q = q.bind(format!("%{}%", cat)); }
    }

    let products: Vec<ProductWithDetails> = q.fetch_all(&state.db).await?;

    let items: Vec<LowStockReportItem> = products.into_iter().map(|p| LowStockReportItem {
        sku: p.sku, name: p.name, category: p.category_name, supplier: p.supplier_name,
        quantity_in_stock: p.quantity_in_stock, reorder_level: p.reorder_level,
        unit_price: p.unit_price,
    }).collect();

    let total_items = items.len() as i64;

    Ok(Json(LowStockReport {
        generated_at: Utc::now().to_rfc3339(),
        report_type: "low_stock".to_string(),
        items, total_items,
    }))
}

/// GET /api/reports/movements
pub async fn get_movements_report(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Query(params): Query<ReportParams>,
) -> AppResult<Json<MovementReport>> {
    let date_from = params.date_from.as_deref()
        .or(params.start_date.as_deref())
        .unwrap_or("1970-01-01");
    let date_to = params.date_to.as_deref()
        .or(params.end_date.as_deref())
        .unwrap_or("2099-12-31");

    let mut conditions = vec![
        "m.tenant_id = $1".to_string(),
        format!("m.created_at >= $2::date"),
        format!("m.created_at < ($3::date + interval '1 day')"),
    ];
    let mut param_idx = 4;

    let mut joins = String::from("JOIN products p ON m.product_id = p.id\n         JOIN users u ON m.performed_by = u.id\n         LEFT JOIN categories c ON p.category_id = c.id\n         LEFT JOIN suppliers s ON p.supplier_id = s.id\n         LEFT JOIN cities ci ON s.city_id = ci.id\n         LEFT JOIN countries co ON ci.country_id = co.id\n         LEFT JOIN regions r ON co.region_id = r.id");

    if let Some(ref region) = params.region {
        if !region.is_empty() {
            conditions.push(format!("r.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }
    if let Some(ref country) = params.country {
        if !country.is_empty() {
            conditions.push(format!("co.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }
    if let Some(ref cat) = params.category {
        if !cat.is_empty() {
            conditions.push(format!("c.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }

    let where_clause = conditions.join(" AND ");
    
    let sql = format!(
        "SELECT m.id, m.tenant_id, m.product_id, p.name as product_name, p.sku as product_sku,
                m.movement_type::text as movement_type, m.quantity, m.reference, m.notes,
                m.performed_by, u.full_name as performed_by_name, m.created_at
         FROM stock_movements m
         {}
         WHERE {}
         ORDER BY m.created_at DESC
         LIMIT 2000",
        joins, where_clause
    );

    let mut q = sqlx::query_as::<_, StockMovementWithDetails>(&sql)
        .bind(tenant_id)
        .bind(date_from)
        .bind(date_to);
    
    if let Some(ref region) = params.region {
        if !region.is_empty() { q = q.bind(format!("%{}%", region)); }
    }
    if let Some(ref country) = params.country {
        if !country.is_empty() { q = q.bind(format!("%{}%", country)); }
    }
    if let Some(ref cat) = params.category {
        if !cat.is_empty() { q = q.bind(format!("%{}%", cat)); }
    }

    let movements = q.fetch_all(&state.db).await?;

    let items: Vec<MovementReportItem> = movements.into_iter().map(|m| MovementReportItem {
        date: m.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        product_sku: m.product_sku, product_name: m.product_name,
        movement_type: m.movement_type, quantity: m.quantity,
        reference: m.reference, performed_by: m.performed_by_name,
    }).collect();

    let total_items = items.len() as i64;

    Ok(Json(MovementReport {
        generated_at: Utc::now().to_rfc3339(),
        report_type: "movements".to_string(),
        items, total_items,
    }))
}

/// GET /api/reports/valuation
pub async fn get_valuation_report(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Query(params): Query<ReportParams>,
) -> AppResult<Json<ValuationReport>> {
    let mut conditions = vec![
        "p.tenant_id = $1".to_string(),
        "p.is_active = true".to_string(),
    ];
    let mut param_idx = 2;

    if params.category_id.is_some() { 
        conditions.push(format!("p.category_id = ${}", param_idx));
        param_idx += 1;
    }
    if params.city_id.is_some() { 
        conditions.push(format!("s.city_id = ${}", param_idx));
        param_idx += 1;
    }

    let mut joins = String::new();
    if params.region.is_some() || params.country.is_some() {
        joins.push_str(" LEFT JOIN cities ci ON s.city_id = ci.id");
        joins.push_str(" LEFT JOIN countries co ON ci.country_id = co.id");
        joins.push_str(" LEFT JOIN regions r ON co.region_id = r.id");
    }

    if let Some(ref region) = params.region {
        if !region.is_empty() {
            conditions.push(format!("r.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }
    if let Some(ref country) = params.country {
        if !country.is_empty() {
            conditions.push(format!("co.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }
    if let Some(ref cat) = params.category {
        if !cat.is_empty() {
            conditions.push(format!("c.name ILIKE ${}", param_idx));
            param_idx += 1;
        }
    }

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        "SELECT c.name as category_name,
                SUM(p.quantity_in_stock)::int8 as total_quantity,
                COALESCE(SUM(p.cost_price  * p.quantity_in_stock::float8), 0.0) as total_cost_value,
                COALESCE(SUM(p.unit_price  * p.quantity_in_stock::float8), 0.0) as total_retail_value
         FROM products p
         LEFT JOIN categories c ON p.category_id = c.id
         LEFT JOIN suppliers  s ON p.supplier_id  = s.id
         {}
         WHERE {}
         GROUP BY c.name
         ORDER BY total_retail_value DESC",
        joins, where_clause
    );

    let mut q = sqlx::query_as::<_, (Option<String>, i64, f64, f64)>(&sql).bind(tenant_id);
    if let Some(cid) = params.category_id { q = q.bind(cid); }
    if let Some(city) = params.city_id { q = q.bind(city); }
    if let Some(ref region) = params.region {
        if !region.is_empty() { q = q.bind(format!("%{}%", region)); }
    }
    if let Some(ref country) = params.country {
        if !country.is_empty() { q = q.bind(format!("%{}%", country)); }
    }
    if let Some(ref cat) = params.category {
        if !cat.is_empty() { q = q.bind(format!("%{}%", cat)); }
    }

    let items: Vec<(Option<String>, i64, f64, f64)> = q.fetch_all(&state.db).await?;

    let report_items: Vec<ValuationReportItem> = items.into_iter().map(|(category, qty, cost, retail)| {
        ValuationReportItem {
            category,
            total_quantity: qty as i32,
            total_cost_value: cost,
            total_retail_value: retail,
        }
    }).collect();

    let total_quantity: i32    = report_items.iter().map(|i| i.total_quantity).sum();
    let total_cost_value: f64  = report_items.iter().map(|i| i.total_cost_value).sum();
    let total_retail_value: f64= report_items.iter().map(|i| i.total_retail_value).sum();

    Ok(Json(ValuationReport {
        generated_at: Utc::now().to_rfc3339(),
        report_type: "valuation".to_string(),
        items: report_items,
        total_quantity, total_cost_value, total_retail_value,
    }))
}
