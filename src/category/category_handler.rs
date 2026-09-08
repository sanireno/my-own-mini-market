use crate::app_error::AppError;
use crate::category::category_repository::*;
use crate::category_models::{Category, CreateCategory, UpdateCategory};
use crate::products_models::Product;
use axum::Json;
use axum::extract::{Path, State};
use sqlx::PgPool;

pub async fn get_category_handler(
    pool: State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Category>, AppError> {
    let category = get_category(&pool, id).await?;
    Ok(Json(category))
}
pub async fn create_category_handler(
    pool: State<PgPool>,
    Json(category): Json<CreateCategory>,
) -> Result<Json<Category>, AppError> {
    let category = create_category(&pool, category).await?;
    Ok(Json(category))
}
pub async fn get_all_category_handler(
    pool: State<PgPool>,
) -> Result<Json<Vec<Category>>, AppError> {
    let category = get_all_categories(&pool).await?;
    Ok(Json(category))
}
pub async fn delete_category_handler(
    pool: State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Category>, AppError> {
    let category = delete_category(&pool, id).await?;
    Ok(Json(category))
}
pub async fn update_category_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(category): Json<UpdateCategory>,
) -> Result<Json<Category>, AppError> {
    let category = update_category(&pool, id, category).await?;

    Ok(Json(category))
}
pub async fn get_products_from_category_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = get_products_from_category(&pool, id).await?;
    Ok(Json(products))
}
