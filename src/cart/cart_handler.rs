use crate::AppState;
use crate::app_error::AppError;
use crate::cart::cart_repository::{
    create_cart_item, delete_cart_item, get_cart, update_cart_item_quantity,
};
use crate::cart_models::{CartItem, CreateCartItem, UpdateCartItem};
use crate::user::auth::AuthUser;
use axum::Json;
use axum::extract::{Path, State};

pub async fn create_cart_item_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(cart_item): Json<CreateCartItem>,
) -> Result<Json<CartItem>, AppError> {
    let cart_item = create_cart_item(&state.pool, auth_user.id, cart_item).await?;
    Ok(Json(cart_item))
}

pub async fn get_cart_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<CartItem>>, AppError> {
    let cart = get_cart(&state.pool, auth_user.id).await?;
    Ok(Json(cart))
}

pub async fn update_cart_item_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(item_id): Path<i64>,
    Json(update): Json<UpdateCartItem>,
) -> Result<Json<CartItem>, AppError> {
    let cart_item =
        update_cart_item_quantity(&state.pool, auth_user.id, item_id, update.quantity).await?;
    Ok(Json(cart_item))
}

pub async fn delete_cart_item_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(item_id): Path<i64>,
) -> Result<Json<CartItem>, AppError> {
    let cart_item = delete_cart_item(&state.pool, auth_user.id, item_id).await?;
    Ok(Json(cart_item))
}
