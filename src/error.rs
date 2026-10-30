//! Error types for the TradingView Streamer crate.

use thiserror::Error;

/// Core error enumeration for TradingView streamer operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Low-level WebSocket connection or transmission error.
    #[error("WebSocket transport error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// HTTP request error.
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// Protocol framing or encoding error.
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// JSON serialization or deserialization failure.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Symbol format is invalid (e.g. missing exchange prefix).
    #[error("Invalid symbol format '{0}'. Expected format: 'EXCHANGE:TICKER' (e.g. 'NASDAQ:AAPL')")]
    InvalidSymbol(String),

    /// Session establishment or negotiation failure.
    #[error("Session negotiation error: {0}")]
    Session(String),

    /// Timed out waiting for data from TradingView server.
    #[error("Operation timed out waiting for server response")]
    Timeout,

    /// Connection was unexpectedly closed by remote peer.
    #[error("WebSocket connection closed unexpectedly")]
    ConnectionClosed,

    /// Requested data was not found or stream completed without data.
    #[error("Data not found: {0}")]
    DataNotFound(String),
}

/// Convenience type alias for operations producing [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
