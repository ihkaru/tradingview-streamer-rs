//! Data structures and response representations for the Scanner API.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::filter::FilterCondition;
use super::sort::SortConfig;

/// Payload submitted to the TradingView Scanner REST endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct ScannerPayload<'a> {
    pub columns: &'a [String],
    pub options: ScannerOptions,
    pub range: [usize; 2],
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub filter: &'a [FilterCondition],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<&'a SortConfig>,
}

/// Additional options for scanner query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerOptions {
    pub lang: String,
}

impl Default for ScannerOptions {
    fn default() -> Self {
        Self {
            lang: "en".to_string(),
        }
    }
}

/// Raw response returned by TradingView Scanner API.
#[derive(Debug, Clone, Deserialize)]
pub struct RawScannerResponse {
    #[serde(rename = "totalCount", default)]
    pub total_count: usize,
    #[serde(default)]
    pub data: Vec<RawScannerItem>,
}

/// Raw item in scanner API response.
#[derive(Debug, Clone, Deserialize)]
pub struct RawScannerItem {
    /// Full symbol string (e.g. `"NASDAQ:AAPL"`).
    pub s: String,
    /// Ordered column values corresponding to requested columns.
    pub d: Vec<serde_json::Value>,
}

/// Deserialized instrument match returned by scanner.
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenedItem {
    /// Full symbol identifier (e.g. `"NASDAQ:AAPL"`).
    pub symbol: String,
    /// Extracted fields keyed by column name.
    pub fields: HashMap<String, serde_json::Value>,
}

impl ScreenedItem {
    /// Creates a screened item from raw symbol and column values.
    pub fn new(symbol: String, columns: &[String], values: Vec<serde_json::Value>) -> Self {
        let mut fields = HashMap::with_capacity(columns.len());
        for (col, val) in columns.iter().zip(values.into_iter()) {
            fields.insert(col.clone(), val);
        }
        Self { symbol, fields }
    }

    /// Extracts a field as `f64`.
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.fields.get(key).and_then(|v| match v {
            serde_json::Value::Number(n) => n.as_f64(),
            _ => None,
        })
    }

    /// Extracts a field as a string slice.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.fields.get(key).and_then(|v| v.as_str())
    }

    /// Extracts a field as `i64`.
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.fields.get(key).and_then(|v| match v {
            serde_json::Value::Number(n) => n.as_i64(),
            _ => None,
        })
    }

    /// Convenience getter for instrument ticker name (e.g. `"AAPL"`).
    pub fn name(&self) -> Option<&str> {
        self.get_str("name")
    }

    /// Convenience getter for latest close/last price.
    pub fn close(&self) -> Option<f64> {
        self.get_f64("close")
    }

    /// Convenience getter for price percentage change.
    pub fn change(&self) -> Option<f64> {
        self.get_f64("change")
    }

    /// Convenience getter for absolute price change.
    pub fn change_abs(&self) -> Option<f64> {
        self.get_f64("change_abs")
    }

    /// Convenience getter for traded volume.
    pub fn volume(&self) -> Option<f64> {
        self.get_f64("volume")
    }

    /// Convenience getter for market capitalization.
    pub fn market_cap(&self) -> Option<f64> {
        self.get_f64("market_cap_basic")
            .or_else(|| self.get_f64("market_cap_calc"))
    }

    /// Convenience getter for technical recommendation score (-1.0 to 1.0).
    pub fn recommend_all(&self) -> Option<f64> {
        self.get_f64("Recommend.All")
    }
}

/// Fully formatted scanner response with matched items.
#[derive(Debug, Clone, PartialEq)]
pub struct ScannerResponse {
    /// Total instruments in the market matching filter conditions.
    pub total_count: usize,
    /// List of instruments returned in current range.
    pub items: Vec<ScreenedItem>,
}
