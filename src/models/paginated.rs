use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Clone, Copy)]
pub struct Paginated {
    pub page: i64,
    pub per_page: i64
}

impl Paginated {
    pub fn from_query(query: &PageQuery) -> Self {
        Self {
            page: query.page.unwrap_or(1).max(1),
            per_page: query.per_page.unwrap_or(20).min(100).max(1)
        }
    }

    #[inline]
    pub fn offset(self) -> i64 {
        (self.page - 1) * self.per_page
    }
}