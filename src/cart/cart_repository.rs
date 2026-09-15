use crate::cart_models::{CartItem, CartItemDetails, CreateCartItem};
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
) -> Result<Option<CartItem>, sqlx::Error> {
    sqlx::query_as::<_, CartItem>(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        SELECT $1, products.id, $3
        FROM products
        WHERE products.id = $2
          AND $3 > 0
          AND $3 <= products.stock
        ON CONFLICT (user_id, product_id) DO UPDATE
        SET quantity = cart_items.quantity + EXCLUDED.quantity,
            updated_at = NOW()
        WHERE cart_items.quantity + EXCLUDED.quantity <= (
            SELECT stock
            FROM products
            WHERE id = EXCLUDED.product_id
        )
        RETURNING id, user_id, product_id, quantity, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(cart_item.product_id)
    .bind(cart_item.quantity)
    .fetch_optional(pool)
    .await
}

pub async fn get_cart(pool: &PgPool, user_id: i64) -> Result<Vec<CartItemDetails>, sqlx::Error> {
    sqlx::query_as::<_, CartItemDetails>(
        r#"
        SELECT ci.id,
               ci.product_id,
               p.name,
               p.price,
               p.stock,
               ci.quantity,
               p.price * ci.quantity::BIGINT AS line_total,
               ci.created_at,
               ci.updated_at
        FROM cart_items AS ci
        JOIN products AS p ON p.id = ci.product_id
        WHERE ci.user_id = $1
        ORDER BY ci.id
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
) -> Result<Option<CartItem>, sqlx::Error> {
    sqlx::query_as::<_, CartItem>(
        r#"
        UPDATE cart_items AS ci
        SET quantity = $1, updated_at = NOW()
        FROM products AS p
        WHERE ci.id = $2
          AND ci.user_id = $3
          AND p.id = ci.product_id
          AND $1 > 0
          AND $1 <= p.stock
        RETURNING ci.id,
                  ci.user_id,
                  ci.product_id,
                  ci.quantity,
                  ci.created_at,
                  ci.updated_at
        "#,
    )
    .bind(quantity)
    .bind(item_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn cart_item_exists(
    pool: &PgPool,
    user_id: i64,
    item_id: i64,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM cart_items
            WHERE id = $1 AND user_id = $2
        )
        "#,
    )
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
