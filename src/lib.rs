//! # TradingView Streamer (`tradingview-streamer`)
//!
//! High-performance, pure async Rust client and real-time streamer for TradingView WebSocket market data.
//! Designed for ultra-minimal line integration mirroring Python's `yfinance`.
//!
//! ## Highlights & Features
//! - **Minimal-Line Integration (`yfinance` style)**: Ingest candles in 1 line via [`download`] or [`Ticker`].
//! - **Smart Exchange Inference**: Pass `"AAPL"` directly without needing `"NASDAQ:AAPL"`.
//! - **Native Extended Trading Hours (ETH) Support**: Solves upstream Issue #63 from `mnwato/tradingview-scraper`
//!   by supporting `SessionType::Extended` to receive pre-market (04:00 - 09:30 EST) and
//!   after-hours (16:00 - 20:00 EST) OHLCV candles.
//! - **Pure Async & Low Memory**: Built with `tokio` and `tokio-tungstenite` with ~32KB crate footprint.
//! - **Zero Production Test Doubles**: 100% genuine protocol logic and verified with hermetic and live tests.
//!
//! ## Getting Started (Minimal Lines)
//!
//! ### 1. The 1-Liner: Download Bars Instantly
//! ```no_run
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let bars = tradingview_streamer::download("AAPL").await?;
//!     println!("Downloaded {} daily bars for AAPL", bars.len());
//!     Ok(())
//! }
//! ```
//!
//! ### 2. The 2-Liner: Ticker with Extended Hours (Issue #63)
//! ```no_run
//! use tradingview_streamer::Ticker;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let bars = Ticker::new("AAPL").history_extended("1m", 100).await?;
//!     println!("Received {} 1-min bars with pre/post market", bars.len());
//!     Ok(())
//! }
//! ```

pub mod backfill;
pub mod calendar;
pub mod client;
pub mod discovery;
pub mod error;
pub mod export;
pub mod fundamentals;
pub mod movers;
pub mod news;
pub mod pool;
pub mod protocol;
pub mod quote;
pub mod scanner;
pub mod session;
pub mod social;
pub mod streamer;
pub mod supervisor;
pub mod technicals;
pub mod ticker;
pub mod types;

// Re-exports of primary ergonomic interfaces
pub use backfill::BackfillPager;
pub use calendar::{CalendarClient, DividendEvent, EarningsEvent};
pub use client::{ClientConfig, TvWebSocketClient, DEFAULT_TRADINGVIEW_WS_URL};
pub use discovery::{ArbitrageSpread, DiscoveryClient, MarketVenue};
pub use error::{Error, Result};
pub use export::ColumnarCandles;
pub use fundamentals::{
    financials, overview, profile, valuation, CompanyProfile, DividendMetrics, FinancialMetrics,
    FundamentalOverview, FundamentalsClient, ValuationMetrics,
};
pub use movers::{MoversClient, MoverCategory, MoverRecord};
pub use news::{news, news_limit, NewsArticle, NewsClient};
pub use pool::{portfolio_quotes, ConnectionPool, MultiQuoteEvent, PoolConfig, SymbolRouter};
pub use scanner::{
    scanner, Filter, FilterCondition, FilterOperation, ScannerBuilder, ScannerClient,
    ScannerMarket, ScannerOptions, ScannerResponse, ScreenedItem, SortConfig, SortOrder,
};
pub use session::{generate_session_id, SessionTokens};
pub use social::{
    ideas, minds, sentiment, IdeaSort, MindAuthor, MindPost, MindsSort, SentimentBias,
    SentimentReport, SocialClient, TradeIdea,
};
pub use streamer::{TvStreamer, TvStreamerBuilder};
pub use supervisor::{
    stream_resilient, CircuitBreaker, CircuitState, ExponentialBackoff, ProxyPool,
    SupervisedStreamer, SupervisorConfig,
};
pub use technicals::{
    rating as technical_rating, rating_tf as technical_rating_tf, TechnicalRating,
    TechnicalSummary, TechnicalsClient,
};
pub use ticker::Ticker;
pub use types::{
    OhlcvCandle, QuoteUpdate, SessionType, StreamEvent, Symbol, Timeframe,
};

/// Instantiates an ergonomic [`Ticker`] for the given symbol with smart exchange resolution.
#[inline]
pub fn ticker(symbol: impl Into<String>) -> Ticker {
    Ticker::new(symbol)
}

/// Downloads recent daily OHLCV bars for a symbol with zero boilerplate.
///
/// # Example
/// ```no_run
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let bars = tradingview_streamer::download("AAPL").await?;
/// # Ok(())
/// # }
/// ```
pub async fn download(symbol: impl AsRef<str>) -> Result<Vec<OhlcvCandle>> {
    Ticker::new(symbol.as_ref()).history("1D", 100).await
}

/// Downloads OHLCV bars including pre-market and after-hours (Extended Trading Hours).
///
/// # Example
/// ```no_run
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let bars = tradingview_streamer::download_extended("AAPL", "1m", 100).await?;
/// # Ok(())
/// # }
/// ```
pub async fn download_extended(
    symbol: impl AsRef<str>,
    timeframe: impl AsRef<str>,
    count: u32,
) -> Result<Vec<OhlcvCandle>> {
    Ticker::new(symbol.as_ref())
        .history_extended(timeframe, count)
        .await
}

/// Fetches the latest real-time Level 1 quote snapshot for a symbol (1-liner).
///
/// # Example
/// ```no_run
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let q = tradingview_streamer::quote("AAPL").await?;
/// println!("AAPL Last: ${:.2}, Change: {:.2}%", q.price(), q.change_percentage.unwrap_or(0.0));
/// # Ok(())
/// # }
/// ```
pub async fn quote(symbol: impl AsRef<str>) -> Result<QuoteUpdate> {
    Ticker::new(symbol.as_ref()).quote().await
}

/// Fetches the latest real-time quote snapshot including pre-market and after-hours (Extended Trading Hours).
pub async fn quote_extended(symbol: impl AsRef<str>) -> Result<QuoteUpdate> {
    Ticker::new(symbol.as_ref()).quote_extended().await
}

/// Backfills deep historical OHLCV bars across multiple thousands of candles backwards.
pub async fn backfill(
    symbol: impl AsRef<str>,
    timeframe: impl AsRef<str>,
    count: usize,
) -> Result<Vec<OhlcvCandle>> {
    Ticker::new(symbol.as_ref()).backfill(timeframe, count).await
}

/// Backfills deep historical OHLCV bars with Extended Trading Hours (ETH).
pub async fn backfill_extended(
    symbol: impl AsRef<str>,
    timeframe: impl AsRef<str>,
    count: usize,
) -> Result<Vec<OhlcvCandle>> {
    Ticker::new(symbol.as_ref())
        .backfill_extended(timeframe, count)
        .await
}

/// Convenient prelude module re-exporting essential types and functions.
pub mod prelude {
    pub use crate::{
        backfill, backfill_extended, download, download_extended, financials, ideas, minds, news,
        overview, portfolio_quotes, profile, quote, quote_extended, scanner, sentiment,
        stream_resilient, technical_rating, technical_rating_tf, ticker, valuation,
        CircuitBreaker, CircuitState, ColumnarCandles, CompanyProfile, ConnectionPool, DividendEvent,
        DividendMetrics, EarningsEvent, ExponentialBackoff, Filter, FinancialMetrics,
        FundamentalOverview, IdeaSort, MarketVenue, MindAuthor, MindPost, MindsSort,
        MoverCategory, MoverRecord, MultiQuoteEvent, NewsArticle, OhlcvCandle, PoolConfig,
        ProxyPool, QuoteUpdate, ScannerMarket, SentimentBias, SentimentReport, SessionType,
        StreamEvent, SupervisedStreamer, SupervisorConfig, Symbol, SymbolRouter, TechnicalRating,
        TechnicalSummary, Ticker, Timeframe, TradeIdea, TvStreamer, ValuationMetrics,
    };
}
