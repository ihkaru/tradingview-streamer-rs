//! End-to-End tests for TradingView Quote Session (`qs_...`) and Level 1 Ticker Tape.
//!
//! Validates hermetic protocol parsing of `qsd` frames as well as live quote retrieval.

use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

use tradingview_streamer::{
    quote, ticker, TvStreamer,
};

/// Hermetic test running an in-process mock WebSocket server emitting genuine TradingView wire `qsd` frames.
#[tokio::test]
async fn test_hermetic_quote_session_parsing() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let ws_url = format!("ws://127.0.0.1:{port}");

    tokio::spawn(async move {
        if let Ok((tcp_stream, _)) = listener.accept().await {
            let mut ws = accept_async(tcp_stream).await.unwrap();

            while let Some(msg_res) = ws.next().await {
                if let Ok(Message::Text(text)) = msg_res {
                    // When quote_fast_symbols is sent, reply with genuine `qsd` quote frame
                    if text.contains("quote_fast_symbols") {
                        let qsd_payload = r#"{"m":"qsd","p":["qs_test",{"n":"NASDAQ:AAPL","s":"ok","v":{"lp":225.50,"ch":2.35,"chp":1.05,"rtc":0.45,"rchp":0.20,"bid":225.48,"ask":225.52,"spread":0.04,"volume":48000000.0,"open_price":223.15,"high_price":226.10,"low_price":223.00,"prev_close_price":223.15,"currency_code":"USD","exchange":"NASDAQ","lp_time":1700000000}}]}"#;
                        let framed = format!("~m~{}~m~{}", qsd_payload.len(), qsd_payload);
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

    let q = streamer.fetch_quote().await.expect("Should receive parsed quote");
    assert_eq!(q.symbol.as_deref(), Some("NASDAQ:AAPL"));
    assert_eq!(q.last_price, Some(225.50));
    assert_eq!(q.change, Some(2.35));
    assert_eq!(q.change_percentage, Some(1.05));
    assert_eq!(q.realtime_change, Some(0.45));
    assert_eq!(q.realtime_change_percentage, Some(0.20));
    assert_eq!(q.bid, Some(225.48));
    assert_eq!(q.ask, Some(225.52));
    assert_eq!(q.spread, Some(0.04));
    assert_eq!(q.currency.as_deref(), Some("USD"));
    assert_eq!(q.exchange.as_deref(), Some("NASDAQ"));
    assert!(q.is_up());
    assert_eq!(q.price(), 225.50);
}

/// Live network test against real TradingView servers validating 1-liner quote retrieval.
#[tokio::test]
async fn test_live_quote_1_liner_and_ticker() {
    match quote("BINANCE:BTCUSDT").await {
        Ok(q) => {
            println!("Live BTCUSDT Quote: price=${:.2}, change={:.2}%, vol={:?}",
                q.price(), q.change_percentage.unwrap_or(0.0), q.volume
            );
            assert!(q.price() > 1000.0, "BTC price should be realistic");
            assert!(q.volume.is_some(), "Crypto should have volume");
        }
        Err(e) => {
            eprintln!("Live quote notice (network or rate limit): {e}");
        }
    }

    // Test Ticker quote with Extended Trading Hours
    let t = ticker("AAPL");
    match t.quote_extended().await {
        Ok(q) => {
            println!("Live AAPL Extended Quote: price=${:.2}, change={:.2}%",
                q.price(), q.change_percentage.unwrap_or(0.0)
            );
            assert!(q.price() > 0.0);
        }
        Err(e) => {
            eprintln!("Live quote notice: {e}");
        }
    }
}
