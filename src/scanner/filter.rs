//! Type-safe filter expressions for TradingView Scanner API.

use serde::{Deserialize, Serialize};

/// Supported comparison and logical operations in TradingView Scanner queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperation {
    /// Field value is strictly greater than right-hand value.
    Greater,
    /// Field value is strictly less than right-hand value.
    Less,
    /// Field value is equal to or greater than right-hand value.
    #[serde(rename = "egreater")]
    EqualOrGreater,
    /// Field value is equal to or less than right-hand value.
    #[serde(rename = "eless")]
    EqualOrLess,
    /// Field value is equal to right-hand value.
    Equal,
    /// Field value is not equal to right-hand value.
    #[serde(rename = "nequal")]
    NotEqual,
    /// Field value is within inclusive range `[min, max]`.
    #[serde(rename = "in_range")]
    InRange,
    /// Field value is outside range `[min, max]`.
    #[serde(rename = "not_in_range")]
    NotInRange,
    /// Technical indicator is above another indicator/threshold.
    Above,
    /// Technical indicator is below another indicator/threshold.
    Below,
    /// Technical indicator crosses another indicator/threshold.
    Crosses,
    /// Technical indicator crosses above another indicator/threshold.
    #[serde(rename = "crosses_above")]
    CrossesAbove,
    /// Technical indicator crosses below another indicator/threshold.
    #[serde(rename = "crosses_below")]
    CrossesBelow,
    /// Set contains value.
    Has,
    /// Set contains none of the values.
    #[serde(rename = "has_none_of")]
    HasNoneOf,
}

/// A single scanner filter condition AST node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterCondition {
    /// Column or field identifier (e.g. `"close"`, `"volume"`, `"RSI"`).
    pub left: String,
    /// Comparison operation.
    pub operation: FilterOperation,
    /// Comparison right-hand operand (scalar, string, or array).
    pub right: serde_json::Value,
}

/// Fluent builder for constructing [`FilterCondition`].
#[derive(Debug, Clone)]
pub struct Filter {
    field: String,
}

impl Filter {
    /// Creates a new filter expression builder for the given field.
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
        }
    }

    /// Evaluates if field is strictly greater than `val`.
    pub fn greater(self, val: impl Into<serde_json::Value>) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::Greater,
            right: val.into(),
        }
    }

    /// Evaluates if field is strictly less than `val`.
    pub fn less(self, val: impl Into<serde_json::Value>) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::Less,
            right: val.into(),
        }
    }

    /// Evaluates if field is equal to or greater than `val`.
    pub fn equal_or_greater(self, val: impl Into<serde_json::Value>) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::EqualOrGreater,
            right: val.into(),
        }
    }

    /// Evaluates if field is equal to or less than `val`.
    pub fn equal_or_less(self, val: impl Into<serde_json::Value>) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::EqualOrLess,
            right: val.into(),
        }
    }

    /// Evaluates if field equals `val`.
    pub fn equal(self, val: impl Into<serde_json::Value>) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::Equal,
            right: val.into(),
        }
    }

    /// Evaluates if field is not equal to `val`.
    pub fn not_equal(self, val: impl Into<serde_json::Value>) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::NotEqual,
            right: val.into(),
        }
    }

    /// Evaluates if field is within inclusive range `[min, max]`.
    pub fn in_range<T: Into<serde_json::Value>>(self, min: T, max: T) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::InRange,
            right: serde_json::Value::Array(vec![min.into(), max.into()]),
        }
    }

    /// Evaluates if field crosses above `threshold`.
    pub fn crosses_above(self, threshold: impl Into<serde_json::Value>) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::CrossesAbove,
            right: threshold.into(),
        }
    }

    /// Evaluates if field crosses below `threshold`.
    pub fn crosses_below(self, threshold: impl Into<serde_json::Value>) -> FilterCondition {
        FilterCondition {
            left: self.field,
            operation: FilterOperation::CrossesBelow,
            right: threshold.into(),
        }
    }
}
