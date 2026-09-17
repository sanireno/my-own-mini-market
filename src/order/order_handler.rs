use crate::AppState;
use crate::app_error::AppError;
use crate::order::order_repository::{get_order_details, list_orders};
use crate::order::order_service::{cancel_order, checkout, simulate_payment};
use crate::order_models::{Order, OrderDetails};
use crate::user::auth::AuthUser;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};

pub async fn checkout_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<OrderDetails>), AppError> {
    let idempotency_key = headers
        .get("Idempotency-Key")
        .map(|value| value.to_str())
        .transpose()
        .map_err(|_| AppError::BadRequest("Idempotency-Key must be valid ASCII".to_string()))?;

    if idempotency_key.is_some_and(|key| key.is_empty() || key.len() > 128) {
        return Err(AppError::BadRequest(
            "Idempotency-Key must contain 1 to 128 characters".to_string(),
        ));
    }

    let order = checkout(&state.pool, auth_user.id, idempotency_key).await?;
    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn list_orders_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Order>>, AppError> {
    Ok(Json(list_orders(&state.pool, auth_user.id).await?))
}

pub async fn get_order_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<i64>,
) -> Result<Json<OrderDetails>, AppError> {
    Ok(Json(
        get_order_details(&state.pool, auth_user.id, order_id).await?,
    ))
}

pub async fn cancel_order_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<i64>,
) -> Result<Json<OrderDetails>, AppError> {
    Ok(Json(
        cancel_order(&state.pool, auth_user.id, order_id).await?,
    ))
}

// TODO(real payments): remove this endpoint or replace it with payment-intent creation.
// A real provider must confirm payment through a verified webhook; the frontend must never
// be allowed to mark an order as paid directly.
pub async fn simulate_payment_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<i64>,
) -> Result<Json<OrderDetails>, AppError> {
    Ok(Json(
        simulate_payment(&state.pool, auth_user.id, order_id).await?,
    ))
}
