use std::time::Duration;
use crate::client::DEFAULT_TRADINGVIEW_WS_URL;
use crate::types::{QuoteUpdate, SessionType};

/// Event delivered by the multi-symbol multiplexer or connection pool.
#[derive(Debug, Clone)]
pub struct MultiQuoteEvent {
    pub symbol: String,
    pub quote: QuoteUpdate,
}

/// Configuration for the multi-symbol connection pool.
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_symbols_per_socket: usize,
    pub session_type: SessionType,
    pub ws_url: String,
    pub connect_timeout: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_symbols_per_socket: 50,
            session_type: SessionType::Regular,
            ws_url: DEFAULT_TRADINGVIEW_WS_URL.to_string(),
            connect_timeout: Duration::from_secs(10),
        }
    }
}

impl PoolConfig {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn max_symbols_per_socket(mut self, limit: usize) -> Self {
        self.max_symbols_per_socket = limit.max(1);
        self
    }

    #[must_use]
    pub fn session_type(mut self, session: SessionType) -> Self {
        self.session_type = session;
        self
    }

    #[must_use]
    pub fn ws_url(mut self, url: impl Into<String>) -> Self {
        self.ws_url = url.into();
        self
    }
}
