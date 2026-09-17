use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub status: String,
    pub expires_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct OrderItem {
    pub id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub unit_price: i64,
    pub quantity: i32,
    pub line_total: i64,
}

#[derive(Debug, Serialize)]
pub struct OrderDetails {
    #[serde(flatten)]
    pub order: Order,
    pub items: Vec<OrderItem>,
    pub total: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CheckoutCartItem {
    pub product_id: i64,
    pub product_name: String,
    pub unit_price: i64,
    pub quantity: i32,
    pub stock: i32,
    pub reserved_stock: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct InventoryQuantity {
    pub product_id: i64,
    pub quantity: i32,
}
