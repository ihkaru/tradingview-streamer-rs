//! HTTP client implementation for TradingView News Headlines feed.

use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::types::Symbol;
use super::types::NewsArticle;

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

#[derive(Debug, Deserialize)]
struct RawNewsResponse {
    #[serde(default)]
    items: Vec<RawNewsItem>,
}

#[derive(Debug, Deserialize)]
struct RawNewsItem {
    id: String,
    title: String,
    #[serde(default)]
    provider: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    published: i64,
    #[serde(default)]
    urgency: i32,
    #[serde(default)]
    link: String,
    #[serde(rename = "relatedSymbols", default)]
    related_symbols: Vec<RawRelatedSymbol>,
}

#[derive(Debug, Deserialize)]
struct RawRelatedSymbol {
    symbol: String,
}

/// HTTP client for fetching real-time financial market headlines.
#[derive(Debug, Clone)]
pub struct NewsClient {
    client: reqwest::Client,
}

impl Default for NewsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl NewsClient {
    /// Creates a new news client with keep-alive connection pooling.
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

    /// Fetches news headlines for a specific symbol.
    pub async fn fetch_symbol_news(
        &self,
        symbol: impl AsRef<str>,
        limit: usize,
    ) -> Result<Vec<NewsArticle>> {
        let sym_str = symbol.as_ref();
        let resolved = if sym_str.contains(':') {
            Symbol::parse_strict(sym_str)?
        } else {
            crate::discovery::DiscoveryClient::new()
                .resolve_primary_venue(sym_str)
                .await
                .unwrap_or_else(|_| Symbol::new("", sym_str))
        };
        let url = format!(
            "https://news-headlines.tradingview.com/v2/view/headlines/symbol?client=web&lang=en&symbol={}",
            resolved.to_exchange_symbol()
        );

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(Error::Protocol(format!(
                "News headlines API returned HTTP status {}",
                response.status()
            )));
        }

        let raw: RawNewsResponse = response.json().await?;
        let articles = raw
            .items
            .into_iter()
            .take(limit)
            .map(|item| NewsArticle {
                id: item.id,
                title: item.title,
                provider: item.provider,
                source: item.source,
                published: item.published,
                urgency: item.urgency,
                link: item.link,
                related_symbols: item
                    .related_symbols
                    .into_iter()
                    .map(|r| r.symbol)
                    .collect(),
            })
            .collect();

        Ok(articles)
    }
}
