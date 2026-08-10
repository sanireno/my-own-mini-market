
use sqlx::PgPool;
use crate::category_models::{Category, CreateCategory, UpdateCategory};
use crate::products_models::Product;

pub async fn create_category(pool:&PgPool, category:CreateCategory) ->Result<Category,sqlx::Error>{
    sqlx::query_as!(
        Category,
        r#"
        INSERT INTO categories (name)
        VALUES ($1)
        RETURNING id,name
        "#,
        category.name
    )
        .fetch_one(pool)
        .await
}
pub async fn get_category(pool:&PgPool,id:i64)->Result<Category,sqlx::Error>{
    sqlx::query_as!(
        Category,
        r#"
        SELECT id,name
        FROM categories
        WHERE id=$1
        "#,
        id
    )
        .fetch_one(pool)
        .await
}
pub async fn get_all_categories(pool:&PgPool)->Result<Vec<Category>,sqlx::Error>{
    sqlx::query_as!(
        Category,
        r#"
        SELECT id,name FROM categories
        ORDER BY id
        "#,
    )
        .fetch_all(pool)
        .await
}
pub async fn delete_category(
    pool: &PgPool,
    id: i64,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        r#"
        DELETE FROM categories
        WHERE id=$1
        RETURNING id,name
        "#,
        id
    )
        .fetch_one(pool)
        .await
}
pub async fn update_category(
    pool: &PgPool,
    id: i64,
    category: UpdateCategory,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        r#"
        UPDATE categories
        SET
            name = COALESCE($1,name)
        WHERE id = $2
        RETURNING id,name
        "#,
        category.name,
        id
    )
        .fetch_one(pool)
        .await
}
pub async fn get_products_from_category(pool:&PgPool,id:i64)->Result<Vec<Product>,sqlx::Error>{
    sqlx::query_as!(
        Product,
        r#"
        SELECT id, name, description, price, stock, category_id
        FROM products
        WHERE category_id = $1
        ORDER BY id
        "#,
        id
    )
        .fetch_all(pool)
        .await
}