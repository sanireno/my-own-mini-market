use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use crate::category_models::{Category, CreateCategory, UpdateCategory};
use crate::products_models::Product;
use crate::category::category_repository::*;


pub async fn get_category_handler(pool:State<PgPool>, Path(id):Path<i64>) ->Result<Json<Category>,StatusCode>{
    let category=get_category(&pool,id).await.map_err(|e| {match e {
        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(category))
}
pub async fn create_category_handler(pool:State<PgPool>,Json(category):Json<CreateCategory>)->Result<Json<Category>,StatusCode>{
    let category=create_category(&pool,category).await.map_err(|e|
        {
            println!("ERROR:{}",e);
            StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(category))
}
pub async fn get_all_category_handler(pool:State<PgPool>)->Result<Json<Vec<Category>>,StatusCode>{
    println!("список всех категорий /get запрос");
    let category=get_all_categories(&pool).await.map_err(|e| {
        println!("ERROR:{}",e);
        StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(category))
}
pub async fn delete_category_handler(pool:State<PgPool>,Path(id):Path<i64>)->Result<Json<Category>,StatusCode>{
    println!("DELETE запрос пришёл, id = {}", id);
    let category=delete_category(&pool,id).await.map_err(|e| {match e {
        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(category))
}
pub async fn update_category_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(category): Json<UpdateCategory>,
) -> Result<Json<Category>, StatusCode> {
    let category = update_category(&pool, id, category)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(category))
}
pub async fn get_products_from_category_handler(State(pool):State<PgPool>,Path(id):Path<i64>)->Result<Json<Vec<Product>>,StatusCode>{
    let products=get_products_from_category(&pool,id).await
        .map_err(|e| {println!("ERROR:{}",e);StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(products))
}