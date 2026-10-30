//! Corporate & Economic Calendars (Phase 014).
//!
//! Provides automated retrieval of upcoming corporate dividend dates and earnings release surprises.

pub mod client;
pub mod types;

pub use client::CalendarClient;
pub use types::{DividendEvent, EarningsEvent};

use crate::error::Result;

/// Fetches upcoming dividend events across global markets.
pub async fn dividends(limit: usize) -> Result<Vec<DividendEvent>> {
    CalendarClient::new().dividends(None, limit).await
}

/// Fetches upcoming dividend events for US markets.
pub async fn dividends_us(limit: usize) -> Result<Vec<DividendEvent>> {
    CalendarClient::new().dividends(Some("america"), limit).await
}

/// Fetches scheduled quarterly corporate earnings announcements and surprises.
pub async fn earnings(limit: usize) -> Result<Vec<EarningsEvent>> {
    CalendarClient::new().earnings(None, limit).await
}

/// Fetches corporate earnings announcements for US equities.
pub async fn earnings_us(limit: usize) -> Result<Vec<EarningsEvent>> {
    CalendarClient::new().earnings(Some("america"), limit).await
}
