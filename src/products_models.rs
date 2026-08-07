use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Product{
   pub id:i64,
   pub name:String,
   pub description:String,
   pub price:i64,
   pub stock:i32,
}
#[derive(Serialize,Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: String,
    pub price: i64,
    pub stock: i32,
}
#[derive(Serialize,Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub description: String,
    pub price: i64,
    pub stock: i32,
}
#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}
#[derive(Deserialize)]
struct SearchParams {
    q: String,
    limit: Option<usize>,
}