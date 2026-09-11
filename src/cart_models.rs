#[derive(Debug, sqlx::FromRow)]
pub struct CartItem {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct CreateCartItem {
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}