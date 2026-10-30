//! Financial Scanner & Multi-Market Screener Engine (Phase 009).
//!
//! Provides type-safe queries against TradingView's Scanner REST API across 19 global markets.

pub mod builder;
pub mod client;
pub mod filter;
pub mod market;
pub mod sort;
pub mod types;

pub use builder::ScannerBuilder;
pub use client::ScannerClient;
pub use filter::{Filter, FilterCondition, FilterOperation};
pub use market::ScannerMarket;
pub use sort::{SortConfig, SortOrder};
pub use types::{ScannerOptions, ScannerResponse, ScreenedItem};

/// Creates a new [`ScannerBuilder`] for the given market.
#[inline]
pub fn scanner(market: ScannerMarket) -> ScannerBuilder {
    ScannerBuilder::new(market)
}
