use crate::products_models::{CreateProduct, Pagination, Product, UpdateProduct};
use crate::repository;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use crate::category_models::{Category, CreateCategory, UpdateCategory};

pub async fn create_product_handler(State(pool):State<PgPool>, Json(products):Json<CreateProduct>) ->Result<Json<Product>,StatusCode>{
    let product = repository::create_product(&pool,products).await.map_err(|e| {
        println!("ERROR:{}",e);
        StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(product))
}
pub async fn get_product_handler(State(pool):State<PgPool>,Path(id):Path<i64>)->Result<Json<Product>,StatusCode>{
    let product=repository::get_product(&pool,id).await.map_err(|e| {match e{

        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(product))
}
pub async fn delete_product_handler(State(pool):State<PgPool>,Path(id):Path<i64>)->Result<Json<Product>,StatusCode>{
    let product=repository::delete_product(&pool,id).await.map_err(|e| {match e {
        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(product))
}
pub async fn update_product_handler(State(pool):State<PgPool>,Path(id):Path<i64>,Json(product):Json<UpdateProduct>)->Result<Json<Product>,StatusCode>{
    let product=repository::update_product(&pool,id,product).await.map_err(|e| {match e{
        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(product))
}
pub async fn get_all_product_handler(State(pool):State<PgPool>,Query(params):Query<Pagination>)->Result<Json<Vec<Product>>,StatusCode>{
    let page=params.page.unwrap_or(1);
    let limit=params.limit.unwrap_or(20);
    let offset=(page-1)*limit;
    let product=repository::get_all_product(&pool,limit as i64,offset as i64).await.map_err(|e|{
        println!("ERROR:{}",e);
        StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(product))
}
pub async fn get_category_handler(pool:State<PgPool>,Path(id):Path<i64>)->Result<Json<Category>,StatusCode>{
    let category=repository::get_category(&pool,id).await.map_err(|e| {match e {
        sqlx::Error::RowNotFound=>StatusCode::NOT_FOUND,
        _=>StatusCode::INTERNAL_SERVER_ERROR,
    }})?;
    Ok(Json(category))
}
pub async fn create_category_handler(pool:State<PgPool>,Json(category):Json<CreateCategory>)->Result<Json<Category>,StatusCode>{
    let category=repository::create_category(&pool,category).await.map_err(|e|
        {
            println!("ERROR:{}",e);
            StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(category))
}
pub async fn get_all_category_handler(pool:State<PgPool>)->Result<Json<Vec<Category>>,StatusCode>{
    println!("список всех категорий /get запрос");
    let category=repository::get_all_categories(&pool).await.map_err(|e| {
        println!("ERROR:{}",e);
        StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(category))
}
pub async fn delete_category_handler(pool:State<PgPool>,Path(id):Path<i64>)->Result<Json<Category>,StatusCode>{
    println!("DELETE запрос пришёл, id = {}", id);
    let category=repository::delete_category(&pool,id).await.map_err(|e| {match e {
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
    let category = repository::update_category(&pool, id, category)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(category))
}
pub async fn get_products_from_category_handler(State(pool):State<PgPool>,Path(id):Path<i64>)->Result<Json<Vec<Product>>,StatusCode>{
    let products=repository::get_products_from_category(&pool,id).await
        .map_err(|e| {println!("ERROR:{}",e);StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(Json(products))
}