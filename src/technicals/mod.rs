//! Technical Summary Ratings & Indicator Matrix (Phase 011).
//!
//! Provides instant server-evaluated technical ratings (`Recommend.All`, `Recommend.MA`, `Recommend.Other`)
//! and multi-timeframe indicator snapshots without local algorithmic calculations.

pub mod client;
pub mod rating;
pub mod summary;
pub mod timeframe;

pub use client::TechnicalsClient;
pub use rating::TechnicalRating;
pub use summary::TechnicalSummary;
pub use timeframe::format_indicator_timeframe;

use crate::error::Result;

/// Fetches the daily technical summary rating for a symbol (1-liner).
pub async fn rating(symbol: impl AsRef<str>) -> Result<TechnicalSummary> {
    TechnicalsClient::new().summary(symbol, "1d").await
}

/// Fetches the technical summary rating for a symbol on a specific timeframe (e.g. `"1h"`, `"15m"`).
pub async fn rating_tf(
    symbol: impl AsRef<str>,
    timeframe: impl AsRef<str>,
) -> Result<TechnicalSummary> {
    TechnicalsClient::new().summary(symbol, timeframe).await
}

/// Fetches custom indicator values for a symbol on a specific timeframe.
pub async fn indicators(
    symbol: impl AsRef<str>,
    timeframe: impl AsRef<str>,
    indicators: &[impl AsRef<str>],
) -> Result<std::collections::HashMap<String, f64>> {
    TechnicalsClient::new()
        .indicators(symbol, timeframe, indicators)
        .await
}
