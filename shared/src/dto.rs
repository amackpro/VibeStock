use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ─── Auth DTOs ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1, max = 255))]
    pub full_name: String,
    pub tenant_id: Option<Uuid>,
    pub tenant_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTenantRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TenantInfo {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
    pub full_name: String,
    pub role: String,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub is_global_admin: bool,
    pub accessible_tenants: Vec<TenantInfo>,
}

// ─── Product DTOs ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub sku: String,
    pub barcode: Option<String>,
    pub category_id: Option<Uuid>,
    pub supplier_id: Option<Uuid>,
    #[validate(range(min = 0.0))]
    pub unit_price: f64,
    #[validate(range(min = 0.0))]
    pub cost_price: f64,
    #[validate(range(min = 0))]
    pub quantity_in_stock: i32,
    #[validate(range(min = 0))]
    pub reorder_level: i32,
    pub unit_of_measure: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub barcode: Option<String>,
    pub category_id: Option<Uuid>,
    pub supplier_id: Option<Uuid>,
    #[validate(range(min = 0.0))]
    pub unit_price: Option<f64>,
    #[validate(range(min = 0.0))]
    pub cost_price: Option<f64>,
    #[validate(range(min = 0))]
    pub reorder_level: Option<i32>,
    pub unit_of_measure: Option<String>,
    pub is_active: Option<bool>,
    pub image_url: Option<String>,
}

// ─── Category DTOs ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

// ─── Supplier DTOs ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSupplierRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub contact_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSupplierRequest {
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

// ─── Stock Movement DTOs ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStockMovementRequest {
    pub product_id: Uuid,
    pub movement_type: String,
    #[validate(range(min = 1))]
    pub quantity: i32,
    pub reference: Option<String>,
    pub notes: Option<String>,
}

// ─── Pagination ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page.unwrap_or(20);
        (page - 1) * per_page
    }
    pub fn limit(&self) -> i64 {
        self.per_page.unwrap_or(20).min(100)
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total + per_page - 1) / per_page;
        Self {
            data,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

// ─── WebSocket Events ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "payload")]
pub enum WsEvent {
    StockUpdated {
        product_id: Uuid,
        product_name: String,
        new_quantity: i32,
    },
    LowStock {
        product_id: Uuid,
        product_name: String,
        quantity: i32,
        reorder_level: i32,
    },
    NewMovement {
        product_id: Uuid,
        movement_type: String,
        quantity: i32,
    },
}
