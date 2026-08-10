use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use crate::products_models::{CreateProduct, Pagination, Product, UpdateProduct};
use crate::product::product_repository::*;
pub async fn create_product_handler(State(pool):State<PgPool>, Json(products):Json<CreateProduct>) ->Result<Json<Product>,StatusCode>{
    let product =create_product(&pool,products).await.map_err(|e| {
        println!("ERROR:{}",e);
        StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(product))
}
pub async fn get_product_handler(State(pool):State<PgPool>,Path(id):Path<i64>)->Result<Json<Product>,StatusCode>{
    let product=get_product(&pool,id).await.map_err(|e| {match e{

        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(product))
}
pub async fn delete_product_handler(State(pool):State<PgPool>,Path(id):Path<i64>)->Result<Json<Product>,StatusCode>{
    let product=delete_product(&pool,id).await.map_err(|e| {match e {
        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(product))
}
pub async fn update_product_handler(State(pool):State<PgPool>,Path(id):Path<i64>,Json(product):Json<UpdateProduct>)->Result<Json<Product>,StatusCode>{
    let product=update_product(&pool,id,product).await.map_err(|e| {match e{
        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(product))
}
pub async fn get_all_product_handler(State(pool):State<PgPool>,Query(params):Query<Pagination>)->Result<Json<Vec<Product>>,StatusCode>{
    let page=params.page.unwrap_or(1);
    let limit=params.limit.unwrap_or(20);
    let offset=(page-1)*limit;
    let product=get_all_product(&pool,limit as i64,offset as i64).await.map_err(|e|{
        println!("ERROR:{}",e);
        StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(product))
}