//! REST client implementation for Corporate Calendars.

use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde_json::json;

use crate::error::{Error, Result};
use super::types::{DividendEvent, EarningsEvent};

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

/// Client for fetching dividend distributions and earnings release schedules.
#[derive(Debug, Clone)]
pub struct CalendarClient {
    client: reqwest::Client,
}

impl Default for CalendarClient {
    fn default() -> Self {
        Self::new()
    }
}

impl CalendarClient {
    /// Creates a new calendar client.
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(12))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { client }
    }

    /// Fetches upcoming and recent dividend events.
    pub async fn dividends(
        &self,
        market: Option<&str>,
        limit: usize,
    ) -> Result<Vec<DividendEvent>> {
        let url = "https://scanner.tradingview.com/global/scan?label-product=calendar-dividends";
        let columns = [
            "name",
            "description",
            "dividend_amount_recent",
            "dividends_yield",
            "dividend_ex_date_recent",
            "dividend_payment_date_recent",
            "market",
        ];

        let mut filters = vec![json!({
            "left": "dividend_amount_recent",
            "operation": "greater",
            "right": 0
        })];

        if let Some(m) = market {
            filters.push(json!({
                "left": "market",
                "operation": "equal",
                "right": m
            }));
        }

        let payload = json!({
            "columns": columns,
            "filter": filters,
            "range": [0, limit],
            "options": { "lang": "en" }
        });

        let raw = self.post_query(url, payload).await?;
        Ok(raw
            .into_iter()
            .map(|(symbol, d)| DividendEvent {
                symbol,
                name: get_str(&d, 0).unwrap_or_default(),
                description: get_str(&d, 1),
                amount: get_f64(&d, 2),
                yield_percent: get_f64(&d, 3),
                ex_date: get_i64(&d, 4),
                payment_date: get_i64(&d, 5),
                market: get_str(&d, 6),
            })
            .collect())
    }

    /// Fetches corporate earnings release events and surprises.
    pub async fn earnings(
        &self,
        market: Option<&str>,
        limit: usize,
    ) -> Result<Vec<EarningsEvent>> {
        let url = "https://scanner.tradingview.com/global/scan?label-product=calendar-earnings";
        let columns = [
            "name",
            "description",
            "earnings_per_share_fq",
            "earnings_per_share_forecast_next_fq",
            "eps_surprise_percent_fq",
            "revenue_fq",
            "revenue_forecast_next_fq",
            "earnings_release_next_date",
            "market",
        ];

        let mut filters = vec![json!({
            "left": "earnings_per_share_fq",
            "operation": "greater",
            "right": 0
        })];

        if let Some(m) = market {
            filters.push(json!({
                "left": "market",
                "operation": "equal",
                "right": m
            }));
        }

        let payload = json!({
            "columns": columns,
            "filter": filters,
            "range": [0, limit],
            "options": { "lang": "en" }
        });

        let raw = self.post_query(url, payload).await?;
        Ok(raw
            .into_iter()
            .map(|(symbol, d)| EarningsEvent {
                symbol,
                name: get_str(&d, 0).unwrap_or_default(),
                description: get_str(&d, 1),
                eps_actual: get_f64(&d, 2),
                eps_forecast: get_f64(&d, 3),
                eps_surprise_percent: get_f64(&d, 4),
                revenue_actual: get_f64(&d, 5),
                revenue_forecast: get_f64(&d, 6),
                release_date: get_i64(&d, 7),
                market: get_str(&d, 8),
            })
            .collect())
    }

    async fn post_query(
        &self,
        url: &str,
        payload: serde_json::Value,
    ) -> Result<Vec<(String, Vec<serde_json::Value>)>> {
        let response = self.client.post(url).json(&payload).send().await?;
        if !response.status().is_success() {
            return Err(Error::Protocol(format!(
                "Calendar API returned HTTP status {}",
                response.status()
            )));
        }

        let json_body: serde_json::Value = response.json().await?;
        let data = json_body
            .get("data")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut results = Vec::with_capacity(data.len());
        for item in data {
            let symbol = item
                .get("s")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let d = item
                .get("d")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            results.push((symbol, d));
        }

        Ok(results)
    }
}

fn get_str(d: &[serde_json::Value], idx: usize) -> Option<String> {
    d.get(idx).and_then(|v| v.as_str()).map(str::to_string)
}

fn get_f64(d: &[serde_json::Value], idx: usize) -> Option<f64> {
    d.get(idx).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_f64(),
        _ => None,
    })
}

fn get_i64(d: &[serde_json::Value], idx: usize) -> Option<i64> {
    d.get(idx).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_i64(),
        _ => None,
    })
}
