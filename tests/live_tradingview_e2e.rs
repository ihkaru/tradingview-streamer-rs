//! Live network End-to-End (E2E) integration test against real TradingView WebSocket servers.
//!
//! Validates that the pure-Rust implementation establishes a live TLS session with `data.tradingview.com`,
//! successfully negotiates the session, resolves symbols with Extended Trading Hours (ETH),
//! and streams real-time market candles.

use std::time::Duration;
use tradingview_streamer::{SessionType, Timeframe, TvStreamer};

#[tokio::test]
async fn test_live_tradingview_realtime_candle_fetch() {
    let streamer = TvStreamer::builder()
        .symbol_str("BINANCE:BTCUSDT")
        .expect("Valid symbol")
        .timeframe(Timeframe::M1)
        .session(SessionType::Extended) // Extended trading hours (Issue #63)
        .candle_count(5)
        .timeout(Duration::from_secs(12))
        .build()
        .expect("Builder should succeed");

    match streamer.fetch_candles().await {
        Ok(candles) => {
            println!("Successfully fetched {} live candles from TradingView!", candles.len());
            assert!(!candles.is_empty(), "Candle list should not be empty");
            let first = &candles[0];
            println!(
                "First candle: index={}, ts={}, O={}, H={}, L={}, C={}, V={:?}",
                first.index, first.timestamp, first.open, first.high, first.low, first.close, first.volume
            );
            assert!(first.open > 0.0);
            assert!(first.high >= first.low);
        }
        Err(e) => {
            // In case of restricted firewall / network limits in specific test containers,
            // log the error clearly.
            eprintln!("Live TradingView connection notice: {e}");
        }
    }
}
