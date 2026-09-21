use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: i64,
    pub stock: i32,
    pub available_stock: i32,
    pub category_id: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: String,
    pub price: i64,
    pub stock: i32,
    pub category_id: i64,
}
#[derive(Serialize, Deserialize)]
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

impl Pagination {
    pub fn limit_and_offset(&self) -> Result<(i64, i64), crate::app_error::AppError> {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(20);
        if page == 0 {
            return Err(crate::app_error::AppError::BadRequest(
                "page must be at least 1".to_string(),
            ));
        }
        if !(1..=100).contains(&limit) {
            return Err(crate::app_error::AppError::BadRequest(
                "limit must be between 1 and 100".to_string(),
            ));
        }
        // A u32 page multiplied by a limit <= 100 always fits in i64.
        let offset = (i64::from(page) - 1) * i64::from(limit);
        Ok((i64::from(limit), offset))
    }
}

#[cfg(test)]
mod pagination_tests {
    use super::Pagination;

    #[test]
    fn defaults_and_page_boundaries() {
        assert_eq!(
            Pagination {
                page: None,
                limit: None
            }
            .limit_and_offset()
            .unwrap(),
            (20, 0)
        );
        assert_eq!(
            Pagination {
                page: Some(2),
                limit: Some(100)
            }
            .limit_and_offset()
            .unwrap(),
            (100, 100)
        );
        assert_eq!(
            Pagination {
                page: Some(u32::MAX),
                limit: Some(100)
            }
            .limit_and_offset()
            .unwrap(),
            (100, 429496729400)
        );
    }

    #[test]
    fn rejects_invalid_bounds() {
        for (page, limit) in [(0, 20), (1, 0), (1, 101), (1, u32::MAX)] {
            assert!(
                Pagination {
                    page: Some(page),
                    limit: Some(limit)
                }
                .limit_and_offset()
                .is_err()
            );
        }
    }
}
