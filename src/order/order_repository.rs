use crate::order_models::{CheckoutCartItem, Order, OrderDetails, OrderItem};
use sqlx::{PgPool, Postgres, Transaction};

pub async fn lock_cart_owner(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE id = $1 FOR UPDATE")
        .bind(user_id)
        .fetch_one(&mut **transaction)
        .await?;
    Ok(())
}

pub async fn create_pending_order(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i64,
    idempotency_key: Option<&str>,
) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        INSERT INTO orders (user_id, status, idempotency_key, expires_at)
        VALUES ($1, 'pending_payment', $2, NOW() + INTERVAL '15 minutes')
        ON CONFLICT (user_id, idempotency_key) DO NOTHING
        RETURNING id, user_id, status, expires_at, paid_at, cancelled_at, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(idempotency_key)
    .fetch_optional(&mut **transaction)
    .await
}

pub async fn find_order_by_idempotency_key(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i64,
    idempotency_key: &str,
) -> Result<Order, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        SELECT id, user_id, status, expires_at, paid_at, cancelled_at, created_at, updated_at
        FROM orders
        WHERE user_id = $1 AND idempotency_key = $2
        "#,
    )
    .bind(user_id)
    .bind(idempotency_key)
    .fetch_one(&mut **transaction)
    .await
}

pub async fn lock_cart_for_checkout(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i64,
) -> Result<Vec<CheckoutCartItem>, sqlx::Error> {
    sqlx::query_as::<_, CheckoutCartItem>(
        r#"
        SELECT p.id AS product_id,
               p.name AS product_name,
               p.price AS unit_price,
               ci.quantity,
               p.stock,
               p.reserved_stock
        FROM cart_items AS ci
        JOIN products AS p ON p.id = ci.product_id
        WHERE ci.user_id = $1
        ORDER BY p.id
        FOR UPDATE OF ci, p
        "#,
    )
    .bind(user_id)
    .fetch_all(&mut **transaction)
    .await
}

pub async fn create_order_item(
    transaction: &mut Transaction<'_, Postgres>,
    order_id: i64,
    item: &CheckoutCartItem,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO order_items (order_id, product_id, product_name, unit_price, quantity)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(order_id)
    .bind(item.product_id)
    .bind(&item.product_name)
    .bind(item.unit_price)
    .bind(item.quantity)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

pub async fn clear_cart(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM cart_items WHERE user_id = $1")
        .bind(user_id)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

pub async fn list_orders(pool: &PgPool, user_id: i64) -> Result<Vec<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        SELECT id, user_id, status, expires_at, paid_at, cancelled_at, created_at, updated_at
        FROM orders
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_order_details(
    pool: &PgPool,
    user_id: i64,
    order_id: i64,
) -> Result<OrderDetails, sqlx::Error> {
    let order = sqlx::query_as::<_, Order>(
        r#"
        SELECT id, user_id, status, expires_at, paid_at, cancelled_at, created_at, updated_at
        FROM orders
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(order_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    let items = get_order_items(pool, order_id).await?;
    let total = items.iter().map(|item| item.line_total).sum();

    Ok(OrderDetails {
        order,
        items,
        total,
    })
}

pub async fn lock_owned_order(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i64,
    order_id: i64,
) -> Result<Order, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        SELECT id, user_id, status, expires_at, paid_at, cancelled_at, created_at, updated_at
        FROM orders
        WHERE id = $1 AND user_id = $2
        FOR UPDATE
        "#,
    )
    .bind(order_id)
    .bind(user_id)
    .fetch_one(&mut **transaction)
    .await
}

pub async fn lock_next_expired_order(
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        SELECT id, user_id, status, expires_at, paid_at, cancelled_at, created_at, updated_at
        FROM orders
        WHERE status = 'pending_payment'
          AND expires_at <= NOW()
        ORDER BY expires_at, id
        LIMIT 1
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .fetch_optional(&mut **transaction)
    .await
}

pub async fn mark_order_paid(
    transaction: &mut Transaction<'_, Postgres>,
    order_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE orders
        SET status = 'paid', paid_at = NOW(), updated_at = NOW()
        WHERE id = $1 AND status = 'pending_payment'
        "#,
    )
    .bind(order_id)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

pub async fn mark_order_cancelled(
    transaction: &mut Transaction<'_, Postgres>,
    order_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE orders
        SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW()
        WHERE id = $1 AND status = 'pending_payment'
        "#,
    )
    .bind(order_id)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

pub async fn mark_order_expired(
    transaction: &mut Transaction<'_, Postgres>,
    order_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE orders
        SET status = 'expired', updated_at = NOW()
        WHERE id = $1 AND status = 'pending_payment'
        "#,
    )
    .bind(order_id)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn get_order_items(pool: &PgPool, order_id: i64) -> Result<Vec<OrderItem>, sqlx::Error> {
    sqlx::query_as::<_, OrderItem>(
        r#"
        SELECT id,
               product_id,
               product_name,
               unit_price,
               quantity,
               unit_price * quantity::BIGINT AS line_total
        FROM order_items
        WHERE order_id = $1
        ORDER BY id
        "#,
    )
    .bind(order_id)
    .fetch_all(pool)
    .await
}
