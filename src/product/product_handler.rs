use crate::AppState;
use crate::app_error::AppError;
use crate::product::product_repository::*;
use crate::products_models::{CreateProduct, Pagination, Product, UpdateProduct};
use crate::user::auth::AuthUser;
use axum::Json;
use axum::extract::{Path, Query, State};

pub async fn create_product_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(products): Json<CreateProduct>,
) -> Result<Json<Product>, AppError> {
    auth_user.require_admin()?;
    let product = create_product(&state.pool, products).await?;
    Ok(Json(product))
}
pub async fn get_product_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, AppError> {
    let product = get_product(&state.pool, id).await?;
    Ok(Json(product))
}
pub async fn delete_product_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Product>, AppError> {
    auth_user.require_admin()?;
    let product = delete_product(&state.pool, id).await?;
    Ok(Json(product))
}
pub async fn update_product_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(product): Json<UpdateProduct>,
) -> Result<Json<Product>, AppError> {
    auth_user.require_admin()?;
    let product = update_product(&state.pool, id, product).await?;
    Ok(Json(product))
}
pub async fn get_all_product_handler(
    State(state): State<AppState>,
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<Product>>, AppError> {
    let (limit, offset) = params.limit_and_offset()?;
    let product = get_all_product(&state.pool, limit, offset).await?;
    Ok(Json(product))
}
