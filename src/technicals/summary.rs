//! Technical analysis summary snapshot.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::rating::TechnicalRating;

/// Consolidated technical rating and indicator snapshot for an instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TechnicalSummary {
    /// Symbol identifier (e.g. `"NASDAQ:AAPL"`).
    pub symbol: String,
    /// Evaluated timeframe (e.g. `"1d"`, `"1h"`).
    pub timeframe: String,
    /// Overall composite rating.
    pub all: TechnicalRating,
    /// Moving averages consensus rating.
    pub moving_averages: TechnicalRating,
    /// Oscillators consensus rating.
    pub oscillators: TechnicalRating,
    /// Overall score between -1.0 (Strong Sell) and +1.0 (Strong Buy).
    pub all_score: f64,
    /// Moving averages score (-1.0 to 1.0).
    pub ma_score: f64,
    /// Oscillators score (-1.0 to 1.0).
    pub osc_score: f64,
    /// Raw numerical indicator values keyed by indicator name (e.g. `"RSI"`, `"MACD.macd"`).
    pub indicators: HashMap<String, f64>,
}

impl TechnicalSummary {
    /// Returns the value of a specific indicator if present.
    pub fn get(&self, indicator: &str) -> Option<f64> {
        self.indicators.get(indicator).copied()
    }

    /// Convenience getter for 14-period Relative Strength Index (RSI).
    pub fn rsi(&self) -> Option<f64> {
        self.get("RSI")
    }

    /// Convenience getter for MACD line.
    pub fn macd(&self) -> Option<f64> {
        self.get("MACD.macd")
    }

    /// Convenience getter for MACD signal line.
    pub fn macd_signal(&self) -> Option<f64> {
        self.get("MACD.signal")
    }

    /// Convenience getter for latest close price from technical snapshot.
    pub fn close(&self) -> Option<f64> {
        self.get("close")
    }

    /// Convenience getter for traded volume.
    pub fn volume(&self) -> Option<f64> {
        self.get("volume")
    }
}
