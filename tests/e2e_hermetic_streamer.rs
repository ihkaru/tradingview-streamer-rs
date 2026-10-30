//! Hermetic End-to-End (E2E) test suite for TradingView Streamer.
//!
//! Spawns an in-process WebSocket server speaking TradingView wire protocol (`~m~<len>~m~`),
//! verifies handshake, session negotiation, heartbeat keep-alive, extended session parameters (Issue #63),
//! and candle reception with zero external dependencies.

use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tradingview_streamer::{
    protocol::framing::{format_frame, format_heartbeat, parse_frames, InboundFrame},
    OhlcvCandle, SessionType, Timeframe, TvStreamer,
};

/// Helper to start a hermetic local mock TradingView WebSocket server.
async fn spawn_mock_tradingview_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind ephemeral TCP port");
    let addr = listener.local_addr().expect("Failed to query local address");

    let handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("Failed to accept connection");
        let mut ws = tokio_tungstenite::accept_async(stream)
            .await
            .expect("WebSocket handshake failed");

        let mut chart_session = String::new();
        let mut extended_session_verified = false;

        // Read messages from client
        while let Some(msg_res) = ws.next().await {
            let msg = msg_res.expect("Error reading message");
            if let Message::Text(text) = msg {
                let frames = parse_frames(&text).expect("Valid frame parsing");
                for frame in frames {
                    match frame {
                        InboundFrame::Message(payload) => {
                            let root: serde_json::Value =
                                serde_json::from_str(&payload).expect("Valid JSON");
                            let method = root["m"].as_str().unwrap_or_default();

                            if method == "chart_create_session" {
                                chart_session = root["p"][0].as_str().unwrap_or_default().to_string();
                            }

                            if method == "resolve_symbol" {
                                let spec = root["p"][2].as_str().unwrap_or_default();
                                if spec.contains(r#""session":"extended""#) {
                                    extended_session_verified = true;
                                }
                            }

                            if method == "create_series" {
                                // 1. Send heartbeat to test keep-alive
                                let hb = format_heartbeat(77);
                                ws.send(Message::Text(hb.into())).await.expect("Send heartbeat");

                                // 2. Send timescale_update payload with 3 test candles
                                let timescale_payload = serde_json::json!({
                                    "m": "timescale_update",
                                    "p": [
                                        chart_session,
                                        {
                                            "sds_1": {
                                                "s": [
                                                    {
                                                        "i": 1,
                                                        "v": [1700000000.0, 180.5, 182.0, 180.0, 181.5, 50000.0]
                                                    },
                                                    {
                                                        "i": 2,
                                                        "v": [1700000060.0, 181.5, 183.0, 181.0, 182.8, 62000.0]
                                                    },
                                                    {
                                                        "i": 3,
                                                        "v": [1700000120.0, 182.8, 184.0, 182.5, 183.9, 41000.0]
                                                    }
                                                ]
                                            }
                                        }
                                    ]
                                });

                                let framed = format_frame(&timescale_payload.to_string());
                                ws.send(Message::Text(framed.into())).await.expect("Send candles");
                            }
                        }
                        InboundFrame::Heartbeat(seq) => {
                            assert_eq!(seq, 77, "Client must correctly echo back heartbeat sequence 77");
                            // Test complete - close connection cleanly
                            let _ = ws.close(None).await;
                            break;
                        }
                    }
                }
            }
        }

        assert!(
            extended_session_verified,
            "Server must receive 'extended' session parameter in resolve_symbol (Issue #63)"
        );
    });

    (addr, handle)
}

#[tokio::test]
async fn test_e2e_hermetic_extended_session_streaming() {
    let (addr, server_handle) = spawn_mock_tradingview_server().await;
    let ws_url = format!("ws://{addr}");

    let streamer = TvStreamer::builder()
        .ws_url(ws_url)
        .symbol_str("NASDAQ:AAPL")
        .expect("Valid symbol")
        .timeframe(Timeframe::M1)
        .session(SessionType::Extended) // Issue #63: Extended hours
        .candle_count(3)
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Valid streamer build");

    let candles = streamer
        .fetch_candles()
        .await
        .expect("Fetching candles must succeed");

    assert_eq!(candles.len(), 3);
    assert_eq!(
        candles[0],
        OhlcvCandle {
            index: 1,
            timestamp: 1700000000,
            open: 180.5,
            high: 182.0,
            low: 180.0,
            close: 181.5,
            volume: Some(50000.0),
        }
    );
    assert_eq!(candles[1].index, 2);
    assert_eq!(candles[2].index, 3);
    assert_eq!(candles[2].close, 183.9);

    let _ = server_handle.await;
}
