//! Domain models, session modes, timeframes, and OHLCV candle representations.

use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::error::{Error, Result};

/// Trading session type for TradingView market data.
///
/// Crucial for resolving Issue #63:
/// `Regular` sessions exclude pre-market and after-hours data.
/// `Extended` sessions explicitly instruct TradingView to stream Extended Trading Hours (ETH)
/// including pre-market (04:00 - 09:30 EST) and post-market (16:00 - 20:00 EST).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SessionType {
    /// Regular trading hours only (e.g. 09:30 - 16:00 EST for US equities).
    #[default]
    Regular,
    /// Extended trading hours including pre-market and after-hours trading sessions.
    Extended,
}

impl SessionType {
    /// Returns the exact wire string value expected by TradingView's WebSocket protocol.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Extended => "extended",
        }
    }

    /// Indicates whether extended market hours are enabled.
    #[inline]
    pub const fn is_extended(&self) -> bool {
        matches!(self, Self::Extended)
    }
}

impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SessionType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "regular" | "rth" => Ok(Self::Regular),
            "extended" | "eth" | "prepost" => Ok(Self::Extended),
            other => Err(Error::Protocol(format!(
                "Invalid session type '{other}'. Expected 'regular' or 'extended'"
            ))),
        }
    }
}

/// Chart timeframes supported by TradingView.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    /// 1 minute
    M1,
    /// 5 minutes
    M5,
    /// 15 minutes
    M15,
    /// 30 minutes
    M30,
    /// 1 hour (60 minutes)
    H1,
    /// 2 hours (120 minutes)
    H2,
    /// 4 hours (240 minutes)
    H4,
    /// 1 daily bar
    D1,
    /// 1 weekly bar
    W1,
    /// 1 monthly bar
    Month1,
    /// Custom interval representation (e.g. "D", "120", "W")
    Custom(String),
}

impl Timeframe {
    /// Returns the wire protocol representation used in TradingView's `create_series`.
    pub fn as_wire_str(&self) -> &str {
        match self {
            Self::M1 => "1",
            Self::M5 => "5",
            Self::M15 => "15",
            Self::M30 => "30",
            Self::H1 => "60",
            Self::H2 => "120",
            Self::H4 => "240",
            Self::D1 => "1D",
            Self::W1 => "1W",
            Self::Month1 => "1M",
            Self::Custom(val) => val.as_str(),
        }
    }
}

impl Default for Timeframe {
    fn default() -> Self {
        Self::M1
    }
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_wire_str())
    }
}

impl FromStr for Timeframe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tf = match s.trim() {
            "1" | "1m" | "1M_INTRA" => Self::M1,
            "5" | "5m" => Self::M5,
            "15" | "15m" => Self::M15,
            "30" | "30m" => Self::M30,
            "60" | "1h" | "1H" => Self::H1,
            "120" | "2h" | "2H" => Self::H2,
            "240" | "4h" | "4H" => Self::H4,
            "1D" | "D" | "1d" | "daily" => Self::D1,
            "1W" | "W" | "1w" | "weekly" => Self::W1,
            "1M" | "M" | "monthly" => Self::Month1,
            other => Self::Custom(other.to_string()),
        };
        Ok(tf)
    }
}

/// Trading symbol comprised of an exchange and ticker identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol {
    /// Exchange identifier (e.g. "NASDAQ", "BINANCE", "NYSE")
    pub exchange: String,
    /// Ticker symbol (e.g. "AAPL", "BTCUSDT", "SPY")
    pub ticker: String,
}

impl Symbol {
    /// Creates a new Symbol from exchange and ticker components.
    pub fn new(exchange: impl Into<String>, ticker: impl Into<String>) -> Self {
        Self {
            exchange: exchange.into().to_uppercase(),
            ticker: ticker.into().to_uppercase(),
        }
    }

    /// Formats as standard TradingView symbol representation (e.g. "NASDAQ:AAPL").
    pub fn to_exchange_symbol(&self) -> String {
        format!("{}:{}", self.exchange, self.ticker)
    }

    /// Parses a symbol string with smart exchange resolution.
    ///
    /// If an exchange prefix is provided (e.g. `"BINANCE:BTCUSDT"`, `"NYSE:IBM"`), it is respected.
    /// If only a ticker is passed (e.g. `"AAPL"`, `"BTCUSDT"`, `"EURUSD"`), the exchange is
    /// intelligently inferred:
    /// - Common crypto pairs ending in USDT/USDC/BTC -> `"BINANCE"`
    /// - 6-character forex currency pairs -> `"FX_IDC"`
    /// - General equity tickers -> `"NASDAQ"`
    pub fn smart_parse(s: &str) -> Result<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(Error::InvalidSymbol("Symbol cannot be empty".to_string()));
        }

        if let Some((exch, sym)) = trimmed.split_once(':') {
            if exch.trim().is_empty() || sym.trim().is_empty() {
                return Err(Error::InvalidSymbol(s.to_string()));
            }
            return Ok(Self::new(exch, sym));
        }

        let upper = trimmed.to_uppercase();
        let inferred_exchange = if upper.ends_with("USDT")
            || upper.ends_with("USDC")
            || upper.ends_with("BUSD")
            || upper == "BTC"
            || upper == "ETH"
            || upper == "SOL"
        {
            "BINANCE"
        } else if is_forex_pair(&upper) {
            "FX_IDC"
        } else {
            "NASDAQ"
        };

        Ok(Self::new(inferred_exchange, upper))
    }
}

/// Helper detecting standard 6-character currency pairs (e.g. EURUSD, GBPJPY).
fn is_forex_pair(s: &str) -> bool {
    if s.len() != 6 {
        return false;
    }
    const CURRENCIES: &[&str] = &[
        "USD", "EUR", "GBP", "JPY", "AUD", "CAD", "CHF", "NZD", "CNH", "SGD", "HKD",
    ];
    let base = &s[..3];
    let quote = &s[3..];
    CURRENCIES.contains(&base) && CURRENCIES.contains(&quote)
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.exchange, self.ticker)
    }
}

impl FromStr for Symbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::smart_parse(s)
    }
}

/// Represents a single OHLCV (Open-High-Low-Close-Volume) candle item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OhlcvCandle {
    /// Monotonically increasing bar index on the TradingView chart timescale.
    pub index: i64,
    /// Bar epoch timestamp in seconds.
    pub timestamp: i64,
    /// Opening price.
    pub open: f64,
    /// Highest price reached during bar interval.
    pub high: f64,
    /// Lowest price reached during bar interval.
    pub low: f64,
    /// Closing price.
    pub close: f64,
    /// Trading volume (optional, as some assets or extended frames lack volume).
    pub volume: Option<f64>,
}

pub use crate::quote::QuoteUpdate;

/// Inbound stream event yielded by the TradingView connection.
#[derive(Debug, Clone, PartialEq)]
pub enum StreamEvent {
    /// Batch of parsed OHLCV candles (from timescale_update or series completion).
    Candles(Vec<OhlcvCandle>),
    /// Incremental quote update for monitored symbol.
    Quote(QuoteUpdate),
    /// Indicator study data update.
    Study {
        /// Study identifier (e.g. "st1", "STD;RSI")
        study_id: String,
        /// Raw study data values
        data: serde_json::Value,
    },
    /// Heartbeat ping received from TradingView server.
    Heartbeat(i64),
    /// Symbol resolution completed successfully.
    SymbolResolved {
        /// Symbol string resolved
        symbol: String,
        /// Metadata payload
        metadata: serde_json::Value,
    },
    /// Series loading finished.
    SeriesCompleted,
}
