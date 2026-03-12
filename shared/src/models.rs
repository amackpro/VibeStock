use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ─── Category ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Supplier ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Supplier {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Product ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub barcode: Option<String>,
    pub category_id: Option<Uuid>,
    pub supplier_id: Option<Uuid>,
    pub unit_price: f64,
    pub cost_price: f64,
    pub quantity_in_stock: i32,
    pub reorder_level: i32,
    pub unit_of_measure: String,
    pub is_active: bool,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Product with relations ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductWithDetails {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub barcode: Option<String>,
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub supplier_id: Option<Uuid>,
    pub supplier_name: Option<String>,
    pub unit_price: f64,
    pub cost_price: f64,
    pub quantity_in_stock: i32,
    pub reorder_level: i32,
    pub unit_of_measure: String,
    pub is_active: bool,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── StockMovement ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "movement_type", rename_all = "lowercase")]
pub enum MovementType {
    In,
    Out,
    Adjustment,
    Return,
}

impl std::fmt::Display for MovementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MovementType::In => write!(f, "in"),
            MovementType::Out => write!(f, "out"),
            MovementType::Adjustment => write!(f, "adjustment"),
            MovementType::Return => write!(f, "return"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StockMovement {
    pub id: Uuid,
    pub product_id: Uuid,
    pub movement_type: MovementType,
    pub quantity: i32,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub performed_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// movement_type is stored as String (cast from PG enum via ::text)
/// so that runtime sqlx::query_as() works without DB type registration.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StockMovementWithDetails {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_sku: String,
    pub movement_type: String,   // "in" | "out" | "adjustment" | "return"
    pub quantity: i32,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub performed_by: Uuid,
    pub performed_by_name: String,
    pub created_at: DateTime<Utc>,
}

// ─── User ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Manager,
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: String,
    pub role: String,            // stored as text for runtime query compatibility
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Dashboard Stats ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_products: i64,
    pub total_categories: i64,
    pub total_suppliers: i64,
    pub low_stock_count: i64,
    pub out_of_stock_count: i64,
    pub total_stock_value: f64,
    pub total_movements_today: i64,
}
