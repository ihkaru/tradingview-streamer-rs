//! Tests for Phase 002: Technical Indicators & Built-in Studies Ingestion.

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use std::time::Duration;
use serde_json::json;

use tradingview_streamer::{
    protocol::messages::create_study,
    StreamEvent, TvStreamer,
};

#[tokio::test]
async fn test_hermetic_study_creation_and_du_parsing() {
    let study_msg = create_study(
        "cs_test",
        "st1",
        "st1",
        "sds_1",
        "Volume@tv-basicstudies-246",
        json!({"length": 20}),
    );
    assert!(study_msg.contains("create_study"));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let ws_url = format!("ws://127.0.0.1:{port}");

    tokio::spawn(async move {
        if let Ok((tcp_stream, _)) = listener.accept().await {
            let mut ws = accept_async(tcp_stream).await.unwrap();

            while let Some(msg_res) = ws.next().await {
                if let Ok(Message::Text(text)) = msg_res {
                    // When create_series is received, emit a `du` packet with study updates
                    if text.contains("create_series") {
                        let du_payload = json!({
                            "m": "du",
                            "p": [
                                "cs_test",
                                {
                                    "sds_1": {
                                        "s": [
                                            {"i": 1, "v": [1700000000.0, 100.0, 105.0, 99.0, 104.0, 5000.0]}
                                        ]
                                    },
                                    "st1": {
                                        "st": [
                                            {"i": 1, "v": [1700000000.0, 65.4]} // e.g. RSI value = 65.4
                                        ]
                                    }
                                }
                            ]
                        }).to_string();

                        let framed = format!("~m~{}~m~{}", du_payload.len(), du_payload);
                        let _ = ws.send(Message::Text(framed.into())).await;
                    }
                }
            }
        }
    });

    let streamer = TvStreamer::builder()
        .symbol_str("NASDAQ:AAPL")
        .unwrap()
        .ws_url(ws_url)
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    let mut rx = streamer.stream_events().await.unwrap();
    let mut got_candles = false;
    let mut got_study = false;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
    while tokio::time::Instant::now() < deadline {
        if let Ok(Some(evt)) = tokio::time::timeout(Duration::from_millis(500), rx.recv()).await {
            match evt {
                StreamEvent::Candles(c) => {
                    assert_eq!(c.len(), 1);
                    assert_eq!(c[0].close, 104.0);
                    got_candles = true;
                }
                StreamEvent::Study { study_id, data } => {
                    assert_eq!(study_id, "st1");
                    assert!(data.to_string().contains("65.4"));
                    got_study = true;
                }
                _ => {}
            }
        }
        if got_candles && got_study {
            break;
        }
    }

    assert!(got_candles, "Should receive parsed candles from du event");
    assert!(got_study, "Should receive parsed study data from du event");
}
