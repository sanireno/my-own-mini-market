use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CartItem {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CartItemDetails {
    pub id: i64,
    pub product_id: i64,
    pub name: String,
    pub price: i64,
    pub stock: i32,
    pub quantity: i32,
    pub line_total: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub items: Vec<CartItemDetails>,
    pub total: i64,
}

#[derive(Deserialize)]
pub struct CreateCartItem {
    pub product_id: i64,
    pub quantity: i32,
}

#[derive(Deserialize)]
pub struct UpdateCartItem {
    pub quantity: i32,
}
