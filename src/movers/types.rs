//! Types and record models for Market Movers rankings.

use serde::{Deserialize, Serialize};

/// Categories of market movers supported by TradingView.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MoverCategory {
    /// Top percentage gainers during regular session.
    Gainers,
    /// Top percentage losers during regular session.
    Losers,
    /// Most active instruments by trading volume.
    MostActive,
    /// Low-priced equities trading under $5 with high volume.
    PennyStocks,
    /// Top percentage gainers in Pre-Market Extended Trading Hours.
    PreMarketGainers,
    /// Top percentage losers in Pre-Market Extended Trading Hours.
    PreMarketLosers,
    /// Top percentage gainers in After-Hours Extended Trading Hours.
    AfterHoursGainers,
    /// Top percentage losers in After-Hours Extended Trading Hours.
    AfterHoursLosers,
}

/// Normalized market mover instrument record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoverRecord {
    /// Full TradingView symbol string (e.g. `"NASDAQ:NVDA"`).
    pub symbol: String,
    /// Ticker name (e.g. `"NVDA"`).
    pub name: String,
    /// Latest closing or current traded price.
    pub price: f64,
    /// Percentage change.
    pub change_percent: f64,
    /// Absolute price change.
    pub change_abs: Option<f64>,
    /// Traded volume.
    pub volume: f64,
    /// Market capitalization if available.
    pub market_cap: Option<f64>,
    /// Pre-market percentage change if in pre-market session.
    pub premarket_change: Option<f64>,
    /// Pre-market traded volume if available.
    pub premarket_volume: Option<f64>,
    /// After-hours percentage change if in post-market session.
    pub postmarket_change: Option<f64>,
    /// After-hours traded volume if available.
    pub postmarket_volume: Option<f64>,
}
