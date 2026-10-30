//! Real-time quote models and utilities for TradingView Quote Sessions (`qs_...`).

use serde::{Deserialize, Serialize};

/// Real-time Level 1 quote update received from TradingView.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct QuoteUpdate {
    /// Full symbol name (e.g. "NASDAQ:AAPL", "BINANCE:BTCUSDT").
    pub symbol: Option<String>,
    /// Last traded price (`lp`).
    pub last_price: Option<f64>,
    /// Price change compared to previous close (`ch`).
    pub change: Option<f64>,
    /// Percentage price change compared to previous close (`chp`).
    pub change_percentage: Option<f64>,
    /// Real-time price change including extended hours (`rtc`).
    pub realtime_change: Option<f64>,
    /// Real-time percentage price change including extended hours (`rchp`).
    pub realtime_change_percentage: Option<f64>,
    /// Best bid price.
    pub bid: Option<f64>,
    /// Best ask price.
    pub ask: Option<f64>,
    /// Bid/ask spread.
    pub spread: Option<f64>,
    /// Cumulative trading volume.
    pub volume: Option<f64>,
    /// Highest price of current session.
    pub high_price: Option<f64>,
    /// Lowest price of current session.
    pub low_price: Option<f64>,
    /// Opening price of current session.
    pub open_price: Option<f64>,
    /// Previous session closing price.
    pub prev_close_price: Option<f64>,
    /// Currency code (e.g. "USD", "USDT", "EUR").
    pub currency: Option<String>,
    /// Exchange name (e.g. "NASDAQ", "Binance").
    pub exchange: Option<String>,
    /// Last trade timestamp in seconds.
    pub last_price_time: Option<i64>,
    /// Raw unparsed properties from quote update payload.
    pub raw_fields: serde_json::Value,
}

impl QuoteUpdate {
    /// Returns the last traded price, or 0.0 if unavailable.
    pub fn price(&self) -> f64 {
        self.last_price.unwrap_or(0.0)
    }

    /// Returns true if the price change is non-negative.
    pub fn is_up(&self) -> bool {
        self.change.map(|c| c >= 0.0).unwrap_or(true)
    }

    /// Calculates or returns the bid-ask spread.
    pub fn bid_ask_spread(&self) -> Option<f64> {
        if let Some(s) = self.spread {
            Some(s)
        } else if let (Some(a), Some(b)) = (self.ask, self.bid) {
            Some(a - b)
        } else {
            None
        }
    }

    /// Merges an incremental quote delta update into this snapshot.
    pub fn merge(&mut self, other: QuoteUpdate) {
        if other.symbol.is_some() { self.symbol = other.symbol; }
        if other.last_price.is_some() { self.last_price = other.last_price; }
        if other.change.is_some() { self.change = other.change; }
        if other.change_percentage.is_some() { self.change_percentage = other.change_percentage; }
        if other.realtime_change.is_some() { self.realtime_change = other.realtime_change; }
        if other.realtime_change_percentage.is_some() { self.realtime_change_percentage = other.realtime_change_percentage; }
        if other.bid.is_some() { self.bid = other.bid; }
        if other.ask.is_some() { self.ask = other.ask; }
        if other.spread.is_some() { self.spread = other.spread; }
        if other.volume.is_some() { self.volume = other.volume; }
        if other.high_price.is_some() { self.high_price = other.high_price; }
        if other.low_price.is_some() { self.low_price = other.low_price; }
        if other.open_price.is_some() { self.open_price = other.open_price; }
        if other.prev_close_price.is_some() { self.prev_close_price = other.prev_close_price; }
        if other.currency.is_some() { self.currency = other.currency; }
        if other.exchange.is_some() { self.exchange = other.exchange; }
        if other.last_price_time.is_some() { self.last_price_time = other.last_price_time; }

        if let (Some(dest), Some(src)) = (self.raw_fields.as_object_mut(), other.raw_fields.as_object()) {
            for (k, v) in src {
                dest.insert(k.clone(), v.clone());
            }
        } else if other.raw_fields.is_object() {
            self.raw_fields = other.raw_fields;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_quote_update_defaults_and_helpers() {
        let mut q = QuoteUpdate::default();
        assert_eq!(q.price(), 0.0);
        assert!(q.is_up());
        assert_eq!(q.bid_ask_spread(), None);

        q.last_price = Some(150.25);
        q.change = Some(-1.50);
        q.bid = Some(150.20);
        q.ask = Some(150.30);
        q.raw_fields = json!({"test": 123});

        assert_eq!(q.price(), 150.25);
        assert!(!q.is_up());
        let spread = q.bid_ask_spread().unwrap();
        assert!((spread - 0.10).abs() < 1e-6);
    }
}
