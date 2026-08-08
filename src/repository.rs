use crate::products_models::{CreateProduct, Product, UpdateProduct};
use sqlx::PgPool;
use crate::category_models::{Category, CreateCategory, UpdateCategory};

pub async fn create_product(
    pool: &PgPool,
    product: CreateProduct,
) -> Result<Product, sqlx::Error> {
    sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (name, description, price, stock,category_id)
        VALUES ($1, $2, $3, $4,$5)
        RETURNING id, name, description, price, stock,category_id
        "#,
        product.name,
        product.description,
        product.price,
        product.stock,
        product.category_id
    )
        .fetch_one(pool)
        .await
}
pub async fn get_product(
    pool:&PgPool,
    id:i64
)->Result<Product,sqlx::Error>{
    sqlx::query_as!(
        Product,
        r#"
        SELECT id,name,description,price,stock,category_id
        FROM products
        WHERE id=$1
        "#,
        id
    )
        .fetch_one(pool)
        .await
}
pub async fn delete_product(
    pool:&PgPool,
    id:i64
)->Result<Product,sqlx::Error>{
    sqlx::query_as!(
        Product,
        r#"
        DELETE FROM products
        WHERE id=$1
        RETURNING id,name,description,price,stock,category_id
        "#,
        id
    )
        .fetch_one(pool)
        .await
}
pub async fn update_product(
    pool: &PgPool,
    id: i64,
    product: UpdateProduct,
) -> Result<Product, sqlx::Error> {

    sqlx::query_as!(
        Product,
        r#"
        UPDATE products
        SET
            name = $1,
            description = $2,
            price = $3,
            stock = $4
        WHERE id = $5
        RETURNING id, name, description, price, stock,category_id
        "#,
        product.name,
        product.description,
        product.price,
        product.stock,
        id
    )
        .fetch_one(pool)
        .await
}
pub async fn get_all_product(
    pool:&PgPool,limit:i64,offset:i64
)->Result<Vec<Product>,sqlx::Error>{
    sqlx::query_as!(
        Product,
        r#"
        SELECT id,name,description,price,stock,category_id FROM products
        ORDER BY id
        LIMIT $1 OFFSET $2
            "#,
        limit,
        offset
    )
        .fetch_all(pool)
        .await
}
pub async fn create_category(pool:&PgPool,category:CreateCategory)->Result<Category,sqlx::Error>{
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