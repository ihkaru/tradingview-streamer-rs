//! # TradingView Streamer (`tradingview-streamer`)
//!
//! High-performance, pure async Rust client and real-time streamer for TradingView WebSocket market data.
//!
//! ## Highlights & Features
//! - **Native Extended Trading Hours (ETH) Support**: Solves upstream Issue #63 from `mnwato/tradingview-scraper`
//!   by explicitly supporting `SessionType::Extended` to receive pre-market (04:00 - 09:30 EST) and
//!   after-hours (16:00 - 20:00 EST) OHLCV candles.
//! - **Pure Async & Low Memory**: Built with `tokio` and `tokio-tungstenite` using zero-copy framing.
//! - **Strongly Typed**: Clean domain types for [`Symbol`], [`Timeframe`], [`SessionType`], and [`OhlcvCandle`].
//! - **Automatic Heartbeat Management**: Seamlessly echos TradingView keep-alive ping frames (`~h~<seq>`).
//! - **Zero Production Test Doubles**: 100% genuine protocol logic and hermetic end-to-end integration tests.
//!
//! ## Quickstart
//!
//! ```no_run
//! use tradingview_streamer::{TvStreamer, SessionType, Timeframe};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Build a streamer with Extended Hours enabled (Issue #63)
//!     let streamer = TvStreamer::builder()
//!         .symbol_str("NASDAQ:AAPL")?
//!         .timeframe(Timeframe::M1)
//!         .session(SessionType::Extended) // Pre & post market bars
//!         .candle_count(50)
//!         .build()?;
//!
//!     let candles = streamer.fetch_candles().await?;
//!     println!("Fetched {} candles for AAPL", candles.len());
//!     for candle in candles.iter().take(5) {
//!         println!("Bar [{}]: O={:.2} H={:.2} L={:.2} C={:.2}",
//!             candle.timestamp, candle.open, candle.high, candle.low, candle.close);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod protocol;
pub mod session;
pub mod streamer;
pub mod types;

// Re-exports of primary ergonomic interfaces
pub use client::{ClientConfig, TvWebSocketClient, DEFAULT_TRADINGVIEW_WS_URL};
pub use error::{Error, Result};
pub use session::{generate_session_id, SessionTokens};
pub use streamer::{TvStreamer, TvStreamerBuilder};
pub use types::{
    OhlcvCandle, QuoteUpdate, SessionType, StreamEvent, Symbol, Timeframe,
};
