//! Discovery and cross-exchange price arbitrage client.

use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde_json::json;

use crate::error::{Error, Result};
use crate::types::Symbol;
use super::types::{ArbitrageSpread, MarketVenue};

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

const DISCOVERY_COLUMNS: &[&str] = &[
    "name",
    "close",
    "change",
    "volume",
    "exchange",
    "currency",
    "type",
];

/// HTTP client for scanning multi-exchange venues and cross-market price discrepancies.
#[derive(Debug, Clone)]
pub struct DiscoveryClient {
    client: reqwest::Client,
}

impl Default for DiscoveryClient {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryClient {
    /// Creates a new venue discovery client.
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

    /// Dynamically resolves the primary high-liquidity venue for a symbol.
    ///
    /// Unlike hardcoded exchange heuristics, this queries TradingView's Global Scanner
    /// and selects the authentic venue strictly ranked by highest 24h trading volume.
    pub async fn resolve_primary_venue(&self, symbol: &str) -> Result<Symbol> {
        let trimmed = symbol.trim();
        if let Some((exch, sym)) = trimmed.split_once(':') {
            if !exch.trim().is_empty() && !sym.trim().is_empty() {
                return Ok(Symbol::new(exch.trim(), sym.trim()));
            }
        }

        let venues = self.discover_venues_exact(trimmed).await?;
        if venues.is_empty() {
            return Err(Error::DataNotFound(format!(
                "No trading venues discovered for ticker '{trimmed}'"
            )));
        }

        // Top venue by 24h volume is the primary listing
        let primary = &venues[0];
        let (exch, ticker) = if let Some((e, t)) = primary.symbol.split_once(':') {
            (e, t)
        } else {
            (primary.exchange.as_str(), primary.name.as_str())
        };

        Ok(Symbol::new(exch, ticker))
    }

    /// Discovers all trading venues with exact ticker matching.
    pub async fn discover_venues_exact(&self, ticker: &str) -> Result<Vec<MarketVenue>> {
        self.scan_venues("equal", ticker).await
    }

    /// Discovers all exchanges and markets where the given asset is traded.
    pub async fn discover_venues(&self, asset_name: &str) -> Result<Vec<MarketVenue>> {
        self.scan_venues("match", asset_name).await
    }

    async fn scan_venues(&self, filter_op: &str, query: &str) -> Result<Vec<MarketVenue>> {
        let url = "https://scanner.tradingview.com/global/scan";
        let payload = json!({
            "filter": [
                {
                    "left": "name",
                    "operation": filter_op,
                    "right": query
                }
            ],
            "columns": DISCOVERY_COLUMNS,
            "sort": {
                "sortBy": "volume",
                "sortOrder": "desc"
            },
            "range": [0, 50],
            "options": { "lang": "en" }
        });

        let response = self.client.post(url).json(&payload).send().await?;
        if !response.status().is_success() {
            return Err(Error::Protocol(format!(
                "Discovery endpoint returned HTTP status {}",
                response.status()
            )));
        }

        let body: serde_json::Value = response.json().await?;
        let data = body
            .get("data")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut venues = Vec::with_capacity(data.len());
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

            let price = d.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
            if price <= 0.0 {
                continue;
            }

            venues.push(MarketVenue {
                symbol,
                name: d.get(0).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                exchange: d.get(4).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                price,
                change_percent: d.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0),
                volume: d.get(3).and_then(|v| v.as_f64()).unwrap_or(0.0),
                currency: d.get(5).and_then(|v| v.as_str()).map(str::to_string),
                security_type: d.get(6).and_then(|v| v.as_str()).map(str::to_string),
            });
        }

        Ok(venues)
    }

    /// Computes cross-venue price spread and detects arbitrage opportunities.
    pub async fn arbitrage_spread(&self, asset_name: &str) -> Result<Option<ArbitrageSpread>> {
        let mut venues = self.discover_venues(asset_name).await?;
        if venues.len() < 2 {
            return Ok(None);
        }

        venues.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(std::cmp::Ordering::Equal));

        let highest = venues.first().unwrap();
        let lowest = venues.last().unwrap();

        let spread_abs = highest.price - lowest.price;
        let spread_percent = if lowest.price > 0.0 {
            (spread_abs / lowest.price) * 100.0
        } else {
            0.0
        };

        Ok(Some(ArbitrageSpread {
            asset: asset_name.to_string(),
            highest_venue: highest.exchange.clone(),
            highest_price: highest.price,
            lowest_venue: lowest.exchange.clone(),
            lowest_price: lowest.price,
            spread_abs,
            spread_percent,
            surveyed_venues: venues,
        }))
    }
}
