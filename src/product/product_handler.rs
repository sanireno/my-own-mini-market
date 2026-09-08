use crate::app_error::AppError;
use crate::product::product_repository::*;
use crate::products_models::{CreateProduct, Pagination, Product, UpdateProduct};
use axum::Json;
use axum::extract::{Path, Query, State};
use sqlx::PgPool;

pub async fn create_product_handler(
    State(pool): State<PgPool>,
    Json(products): Json<CreateProduct>,
) -> Result<Json<Product>, AppError> {
    let product = create_product(&pool, products).await?;
    Ok(Json(product))
}
pub async fn get_product_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, AppError> {
    let product = get_product(&pool, id).await?;
    Ok(Json(product))
}
pub async fn delete_product_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, AppError> {
    let product = delete_product(&pool, id).await?;
    Ok(Json(product))
}
pub async fn update_product_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(product): Json<UpdateProduct>,
) -> Result<Json<Product>, AppError> {
    let product = update_product(&pool, id, product).await?;
    Ok(Json(product))
}
pub async fn get_all_product_handler(
    State(pool): State<PgPool>,
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<Product>>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    let product = get_all_product(&pool, limit as i64, offset as i64).await?;
    Ok(Json(product))
}
