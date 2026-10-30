//! Models for multi-exchange trading venue discovery and arbitrage spreads.

use serde::{Deserialize, Serialize};

/// Trading venue information where an asset is listed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketVenue {
    /// Full exchange symbol (e.g. `"BINANCE:BTCUSDT"` or `"COINBASE:BTCUSD"`).
    pub symbol: String,
    /// Instrument ticker name (e.g. `"BTCUSDT"`).
    pub name: String,
    /// Operating exchange or liquidity venue (e.g. `"BINANCE"`, `"COINBASE"`).
    pub exchange: String,
    /// Latest closing or traded price on venue.
    pub price: f64,
    /// Percentage price change.
    pub change_percent: f64,
    /// Traded volume.
    pub volume: f64,
    /// Base trading or settlement currency.
    pub currency: Option<String>,
    /// Instrument asset classification.
    pub security_type: Option<String>,
}

/// Cross-venue price divergence and arbitrage opportunity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArbitrageSpread {
    /// Base asset ticker.
    pub asset: String,
    /// Venue with highest traded price.
    pub highest_venue: String,
    /// Highest price.
    pub highest_price: f64,
    /// Venue with lowest traded price.
    pub lowest_venue: String,
    /// Lowest price.
    pub lowest_price: f64,
    /// Absolute price difference `highest - lowest`.
    pub spread_abs: f64,
    /// Percentage price difference `(spread_abs / lowest_price) * 100`.
    pub spread_percent: f64,
    /// All surveyed venues sorted by price descending.
    pub surveyed_venues: Vec<MarketVenue>,
}
