use crate::app_error::AppError;
use crate::inventory::inventory_repository::{
    commit_order_stock, release_order_stock, reserve_stock,
};
use crate::order::order_repository::{
    clear_cart, create_order_item, create_pending_order, find_order_by_idempotency_key,
    get_order_details, lock_cart_for_checkout, lock_cart_owner, lock_next_expired_order,
    lock_owned_order, mark_order_cancelled, mark_order_expired, mark_order_paid,
};
use crate::order_models::OrderDetails;
use chrono::Utc;
use sqlx::PgPool;

const MAX_EXPIRATIONS_PER_RUN: usize = 100;

pub async fn checkout(
    pool: &PgPool,
    user_id: i64,
    idempotency_key: Option<&str>,
) -> Result<OrderDetails, AppError> {
    let mut transaction = pool.begin().await?;
    lock_cart_owner(&mut transaction, user_id).await?;
    let new_order = create_pending_order(&mut transaction, user_id, idempotency_key).await?;

    let order = match new_order {
        Some(order) => order,
        None => {
            let key = idempotency_key.ok_or(AppError::InternalServerError)?;
            let order = find_order_by_idempotency_key(&mut transaction, user_id, key).await?;
            transaction.commit().await?;
            return Ok(get_order_details(pool, user_id, order.id).await?);
        }
    };

    let cart = lock_cart_for_checkout(&mut transaction, user_id).await?;
    if cart.is_empty() {
        return Err(AppError::BadRequest("Cart is empty".to_string()));
    }

    for item in &cart {
        if item.quantity > item.stock - item.reserved_stock {
            return Err(AppError::ConflictMessage(format!(
                "Not enough available stock for product {}",
                item.product_id
            )));
        }
        reserve_stock(&mut transaction, item.product_id, item.quantity).await?;
        create_order_item(&mut transaction, order.id, item).await?;
    }

    clear_cart(&mut transaction, user_id).await?;
    transaction.commit().await?;
    Ok(get_order_details(pool, user_id, order.id).await?)
}

pub async fn cancel_order(
    pool: &PgPool,
    user_id: i64,
    order_id: i64,
) -> Result<OrderDetails, AppError> {
    let mut transaction = pool.begin().await?;
    let order = lock_owned_order(&mut transaction, user_id, order_id).await?;

    match order.status.as_str() {
        "cancelled" | "expired" => {
            transaction.commit().await?;
            return Ok(get_order_details(pool, user_id, order_id).await?);
        }
        "paid" => {
            return Err(AppError::ConflictMessage(
                "A paid order cannot be cancelled".to_string(),
            ));
        }
        "pending_payment" if order.expires_at <= Utc::now() => {
            release_order_stock(&mut transaction, order_id).await?;
            mark_order_expired(&mut transaction, order_id).await?;
        }
        "pending_payment" => {
            release_order_stock(&mut transaction, order_id).await?;
            mark_order_cancelled(&mut transaction, order_id).await?;
        }
        _ => return Err(AppError::InternalServerError),
    }

    transaction.commit().await?;
    Ok(get_order_details(pool, user_id, order_id).await?)
}

pub async fn simulate_payment(
    pool: &PgPool,
    user_id: i64,
    order_id: i64,
) -> Result<OrderDetails, AppError> {
    let mut transaction = pool.begin().await?;
    let order = lock_owned_order(&mut transaction, user_id, order_id).await?;

    match order.status.as_str() {
        "paid" => {
            transaction.commit().await?;
            return Ok(get_order_details(pool, user_id, order_id).await?);
        }
        "cancelled" | "expired" => {
            return Err(AppError::ConflictMessage(
                "Only a pending order can be paid".to_string(),
            ));
        }
        "pending_payment" if order.expires_at <= Utc::now() => {
            release_order_stock(&mut transaction, order_id).await?;
            mark_order_expired(&mut transaction, order_id).await?;
            transaction.commit().await?;
            return Err(AppError::ConflictMessage(
                "The order reservation has expired".to_string(),
            ));
        }
        "pending_payment" => {
            commit_order_stock(&mut transaction, order_id).await?;
            mark_order_paid(&mut transaction, order_id).await?;
        }
        _ => return Err(AppError::InternalServerError),
    }

    transaction.commit().await?;
    Ok(get_order_details(pool, user_id, order_id).await?)
}

pub async fn expire_pending_orders(pool: &PgPool) -> Result<usize, AppError> {
    let mut expired = 0;

    while expired < MAX_EXPIRATIONS_PER_RUN {
        let mut transaction = pool.begin().await?;
        let Some(order) = lock_next_expired_order(&mut transaction).await? else {
            transaction.rollback().await?;
            break;
        };

        release_order_stock(&mut transaction, order.id).await?;
        mark_order_expired(&mut transaction, order.id).await?;
        transaction.commit().await?;
        expired += 1;
    }

    Ok(expired)
}
