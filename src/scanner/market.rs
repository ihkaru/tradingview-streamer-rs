//! Market definitions for TradingView Scanner API.

use serde::{Deserialize, Serialize};

/// Supported geographic and asset markets for the TradingView scanner endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScannerMarket {
    /// United States equity markets (NYSE, NASDAQ, AMEX, OTC).
    America,
    /// Australian Securities Exchange (ASX).
    Australia,
    /// Canadian markets (TSX, TSXV, CSE).
    Canada,
    /// German markets (XETRA, Frankfurt, etc.).
    Germany,
    /// National Stock Exchange of India (NSE) & BSE.
    India,
    /// Tel Aviv Stock Exchange (TASE).
    Israel,
    /// Borsa Italiana (Milan).
    Italy,
    /// Luxembourg Stock Exchange.
    Luxembourg,
    /// Mexican Stock Exchange (BMV).
    Mexico,
    /// Bolsa de Madrid (BME).
    Spain,
    /// Borsa Istanbul (BIST).
    Turkey,
    /// London Stock Exchange (LSE).
    Uk,
    /// Pakistan Stock Exchange (PSX).
    Pakistan,
    /// Global Cryptocurrency pairs and spot/derivatives.
    Crypto,
    /// Foreign exchange currency pairs.
    Forex,
    /// Contract for Difference (CFD) markets.
    Cfd,
    /// Global commodity and financial futures contracts.
    Futures,
    /// Sovereign and corporate government bonds.
    Bonds,
    /// Global aggregated international screener.
    Global,
}

impl ScannerMarket {
    /// Returns the canonical API identifier slug for the market.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::America => "america",
            Self::Australia => "australia",
            Self::Canada => "canada",
            Self::Germany => "germany",
            Self::India => "india",
            Self::Israel => "israel",
            Self::Italy => "italy",
            Self::Luxembourg => "luxembourg",
            Self::Mexico => "mexico",
            Self::Spain => "spain",
            Self::Turkey => "turkey",
            Self::Uk => "uk",
            Self::Pakistan => "pakistan",
            Self::Crypto => "crypto",
            Self::Forex => "forex",
            Self::Cfd => "cfd",
            Self::Futures => "futures",
            Self::Bonds => "bonds",
            Self::Global => "global",
        }
    }

    /// Returns the full TradingView Scanner REST endpoint URL.
    #[must_use]
    pub const fn scan_url(&self) -> &'static str {
        match self {
            Self::America => "https://scanner.tradingview.com/america/scan",
            Self::Australia => "https://scanner.tradingview.com/australia/scan",
            Self::Canada => "https://scanner.tradingview.com/canada/scan",
            Self::Germany => "https://scanner.tradingview.com/germany/scan",
            Self::India => "https://scanner.tradingview.com/india/scan",
            Self::Israel => "https://scanner.tradingview.com/israel/scan",
            Self::Italy => "https://scanner.tradingview.com/italy/scan",
            Self::Luxembourg => "https://scanner.tradingview.com/luxembourg/scan",
            Self::Mexico => "https://scanner.tradingview.com/mexico/scan",
            Self::Spain => "https://scanner.tradingview.com/spain/scan",
            Self::Turkey => "https://scanner.tradingview.com/turkey/scan",
            Self::Uk => "https://scanner.tradingview.com/uk/scan",
            Self::Pakistan => "https://scanner.tradingview.com/pakistan/scan",
            Self::Crypto => "https://scanner.tradingview.com/crypto/scan",
            Self::Forex => "https://scanner.tradingview.com/forex/scan",
            Self::Cfd => "https://scanner.tradingview.com/cfd/scan",
            Self::Futures => "https://scanner.tradingview.com/futures/scan",
            Self::Bonds => "https://scanner.tradingview.com/bonds/scan",
            Self::Global => "https://scanner.tradingview.com/global/scan",
        }
    }

    /// Returns default columns fetched when none are explicitly provided.
    #[must_use]
    pub fn default_columns(&self) -> Vec<&'static str> {
        match self {
            Self::Crypto => vec![
                "name",
                "close",
                "change",
                "change_abs",
                "volume",
                "market_cap_calc",
                "Recommend.All",
            ],
            Self::Forex => vec![
                "name",
                "close",
                "change",
                "change_abs",
                "Recommend.All",
            ],
            _ => vec![
                "name",
                "close",
                "change",
                "change_abs",
                "volume",
                "Recommend.All",
                "market_cap_basic",
                "price_earnings_ttm",
                "earnings_per_share_basic_ttm",
            ],
        }
    }
}
