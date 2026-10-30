//! Sorting configuration for TradingView Scanner queries.

use serde::{Deserialize, Serialize};

/// Sorting order direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order (lowest to highest).
    Asc,
    /// Descending order (highest to lowest).
    Desc,
}

impl SortOrder {
    /// Returns the string representation expected by TradingView API.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

/// Sort parameter block for scanner queries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortConfig {
    /// Field name to sort by.
    #[serde(rename = "sortBy")]
    pub sort_by: String,
    /// Sort direction.
    #[serde(rename = "sortOrder")]
    pub sort_order: SortOrder,
}

impl SortConfig {
    /// Constructs a new sort configuration.
    pub fn new(sort_by: impl Into<String>, sort_order: SortOrder) -> Self {
        Self {
            sort_by: sort_by.into(),
            sort_order,
        }
    }

    /// Convenience helper for ascending sort.
    pub fn asc(field: impl Into<String>) -> Self {
        Self::new(field, SortOrder::Asc)
    }

    /// Convenience helper for descending sort.
    pub fn desc(field: impl Into<String>) -> Self {
        Self::new(field, SortOrder::Desc)
    }
}
