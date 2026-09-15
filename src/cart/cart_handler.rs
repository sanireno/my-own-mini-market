use axum::extract::{Path, State};
use axum::Json;
use crate::app_error::AppError;
use crate::AppState;
use crate::cart::cart_repository::get_cart;
use crate::cart_models::CartItem;
use crate::user::auth::AuthUser;

pub async fn get_cart_handler(State(state):State<AppState>,auth_user: AuthUser) ->Result<Json<Vec<CartItem>>,AppError>{
    let id = auth_user.id;
    let cart=get_cart(&state.pool, id).await?;
    Ok(Json(cart))
}
