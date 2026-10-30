//! High-level ergonomic streamer interface and builder.

use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::client::{ClientConfig, TvWebSocketClient, DEFAULT_TRADINGVIEW_WS_URL};
use crate::error::{Error, Result};
use crate::session::SessionTokens;
use crate::types::{OhlcvCandle, SessionType, StreamEvent, Symbol, Timeframe};

/// Ergonomic builder for constructing [`TvStreamer`].
#[derive(Debug, Clone)]
pub struct TvStreamerBuilder {
    symbol: Option<Symbol>,
    timeframe: Timeframe,
    session_type: SessionType,
    candle_count: u32,
    jwt_token: String,
    ws_url: String,
    timeout: Duration,
}

impl Default for TvStreamerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TvStreamerBuilder {
    /// Creates a new builder instance with default settings.
    pub fn new() -> Self {
        Self {
            symbol: None,
            timeframe: Timeframe::M1,
            session_type: SessionType::Regular,
            candle_count: 100,
            jwt_token: "unauthorized_user_token".to_string(),
            ws_url: DEFAULT_TRADINGVIEW_WS_URL.to_string(),
            timeout: Duration::from_secs(15),
        }
    }

    /// Sets the target symbol using a string format (`"EXCHANGE:TICKER"`, e.g. `"NASDAQ:AAPL"`).
    pub fn symbol_str(mut self, s: &str) -> Result<Self> {
        self.symbol = Some(Symbol::from_str(s)?);
        Ok(self)
    }

    /// Sets the target symbol using a [`Symbol`].
    pub fn symbol(mut self, sym: Symbol) -> Self {
        self.symbol = Some(sym);
        self
    }

    /// Sets the chart timeframe (resolution).
    pub fn timeframe(mut self, tf: Timeframe) -> Self {
        self.timeframe = tf;
        self
    }

    /// Sets the session type (`SessionType::Regular` or `SessionType::Extended`).
    ///
    /// ## Issue #63 Resolution
    /// Enabling `SessionType::Extended` ensures TradingView includes Pre-Market (04:00 - 09:30 EST)
    /// and Post-Market (16:00 - 20:00 EST) OHLCV candles.
    pub fn session(mut self, st: SessionType) -> Self {
        self.session_type = st;
        self
    }

    /// Convenience flag for toggling extended hours data on or off.
    pub fn extended_hours(mut self, enabled: bool) -> Self {
        self.session_type = if enabled {
            SessionType::Extended
        } else {
            SessionType::Regular
        };
        self
    }

    /// Sets the requested number of price candles (bars).
    pub fn candle_count(mut self, count: u32) -> Self {
        self.candle_count = count;
        self
    }

    /// Sets the JWT authentication token.
    pub fn jwt_token(mut self, token: impl Into<String>) -> Self {
        self.jwt_token = token.into();
        self
    }

    /// Sets a custom WebSocket URL (useful for self-hosted proxies or hermetic tests).
    pub fn ws_url(mut self, url: impl Into<String>) -> Self {
        self.ws_url = url.into();
        self
    }

    /// Sets the timeout for batch fetch operations.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    /// Builds the configured [`TvStreamer`].
    pub fn build(self) -> Result<TvStreamer> {
        let symbol = self
            .symbol
            .ok_or_else(|| Error::Protocol("Symbol must be specified before building TvStreamer".to_string()))?;

        let config = ClientConfig {
            ws_url: self.ws_url,
            jwt_token: self.jwt_token,
            locale_lang: "en".to_string(),
            locale_country: "US".to_string(),
            tokens: SessionTokens::new(),
            connect_timeout: self.timeout,
        };

        Ok(TvStreamer {
            symbol,
            timeframe: self.timeframe,
            session_type: self.session_type,
            candle_count: self.candle_count,
            timeout: self.timeout,
            client: TvWebSocketClient::new(config),
        })
    }
}

/// High-level client for fetching and streaming real-time TradingView market data.
pub struct TvStreamer {
    symbol: Symbol,
    timeframe: Timeframe,
    session_type: SessionType,
    candle_count: u32,
    timeout: Duration,
    client: TvWebSocketClient,
}

impl TvStreamer {
    /// Returns a new builder to configure a [`TvStreamer`].
    pub fn builder() -> TvStreamerBuilder {
        TvStreamerBuilder::new()
    }

    /// Subscribes to the live event stream for the configured symbol.
    pub async fn stream_events(&self) -> Result<mpsc::Receiver<StreamEvent>> {
        self.client
            .start_streaming(
                &self.symbol,
                &self.timeframe,
                self.session_type,
                self.candle_count,
            )
            .await
    }

    /// Fetches a batch of OHLCV candles, waiting until the requested count is met or timeout expires.
    pub async fn fetch_candles(&self) -> Result<Vec<OhlcvCandle>> {
        let mut rx = self.stream_events().await?;
        let mut collected: Vec<OhlcvCandle> = Vec::new();

        let fetch_fut = async {
            while let Some(evt) = rx.recv().await {
                if let StreamEvent::Candles(candles) = evt {
                    collected.extend(candles);
                    if collected.len() >= self.candle_count as usize {
                        break;
                    }
                }
            }
        };

        match tokio::time::timeout(self.timeout, fetch_fut).await {
            Ok(_) => {
                if collected.is_empty() {
                    Err(Error::DataNotFound(format!(
                        "No candle data received for {} (session: {})",
                        self.symbol, self.session_type
                    )))
                } else {
                    collected.sort_by_key(|c| c.index);
                    collected.dedup_by_key(|c| c.index);
                    Ok(collected)
                }
            }
            Err(_) => {
                if !collected.is_empty() {
                    collected.sort_by_key(|c| c.index);
                    collected.dedup_by_key(|c| c.index);
                    Ok(collected)
                } else {
                    Err(Error::Timeout)
                }
            }
        }
    }
}
