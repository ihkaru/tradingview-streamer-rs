//! Market Movers & Extended Hours Leaders (Phase 010).
//!
//! Provides rankings for Pre-Market and After-Hours movers, top gainers, losers, and most active assets.

pub mod client;
pub mod types;

pub use client::MoversClient;
pub use types::{MoverCategory, MoverRecord};

use crate::error::Result;
use crate::scanner::ScannerMarket;

/// Fetches top percentage gainers in Pre-Market Extended Trading Hours for US equities.
pub async fn pre_market_gainers(limit: usize) -> Result<Vec<MoverRecord>> {
    MoversClient::new()
        .fetch(ScannerMarket::America, MoverCategory::PreMarketGainers, limit)
        .await
}

/// Fetches top percentage losers in Pre-Market Extended Trading Hours for US equities.
pub async fn pre_market_losers(limit: usize) -> Result<Vec<MoverRecord>> {
    MoversClient::new()
        .fetch(ScannerMarket::America, MoverCategory::PreMarketLosers, limit)
        .await
}

/// Fetches top percentage gainers in After-Hours Extended Trading Hours for US equities.
pub async fn after_hours_gainers(limit: usize) -> Result<Vec<MoverRecord>> {
    MoversClient::new()
        .fetch(ScannerMarket::America, MoverCategory::AfterHoursGainers, limit)
        .await
}

/// Fetches top percentage losers in After-Hours Extended Trading Hours for US equities.
pub async fn after_hours_losers(limit: usize) -> Result<Vec<MoverRecord>> {
    MoversClient::new()
        .fetch(ScannerMarket::America, MoverCategory::AfterHoursLosers, limit)
        .await
}

/// Fetches regular session top percentage gainers for US equities.
pub async fn gainers(limit: usize) -> Result<Vec<MoverRecord>> {
    MoversClient::new()
        .fetch(ScannerMarket::America, MoverCategory::Gainers, limit)
        .await
}

/// Fetches regular session top percentage losers for US equities.
pub async fn losers(limit: usize) -> Result<Vec<MoverRecord>> {
    MoversClient::new()
        .fetch(ScannerMarket::America, MoverCategory::Losers, limit)
        .await
}

/// Fetches most active equities by trading volume.
pub async fn most_active(limit: usize) -> Result<Vec<MoverRecord>> {
    MoversClient::new()
        .fetch(ScannerMarket::America, MoverCategory::MostActive, limit)
        .await
}

/// Fetches active penny stocks trading under $5.
pub async fn penny_stocks(limit: usize) -> Result<Vec<MoverRecord>> {
    MoversClient::new()
        .fetch(ScannerMarket::America, MoverCategory::PennyStocks, limit)
        .await
}
