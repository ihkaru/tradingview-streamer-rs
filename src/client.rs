//! Low-level WebSocket client managing TradingView connection lifecycle and frame dispatch.

use futures_util::{SinkExt, StreamExt};
use http::Request;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::handshake::client::generate_key;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::error::{Error, Result};
use crate::protocol::framing::{format_heartbeat, parse_frames, InboundFrame};
use crate::protocol::messages::{
    chart_create_session, create_series, quote_add_symbols, quote_create_session,
    quote_fast_symbols, quote_hibernate_all, quote_set_fields, resolve_symbol, set_auth_token,
    set_locale, switch_timezone, STANDARD_QUOTE_FIELDS,
};
use crate::protocol::parser::parse_message_payload;
use crate::session::SessionTokens;
use crate::types::{PriceAdjustment, SessionType, StreamEvent, Symbol, Timeframe};

/// Default TradingView WebSocket endpoint.
pub const DEFAULT_TRADINGVIEW_WS_URL: &str =
    "wss://data.tradingview.com/socket.io/websocket?from=chart%2FVEPYsueI%2F&type=chart";

/// Configuration parameters for establishing a WebSocket connection with TradingView.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// WebSocket server URL.
    pub ws_url: String,
    /// JWT authentication token.
    pub jwt_token: String,
    /// Language locale code.
    pub locale_lang: String,
    /// Country locale code.
    pub locale_country: String,
    /// Session tokens.
    pub tokens: SessionTokens,
    /// Connection timeout.
    pub connect_timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            ws_url: DEFAULT_TRADINGVIEW_WS_URL.to_string(),
            jwt_token: "unauthorized_user_token".to_string(),
            locale_lang: "en".to_string(),
            locale_country: "US".to_string(),
            tokens: SessionTokens::new(),
            connect_timeout: Duration::from_secs(15),
        }
    }
}

/// TradingView WebSocket client.
#[derive(Debug, Clone)]
pub struct TvWebSocketClient {
    config: ClientConfig,
}

impl TvWebSocketClient {
    /// Creates a new client with the given configuration.
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    /// Establishes the WebSocket connection with TradingView headers.
    pub async fn connect(
        &self,
    ) -> Result<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>> {
        let _ = rustls::crypto::ring::default_provider().install_default();

        let uri: http::Uri = self
            .config
            .ws_url
            .parse()
            .map_err(|e| Error::Protocol(format!("Invalid WebSocket URI: {e}")))?;

        let host = uri
            .host()
            .ok_or_else(|| Error::Protocol("Missing host in WebSocket URI".to_string()))?;

        let request = Request::builder()
            .uri(&self.config.ws_url)
            .header("Host", host)
            .header("Origin", "https://www.tradingview.com")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .header("Sec-WebSocket-Key", generate_key())
            .header("Sec-WebSocket-Version", "13")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .body(())
            .map_err(|e| Error::Protocol(format!("Failed to build HTTP request: {e}")))?;

        let connect_res = tokio::time::timeout(self.config.connect_timeout, connect_async(request)).await;
        let (ws_stream, _) = match connect_res {
            Ok(Ok(pair)) => pair,
            Ok(Err(ws_err)) => return Err(Error::WebSocket(ws_err)),
            Err(_) => return Err(Error::Timeout),
        };

        Ok(ws_stream)
    }

    /// Subscribes to symbol OHLCV data with configurable session type (Regular vs Extended).
    pub async fn start_streaming(
        &self,
        symbol: &Symbol,
        timeframe: &Timeframe,
        session_type: SessionType,
        candle_count: u32,
    ) -> Result<mpsc::Receiver<StreamEvent>> {
        self.start_streaming_adjusted(symbol, timeframe, session_type, PriceAdjustment::default(), candle_count).await
    }

    /// Subscribes to symbol OHLCV data with configurable session type and price adjustment (splits vs dividends).
    pub async fn start_streaming_adjusted(
        &self,
        symbol: &Symbol,
        timeframe: &Timeframe,
        session_type: SessionType,
        adjustment: PriceAdjustment,
        candle_count: u32,
    ) -> Result<mpsc::Receiver<StreamEvent>> {
        let (_cmd_tx, event_rx, _tokens) = self
            .start_interactive_streaming_adjusted(symbol, timeframe, session_type, adjustment, candle_count)
            .await?;
        Ok(event_rx)
    }

    /// Starts an interactive streaming session with outbound command channel and session tokens.
    pub async fn start_interactive_streaming(
        &self,
        symbol: &Symbol,
        timeframe: &Timeframe,
        session_type: SessionType,
        candle_count: u32,
    ) -> Result<(mpsc::Sender<String>, mpsc::Receiver<StreamEvent>, SessionTokens)> {
        self.start_interactive_streaming_adjusted(symbol, timeframe, session_type, PriceAdjustment::default(), candle_count).await
    }

    /// Starts an interactive streaming session with explicit price adjustment mode.
    pub async fn start_interactive_streaming_adjusted(
        &self,
        symbol: &Symbol,
        timeframe: &Timeframe,
        session_type: SessionType,
        adjustment: PriceAdjustment,
        candle_count: u32,
    ) -> Result<(mpsc::Sender<String>, mpsc::Receiver<StreamEvent>, SessionTokens)> {
        let ws_stream = self.connect().await?;
        let (event_tx, event_rx) = mpsc::channel(256);
        let (cmd_tx, cmd_rx) = mpsc::channel(64);
        let tokens = self.config.tokens.clone();
        let config = self.config.clone();
        let sym = symbol.clone();
        let tf = timeframe.clone();
        let session_tokens = tokens.clone();

        tokio::spawn(async move {
            if let Err(e) = run_client_loop(
                ws_stream,
                config,
                tokens,
                sym,
                tf,
                session_type,
                adjustment,
                candle_count,
                event_tx,
                cmd_rx,
            )
            .await
            {
                tracing::warn!("TradingView client loop exited: {e}");
            }
        });

        Ok((cmd_tx, event_rx, session_tokens))
    }
}

/// Core asynchronous message pump loop.
async fn run_client_loop(
    mut ws_stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    config: ClientConfig,
    tokens: SessionTokens,
    symbol: Symbol,
    timeframe: Timeframe,
    session_type: SessionType,
    adjustment: PriceAdjustment,
    candle_count: u32,
    event_tx: mpsc::Sender<StreamEvent>,
    mut cmd_rx: mpsc::Receiver<String>,
) -> Result<()> {
    send_initialization_messages(&mut ws_stream, &config, &tokens).await?;
    send_subscription_messages(
        &mut ws_stream,
        &tokens,
        &symbol,
        &timeframe,
        session_type,
        adjustment,
        candle_count,
    )
    .await?;

    loop {
        tokio::select! {
            Some(cmd) = cmd_rx.recv() => {
                ws_stream.send(Message::Text(cmd.into())).await.map_err(Error::WebSocket)?;
            }
            msg_res = ws_stream.next() => {
                let msg = match msg_res {
                    Some(Ok(m)) => m,
                    Some(Err(e)) => return Err(Error::WebSocket(e)),
                    None => break,
                };

                match msg {
                    Message::Text(text) => {
                        let frames = parse_frames(&text)?;
                        for frame in frames {
                            match frame {
                                InboundFrame::Heartbeat(seq) => {
                                    let pong = format_heartbeat(seq);
                                    ws_stream.send(Message::Text(pong.into())).await.map_err(Error::WebSocket)?;
                                    let _ = event_tx.send(StreamEvent::Heartbeat(seq)).await;
                                }
                                InboundFrame::Message(payload) => {
                                    let events = parse_message_payload(&payload, &tokens.series_id)?;
                                    for evt in events {
                                        if event_tx.send(evt).await.is_err() {
                                            return Ok(());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Message::Close(_) => return Err(Error::ConnectionClosed),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

/// Sends protocol initialization commands (auth, locale, create sessions, fields).
async fn send_initialization_messages<S>(
    stream: &mut S,
    config: &ClientConfig,
    tokens: &SessionTokens,
) -> Result<()>
where
    S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let msgs = [
        set_auth_token(&config.jwt_token),
        set_locale(&config.locale_lang, &config.locale_country),
        chart_create_session(&tokens.chart_session),
        quote_create_session(&tokens.quote_session),
        quote_set_fields(&tokens.quote_session, STANDARD_QUOTE_FIELDS),
        quote_hibernate_all(&tokens.quote_session),
    ];

    for msg in msgs {
        stream.send(Message::Text(msg.into())).await.map_err(Error::WebSocket)?;
    }

    Ok(())
}

/// Sends symbol resolution and chart series creation commands.
///
/// Implements the explicit `session_type` parameter in `resolve_symbol` solving Issue #63.
async fn send_subscription_messages<S>(
    stream: &mut S,
    tokens: &SessionTokens,
    symbol: &Symbol,
    timeframe: &Timeframe,
    session_type: SessionType,
    adjustment: PriceAdjustment,
    candle_count: u32,
) -> Result<()>
where
    S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let exchange_symbol = symbol.to_exchange_symbol();
    let spec = crate::protocol::messages::build_symbol_spec(&exchange_symbol, session_type, adjustment);

    let msgs = [
        quote_add_symbols(&tokens.quote_session, &spec),
        resolve_symbol(
            &tokens.chart_session,
            &tokens.symbol_alias,
            &exchange_symbol,
            session_type,
            adjustment,
        ),
        create_series(
            &tokens.chart_session,
            &tokens.series_id,
            "s1",
            &tokens.symbol_alias,
            timeframe,
            candle_count,
        ),
        quote_fast_symbols(&tokens.quote_session, &exchange_symbol),
        switch_timezone(&tokens.chart_session, "exchange"),
    ];

    for msg in msgs {
        stream.send(Message::Text(msg.into())).await.map_err(Error::WebSocket)?;
    }

    Ok(())
}
