//! Ergonomic Ticker interface modeling python's `yfinance.Ticker`.

use std::str::FromStr;
use tokio::sync::mpsc;
use crate::error::Result;
use crate::streamer::TvStreamer;
use crate::types::{OhlcvCandle, QuoteUpdate, SessionType, StreamEvent, Timeframe};

/// Ergonomic ticker client mirroring `yfinance.Ticker` for minimal-line integration.
#[derive(Debug, Clone)]
pub struct Ticker {
    symbol_str: String,
}

impl Ticker {
    /// Creates a new Ticker for the given symbol (e.g. `"AAPL"`, `"NASDAQ:AAPL"`, `"BTCUSDT"`).
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol_str: symbol.into(),
        }
    }

    /// Fetches historical OHLCV candles using regular market hours.
    ///
    /// # Example
    /// ```no_run
    /// use tradingview_streamer::Ticker;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bars = Ticker::new("AAPL").history("1m", 100).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn history(&self, timeframe: impl AsRef<str>, count: u32) -> Result<Vec<OhlcvCandle>> {
        self.fetch_internal(timeframe.as_ref(), count, SessionType::Regular).await
    }

    /// Fetches historical OHLCV candles including pre-market and after-hours (Extended Trading Hours).
    ///
    /// Resolves upstream limitation Issue #63.
    ///
    /// # Example
    /// ```no_run
    /// use tradingview_streamer::Ticker;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bars = Ticker::new("AAPL").history_extended("1m", 100).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn history_extended(&self, timeframe: impl AsRef<str>, count: u32) -> Result<Vec<OhlcvCandle>> {
        self.fetch_internal(timeframe.as_ref(), count, SessionType::Extended).await
    }

    /// Returns a live real-time event stream for this ticker.
    pub async fn stream(&self, timeframe: impl AsRef<str>) -> Result<mpsc::Receiver<StreamEvent>> {
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        TvStreamer::builder()
            .symbol_str(&self.symbol_str)?
            .timeframe(tf)
            .session(SessionType::Regular)
            .build()?
            .stream_events()
            .await
    }

    /// Returns a live real-time event stream with Extended Trading Hours.
    pub async fn stream_extended(&self, timeframe: impl AsRef<str>) -> Result<mpsc::Receiver<StreamEvent>> {
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        TvStreamer::builder()
            .symbol_str(&self.symbol_str)?
            .timeframe(tf)
            .session(SessionType::Extended)
            .build()?
            .stream_events()
            .await
    }

    /// Fetches the latest real-time Level 1 quote for this ticker during regular market hours.
    pub async fn quote(&self) -> Result<QuoteUpdate> {
        self.quote_internal(SessionType::Regular).await
    }

    /// Fetches the latest real-time Level 1 quote including pre-market and after-hours (Extended Trading Hours).
    pub async fn quote_extended(&self) -> Result<QuoteUpdate> {
        self.quote_internal(SessionType::Extended).await
    }

    /// Streams real-time Level 1 quote updates for this ticker.
    pub async fn stream_quotes(&self) -> Result<mpsc::Receiver<QuoteUpdate>> {
        self.stream_quotes_internal(SessionType::Regular).await
    }

    /// Streams real-time Level 1 quote updates with Extended Trading Hours.
    pub async fn stream_quotes_extended(&self) -> Result<mpsc::Receiver<QuoteUpdate>> {
        self.stream_quotes_internal(SessionType::Extended).await
    }

    async fn quote_internal(&self, session: SessionType) -> Result<QuoteUpdate> {
        let streamer = TvStreamer::builder()
            .symbol_str(&self.symbol_str)?
            .session(session)
            .build()?;
        streamer.fetch_quote().await
    }

    async fn stream_quotes_internal(&self, session: SessionType) -> Result<mpsc::Receiver<QuoteUpdate>> {
        let streamer = TvStreamer::builder()
            .symbol_str(&self.symbol_str)?
            .session(session)
            .build()?;
        let mut event_rx = streamer.stream_events().await?;
        let (quote_tx, quote_rx) = mpsc::channel(256);

        tokio::spawn(async move {
            while let Some(evt) = event_rx.recv().await {
                if let StreamEvent::Quote(q) = evt {
                    if quote_tx.send(q).await.is_err() {
                        break;
                    }
                }
            }
        });

        Ok(quote_rx)
    }

    async fn fetch_internal(&self, tf_str: &str, count: u32, session: SessionType) -> Result<Vec<OhlcvCandle>> {
        let tf = Timeframe::from_str(tf_str)?;
        let streamer = TvStreamer::builder()
            .symbol_str(&self.symbol_str)?
            .timeframe(tf)
            .session(session)
            .candle_count(count)
            .build()?;

        streamer.fetch_candles().await
    }
}
