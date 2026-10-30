//! HTTP client querying TradingView symbol technical analysis ratings.

use std::collections::HashMap;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

use crate::error::{Error, Result};
use crate::types::Symbol;
use super::rating::TechnicalRating;
use super::summary::TechnicalSummary;
use super::timeframe::format_indicator_timeframe;

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

/// Standard indicators requested by default for a technical summary.
const STANDARD_INDICATORS: &[&str] = &[
    "Recommend.All",
    "Recommend.MA",
    "Recommend.Other",
    "RSI",
    "MACD.macd",
    "MACD.signal",
    "close",
    "volume",
];

/// HTTP client for fetching server-evaluated technical ratings and indicator snapshots.
#[derive(Debug, Clone)]
pub struct TechnicalsClient {
    client: reqwest::Client,
}

impl Default for TechnicalsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TechnicalsClient {
    /// Constructs a new technicals client with connection pooling.
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { client }
    }

    /// Fetches the standard technical summary for a symbol on the given timeframe.
    pub async fn summary(
        &self,
        symbol: impl AsRef<str>,
        timeframe: impl AsRef<str>,
    ) -> Result<TechnicalSummary> {
        let resolved_sym = Symbol::resolve(symbol.as_ref());
        let tf_str = timeframe.as_ref();
        let tf_suffix = format_indicator_timeframe(tf_str);

        let query_fields: Vec<String> = STANDARD_INDICATORS
            .iter()
            .map(|&ind| format!("{ind}{tf_suffix}"))
            .collect();

        let raw_map = self.fetch_raw(&resolved_sym, &query_fields).await?;
        let normalized = self.normalize_fields(raw_map, tf_suffix);

        let all_score = normalized.get("Recommend.All").copied().unwrap_or(0.0);
        let ma_score = normalized.get("Recommend.MA").copied().unwrap_or(0.0);
        let osc_score = normalized.get("Recommend.Other").copied().unwrap_or(0.0);

        Ok(TechnicalSummary {
            symbol: resolved_sym.to_string(),
            timeframe: tf_str.to_string(),
            all: TechnicalRating::from_score(all_score),
            moving_averages: TechnicalRating::from_score(ma_score),
            oscillators: TechnicalRating::from_score(osc_score),
            all_score,
            ma_score,
            osc_score,
            indicators: normalized,
        })
    }

    /// Fetches custom indicator values for a symbol on the given timeframe.
    pub async fn indicators(
        &self,
        symbol: impl AsRef<str>,
        timeframe: impl AsRef<str>,
        indicators: &[impl AsRef<str>],
    ) -> Result<HashMap<String, f64>> {
        let resolved_sym = Symbol::resolve(symbol.as_ref());
        let tf_suffix = format_indicator_timeframe(timeframe.as_ref());

        let query_fields: Vec<String> = indicators
            .iter()
            .map(|ind| format!("{}{tf_suffix}", ind.as_ref()))
            .collect();

        let raw_map = self.fetch_raw(&resolved_sym, &query_fields).await?;
        Ok(self.normalize_fields(raw_map, tf_suffix))
    }

    async fn fetch_raw(
        &self,
        symbol: &Symbol,
        query_fields: &[String],
    ) -> Result<HashMap<String, serde_json::Value>> {
        let fields_param = query_fields.join(",").replace('+', "%2B");
        let url = format!(
            "https://scanner.tradingview.com/symbol?symbol={}&fields={fields_param}&no_404=true",
            symbol.to_exchange_symbol()
        );

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(Error::Protocol(format!(
                "Technicals endpoint returned HTTP status {}",
                response.status()
            )));
        }

        let map: HashMap<String, serde_json::Value> = response.json().await?;
        Ok(map)
    }

    fn normalize_fields(
        &self,
        raw_map: HashMap<String, serde_json::Value>,
        tf_suffix: &str,
    ) -> HashMap<String, f64> {
        let mut normalized = HashMap::with_capacity(raw_map.len());
        for (key, val) in raw_map {
            let clean_key = if !tf_suffix.is_empty() && key.ends_with(tf_suffix) {
                key[..key.len() - tf_suffix.len()].to_string()
            } else {
                key
            };

            if let Some(num) = val.as_f64() {
                normalized.insert(clean_key, num);
            }
        }
        normalized
    }
}
