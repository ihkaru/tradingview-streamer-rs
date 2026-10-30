//! Multi-Exchange Discovery & Global Arbitrage Engine (Phase 013).
//!
//! Provides global cross-exchange symbol mapping and real-time venue spread detection.

pub mod client;
pub mod types;

pub use client::DiscoveryClient;
pub use types::{ArbitrageSpread, MarketVenue};

use crate::error::Result;

/// Discovers all trading venues and exchange listings for an asset (1-liner).
pub async fn venues(asset_name: &str) -> Result<Vec<MarketVenue>> {
    DiscoveryClient::new().discover_venues(asset_name).await
}

/// Detects cross-venue price spread and potential arbitrage divergence (1-liner).
pub async fn arbitrage(asset_name: &str) -> Result<Option<ArbitrageSpread>> {
    DiscoveryClient::new().arbitrage_spread(asset_name).await
}
