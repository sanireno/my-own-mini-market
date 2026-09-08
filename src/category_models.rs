use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
}
