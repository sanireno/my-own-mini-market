
use sqlx::PgPool;
use crate::cart_models::{CartItem, CreateCartItem};

//3. Сделать операции с корзиной
// Минимальный набор:
// POST   /cart/items       добавить товар
// GET    /cart             получить корзину
// PATCH  /cart/items/:id   изменить quantity
// DELETE /cart/items/:id   удалить товар
pub async fn create_cart(pool:PgPool,cart_item: CreateCartItem)->Result<CartItem, sqlx::Error>{
    sqlx::query_as!(
        CartItem,
        r#"
        INSERT INTO cart_items(user_id,product_id,quantity,created_at,updated_at)
        VALUES ($1,$2,$3,$4,$5)
        RETURNING id, name, description, price, stock,category_id
        "#,
        cart_item.user_id,
        cart_item.product_id,
        cart_item.quantity,
        cart_item.created_at,
        cart_item.updated_at

    )
        .fetch_one(pool).await
}