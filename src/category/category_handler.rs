use crate::AppState;
use crate::app_error::AppError;
use crate::category::category_repository::*;
use crate::category_models::{Category, CreateCategory, UpdateCategory};
use crate::products_models::Product;
use crate::user::auth::AuthUser;
use axum::Json;
use axum::extract::{Path, State};

pub async fn get_category_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Category>, AppError> {
    let category = get_category(&state.pool, id).await?;
    Ok(Json(category))
}
pub async fn create_category_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(category): Json<CreateCategory>,
) -> Result<Json<Category>, AppError> {
    auth_user.require_admin()?;
    let category = create_category(&state.pool, category).await?;
    Ok(Json(category))
}
pub async fn get_all_category_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Category>>, AppError> {
    let category = get_all_categories(&state.pool).await?;
    Ok(Json(category))
}
pub async fn delete_category_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Category>, AppError> {
    auth_user.require_admin()?;
    let category = delete_category(&state.pool, id).await?;
    Ok(Json(category))
}
pub async fn update_category_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(category): Json<UpdateCategory>,
) -> Result<Json<Category>, AppError> {
    auth_user.require_admin()?;
    let category = update_category(&state.pool, id, category).await?;

    Ok(Json(category))
}
pub async fn get_products_from_category_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = get_products_from_category(&state.pool, id).await?;
    Ok(Json(products))
}
