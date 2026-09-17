use crate::AppState;
use crate::app_error::AppError;
use crate::cart::cart_repository::{
    cart_item_exists, create_cart_item, delete_cart_item, get_cart, update_cart_item_quantity,
};
use crate::cart_models::{CartItem, CartResponse, CreateCartItem, UpdateCartItem};
use crate::user::auth::AuthUser;
use axum::Json;
use axum::extract::{Path, State};

pub async fn create_cart_item_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(cart_item): Json<CreateCartItem>,
) -> Result<Json<CartItem>, AppError> {
    validate_quantity(cart_item.quantity)?;
    let cart_item = create_cart_item(&state.pool, auth_user.id, cart_item)
        .await?
        .ok_or_else(|| {
            AppError::Validation(
                "Product does not exist or requested quantity exceeds available stock".to_string(),
            )
        })?;
    Ok(Json(cart_item))
}

pub async fn get_cart_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<CartResponse>, AppError> {
    let items = get_cart(&state.pool, auth_user.id).await?;
    let total = items.iter().map(|item| item.line_total).sum();
    Ok(Json(CartResponse { items, total }))
}

pub async fn update_cart_item_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(item_id): Path<i64>,
    Json(update): Json<UpdateCartItem>,
) -> Result<Json<CartItem>, AppError> {
    validate_quantity(update.quantity)?;
    let cart_item =
        update_cart_item_quantity(&state.pool, auth_user.id, item_id, update.quantity).await?;

    let cart_item = match cart_item {
        Some(cart_item) => cart_item,
        None if cart_item_exists(&state.pool, auth_user.id, item_id).await? => {
            return Err(AppError::Validation(
                "Requested quantity exceeds available product stock".to_string(),
            ));
        }
        None => return Err(AppError::NotFound),
    };

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

fn validate_quantity(quantity: i32) -> Result<(), AppError> {
    if quantity > 0 {
        Ok(())
    } else {
        Err(AppError::BadRequest(
            "Quantity must be greater than zero".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::validate_quantity;

    #[test]
    fn accepts_positive_quantity() {
        assert!(validate_quantity(1).is_ok());
    }

    #[test]
    fn rejects_zero_quantity() {
        assert!(validate_quantity(0).is_err());
    }

    #[test]
    fn rejects_negative_quantity() {
        assert!(validate_quantity(-1).is_err());
    }
}
