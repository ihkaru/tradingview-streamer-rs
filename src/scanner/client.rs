//! HTTP client implementation for TradingView Scanner REST API.

use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};

use crate::error::{Error, Result};
use super::filter::FilterCondition;
use super::market::ScannerMarket;
use super::sort::SortConfig;
use super::types::{RawScannerResponse, ScannerOptions, ScannerPayload, ScannerResponse, ScreenedItem};

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

/// Asynchronous REST client querying TradingView's high-speed scanner endpoints.
#[derive(Debug, Clone)]
pub struct ScannerClient {
    client: reqwest::Client,
}

impl Default for ScannerClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ScannerClient {
    /// Constructs a new scanner client with standard connection pooling and timeouts.
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(15))
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(90))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { client }
    }

    /// Executes a scanner query against the specified market endpoint.
    pub async fn scan(
        &self,
        market: ScannerMarket,
        filters: &[FilterCondition],
        columns: &[String],
        sort: Option<&SortConfig>,
        range: (usize, usize),
    ) -> Result<ScannerResponse> {
        let url = market.scan_url();
        let payload = ScannerPayload {
            columns,
            options: ScannerOptions::default(),
            range: [range.0, range.1],
            filter: filters,
            sort,
        };

        let response = self.client.post(url).json(&payload).send().await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::Protocol(format!(
                "Scanner API returned error status {status}: {text}"
            )));
        }

        let raw: RawScannerResponse = response.json().await?;
        let items = raw
            .data
            .into_iter()
            .map(|item| ScreenedItem::new(item.s, columns, item.d))
            .collect();

        Ok(ScannerResponse {
            total_count: raw.total_count,
            items,
        })
    }
}
