use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const DEFAULT_PAGE_SIZE: u64 = 20;
pub const MAX_PAGE_SIZE: u64 = 100;

/// Pagination values resolved from a list query.
///
/// Query structs declare `page` and `page_size` directly and build this type
/// with [`Self::from_parts`]. `#[serde(flatten)]` is deliberately avoided:
/// `serde_urlencoded` hands flattened numeric fields to serde as strings, which
/// fails to deserialize into `u64`. Endpoints that keep a legacy bare-array
/// response use [`Self::pagination`]; endpoints that always return an envelope
/// use [`Self::pagination_or_default`].
#[derive(Debug, Clone, Default, Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

impl PaginationQuery {
    pub fn from_parts(page: Option<u64>, page_size: Option<u64>) -> Self {
        Self { page, page_size }
    }

    /// Returns `None` when neither parameter is present, so legacy callers keep
    /// receiving the full list instead of an envelope they cannot parse.
    pub fn pagination(&self) -> Option<(u64, u64)> {
        if self.page.is_none() && self.page_size.is_none() {
            return None;
        }

        Some(self.pagination_or_default())
    }

    /// Resolves the requested page, defaulting to the first page of twenty.
    pub fn pagination_or_default(&self) -> (u64, u64) {
        let page = self.page.unwrap_or(1).max(1);
        let page_size = self
            .page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(1, MAX_PAGE_SIZE);

        (page, page_size)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub total_pages: u64,
}

impl<T> Paginated<T> {
    pub fn new(items: Vec<T>, page: u64, page_size: u64, total: u64) -> Self {
        debug_assert!(page_size > 0, "page_size must be greater than zero");

        Self {
            items,
            page,
            page_size,
            total,
            total_pages: total.div_ceil(page_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pagination_from(json: &str) -> Option<(u64, u64)> {
        serde_json::from_str::<PaginationQuery>(json)
            .expect("valid pagination query")
            .pagination()
    }

    #[test]
    fn omitted_pagination_keeps_the_legacy_shape() {
        assert_eq!(pagination_from("{}"), None);
    }

    #[test]
    fn pagination_defaults_and_clamps() {
        assert_eq!(pagination_from(r#"{"page":3}"#), Some((3, 20)));
        assert_eq!(pagination_from(r#"{"page_size":500}"#), Some((1, 100)));
        assert_eq!(pagination_from(r#"{"page":0,"page_size":0}"#), Some((1, 1)));
    }

    #[test]
    fn default_pagination_is_the_first_page_of_twenty() {
        let query: PaginationQuery = serde_json::from_str("{}").unwrap();

        assert_eq!(query.pagination_or_default(), (1, DEFAULT_PAGE_SIZE));
    }

    #[test]
    fn pagination_rejects_camel_case_aliases() {
        let error = serde_json::from_str::<PaginationQuery>(r#"{"pageSize":10}"#)
            .expect_err("camelCase pagination is not accepted");

        assert!(error.to_string().contains("unknown field"));
    }
}
