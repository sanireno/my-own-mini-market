use crate::products_models::{CreateProduct, Product, UpdateProduct};
use sqlx::PgPool;

pub async fn create_product(pool: &PgPool, product: CreateProduct) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (name, description, price, stock,category_id)
        VALUES ($1, $2, $3, $4,$5)
        RETURNING id, name, description, price, stock,
                  stock - reserved_stock AS available_stock, category_id
        "#,
    )
    .bind(product.name)
    .bind(product.description)
    .bind(product.price)
    .bind(product.stock)
    .bind(product.category_id)
    .fetch_one(pool)
    .await
}
pub async fn get_product(pool: &PgPool, id: i64) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        SELECT id, name, description, price, stock,
               stock - reserved_stock AS available_stock, category_id
        FROM products
        WHERE id=$1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
pub async fn delete_product(pool: &PgPool, id: i64) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        DELETE FROM products
        WHERE id=$1
        RETURNING id, name, description, price, stock,
                  stock - reserved_stock AS available_stock, category_id
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
pub async fn update_product(
    pool: &PgPool,
    id: i64,
    product: UpdateProduct,
) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        UPDATE products
        SET
            name = $1,
            description = $2,
            price = $3,
            stock = $4
        WHERE id = $5
        RETURNING id, name, description, price, stock,
                  stock - reserved_stock AS available_stock, category_id
        "#,
    )
    .bind(product.name)
    .bind(product.description)
    .bind(product.price)
    .bind(product.stock)
    .bind(id)
    .fetch_one(pool)
    .await
}
pub async fn get_all_product(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        SELECT id, name, description, price, stock,
               stock - reserved_stock AS available_stock, category_id
        FROM products
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
