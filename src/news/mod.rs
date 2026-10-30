//! Real-Time Market News & Editorial Intelligence Feed (Phase 015).
//!
//! Provides streaming headline news, wire stories, and publisher metadata for financial instruments.

pub mod client;
pub mod types;

pub use client::NewsClient;
pub use types::NewsArticle;

use crate::error::Result;

/// Fetches recent news headlines for a symbol (default limit: 10 stories).
pub async fn news(symbol: impl AsRef<str>) -> Result<Vec<NewsArticle>> {
    NewsClient::new().fetch_symbol_news(symbol, 10).await
}

/// Fetches a specified number of news headlines for a symbol.
pub async fn news_limit(symbol: impl AsRef<str>, limit: usize) -> Result<Vec<NewsArticle>> {
    NewsClient::new().fetch_symbol_news(symbol, limit).await
}
