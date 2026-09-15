use crate::cart_models::{CartItem, CreateCartItem};
use sqlx::PgPool;

//3. Сделать операции с корзиной
// Минимальный набор:
// POST   /cart/items       добавить товар
// GET    /cart             получить корзину
// PATCH  /cart/items/:id   изменить quantity
// DELETE /cart/items/:id   удалить товар

pub async fn create_cart_item(
    pool: &PgPool,
    user_id: i64,
    cart_item: CreateCartItem,
) -> Result<CartItem, sqlx::Error> {
    sqlx::query_as::<_, CartItem>(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, product_id, quantity, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(cart_item.product_id)
    .bind(cart_item.quantity)
    .fetch_one(pool)
    .await
}

pub async fn get_cart(pool: &PgPool, user_id: i64) -> Result<Vec<CartItem>, sqlx::Error> {
    sqlx::query_as::<_, CartItem>(
        r#"
        SELECT id, user_id, product_id, quantity, created_at, updated_at
        FROM cart_items
        WHERE user_id = $1
        ORDER BY id
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn update_cart_item_quantity(
    pool: &PgPool,
    user_id: i64,
    item_id: i64,
    quantity: i32,
) -> Result<CartItem, sqlx::Error> {
    sqlx::query_as::<_, CartItem>(
        r#"
        UPDATE cart_items
        SET quantity = $1, updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        RETURNING id, user_id, product_id, quantity, created_at, updated_at
        "#,
    )
    .bind(quantity)
    .bind(item_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn delete_cart_item(
    pool: &PgPool,
    user_id: i64,
    item_id: i64,
) -> Result<CartItem, sqlx::Error> {
    sqlx::query_as::<_, CartItem>(
        r#"
        DELETE FROM cart_items
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, product_id, quantity, created_at, updated_at
        "#,
    )
    .bind(item_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}
