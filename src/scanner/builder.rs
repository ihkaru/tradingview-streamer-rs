//! Fluent query builder for the TradingView Scanner API.

use crate::error::Result;
use super::client::ScannerClient;
use super::filter::FilterCondition;
use super::market::ScannerMarket;
use super::sort::{SortConfig, SortOrder};
use super::types::ScannerResponse;

/// Fluent builder for constructing and executing scanner queries.
#[derive(Debug, Clone)]
pub struct ScannerBuilder {
    client: ScannerClient,
    market: ScannerMarket,
    filters: Vec<FilterCondition>,
    columns: Vec<String>,
    sort: Option<SortConfig>,
    range_offset: usize,
    range_limit: usize,
}

impl ScannerBuilder {
    /// Creates a new scanner query builder for the target market.
    pub fn new(market: ScannerMarket) -> Self {
        let default_cols = market
            .default_columns()
            .into_iter()
            .map(str::to_string)
            .collect();

        Self {
            client: ScannerClient::new(),
            market,
            filters: Vec::new(),
            columns: default_cols,
            sort: None,
            range_offset: 0,
            range_limit: 50,
        }
    }

    /// Attaches an existing shared [`ScannerClient`].
    pub fn with_client(mut self, client: ScannerClient) -> Self {
        self.client = client;
        self
    }

    /// Adds a filter condition to the screener.
    pub fn filter(mut self, condition: FilterCondition) -> Self {
        self.filters.push(condition);
        self
    }

    /// Sets the list of columns to retrieve.
    pub fn columns<I, S>(mut self, cols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = cols.into_iter().map(Into::into).collect();
        self
    }

    /// Appends a single column to retrieve.
    pub fn add_column(mut self, col: impl Into<String>) -> Self {
        self.columns.push(col.into());
        self
    }

    /// Configures sorting for query results.
    pub fn sort_by(mut self, field: impl Into<String>, order: SortOrder) -> Self {
        self.sort = Some(SortConfig::new(field, order));
        self
    }

    /// Configures sort config directly.
    pub fn sort(mut self, config: SortConfig) -> Self {
        self.sort = Some(config);
        self
    }

    /// Sets the pagination range limit (maximum items returned).
    pub fn limit(mut self, limit: usize) -> Self {
        self.range_limit = limit;
        self
    }

    /// Sets the pagination start offset.
    pub fn offset(mut self, offset: usize) -> Self {
        self.range_offset = offset;
        self
    }

    /// Executes the scanner query and returns the matching instruments.
    pub async fn execute(self) -> Result<ScannerResponse> {
        let range = (self.range_offset, self.range_limit);
        self.client
            .scan(
                self.market,
                &self.filters,
                &self.columns,
                self.sort.as_ref(),
                range,
            )
            .await
    }
}
