//! Event models for corporate earnings and dividend distribution calendars.

use serde::{Deserialize, Serialize};

/// Scheduled or historical corporate dividend distribution event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DividendEvent {
    /// Full TradingView symbol string (e.g. `"NASDAQ:AAPL"`).
    pub symbol: String,
    /// Ticker identifier (e.g. `"AAPL"`).
    pub name: String,
    /// Detailed instrument or company description.
    pub description: Option<String>,
    /// Dividend payment amount per share.
    pub amount: Option<f64>,
    /// Dividend yield as a percentage.
    pub yield_percent: Option<f64>,
    /// UNIX timestamp of ex-dividend date.
    pub ex_date: Option<i64>,
    /// UNIX timestamp of scheduled or completed payment date.
    pub payment_date: Option<i64>,
    /// Geographical or asset market classification (e.g. `"america"`).
    pub market: Option<String>,
}

/// Corporate quarterly earnings release event with analyst estimates and surprise metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarningsEvent {
    /// Full TradingView symbol string (e.g. `"NASDAQ:AAPL"`).
    pub symbol: String,
    /// Ticker identifier (e.g. `"AAPL"`).
    pub name: String,
    /// Detailed company description.
    pub description: Option<String>,
    /// Reported actual Earnings Per Share for the fiscal quarter.
    pub eps_actual: Option<f64>,
    /// Consensus forecasted Earnings Per Share.
    pub eps_forecast: Option<f64>,
    /// Earnings surprise percentage `((actual - forecast) / forecast) * 100`.
    pub eps_surprise_percent: Option<f64>,
    /// Actual reported revenue for the fiscal quarter.
    pub revenue_actual: Option<f64>,
    /// Consensus forecasted revenue.
    pub revenue_forecast: Option<f64>,
    /// UNIX timestamp of scheduled earnings announcement.
    pub release_date: Option<i64>,
    /// Geographical market classification (e.g. `"america"`).
    pub market: Option<String>,
}
