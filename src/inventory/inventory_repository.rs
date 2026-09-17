use crate::app_error::AppError;
use crate::order_models::InventoryQuantity;
use sqlx::{Postgres, Transaction};

pub async fn reserve_stock(
    transaction: &mut Transaction<'_, Postgres>,
    product_id: i64,
    quantity: i32,
) -> Result<(), AppError> {
    let updated = sqlx::query_scalar::<_, i64>(
        r#"
        UPDATE products
        SET reserved_stock = reserved_stock + $1
        WHERE id = $2
          AND stock - reserved_stock >= $1
        RETURNING id
        "#,
    )
    .bind(quantity)
    .bind(product_id)
    .fetch_optional(&mut **transaction)
    .await?;

    if updated.is_none() {
        return Err(AppError::ConflictMessage(
            "A product no longer has enough available stock".to_string(),
        ));
    }

    Ok(())
}

pub async fn release_order_stock(
    transaction: &mut Transaction<'_, Postgres>,
    order_id: i64,
) -> Result<(), AppError> {
    for item in order_inventory(transaction, order_id).await? {
        let updated = sqlx::query_scalar::<_, i64>(
            r#"
            UPDATE products
            SET reserved_stock = reserved_stock - $1
            WHERE id = $2
              AND reserved_stock >= $1
            RETURNING id
            "#,
        )
        .bind(item.quantity)
        .bind(item.product_id)
        .fetch_optional(&mut **transaction)
        .await?;

        if updated.is_none() {
            return Err(AppError::InternalServerError);
        }
    }

    Ok(())
}

pub async fn commit_order_stock(
    transaction: &mut Transaction<'_, Postgres>,
    order_id: i64,
) -> Result<(), AppError> {
    for item in order_inventory(transaction, order_id).await? {
        let updated = sqlx::query_scalar::<_, i64>(
            r#"
            UPDATE products
            SET stock = stock - $1,
                reserved_stock = reserved_stock - $1
            WHERE id = $2
              AND stock >= $1
              AND reserved_stock >= $1
            RETURNING id
            "#,
        )
        .bind(item.quantity)
        .bind(item.product_id)
        .fetch_optional(&mut **transaction)
        .await?;

        if updated.is_none() {
            return Err(AppError::InternalServerError);
        }
    }

    Ok(())
}

async fn order_inventory(
    transaction: &mut Transaction<'_, Postgres>,
    order_id: i64,
) -> Result<Vec<InventoryQuantity>, sqlx::Error> {
    sqlx::query_as::<_, InventoryQuantity>(
        r#"
        SELECT product_id, quantity
        FROM order_items
        WHERE order_id = $1
        ORDER BY product_id
        "#,
    )
    .bind(order_id)
    .fetch_all(&mut **transaction)
    .await
}
