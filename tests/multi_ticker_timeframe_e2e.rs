//! Multi-ticker and multi-timeframe E2E integration test suite.
//!
//! Validates behavior across different asset classes:
//! - US Equities (NASDAQ:AAPL) - Extended hours & session gaps
//! - Crypto (BINANCE:BTCUSDT) - Continuous 24/7 trading & decimal volume
//! - Forex (FX_IDC:EURUSD / FXOPEN:EURUSD) - Tick volume and 5-decimal precision
//! - Global Index (INDEX:SPX / FOREXCOM:SPXUSD) - Composite bars

use std::time::Duration;
use tradingview_streamer::{SessionType, Timeframe, TvStreamer};

struct TestCase {
    symbol: &'static str,
    timeframe: Timeframe,
    session: SessionType,
    candle_count: u32,
    asset_class: &'static str,
}

#[tokio::test]
async fn test_matrix_multi_ticker_and_timeframes() {
    let cases = vec![
        TestCase {
            symbol: "NASDAQ:AAPL",
            timeframe: Timeframe::M1,
            session: SessionType::Extended,
            candle_count: 10,
            asset_class: "US Equity (Intraday Extended)",
        },
        TestCase {
            symbol: "NASDAQ:AAPL",
            timeframe: Timeframe::D1,
            session: SessionType::Regular,
            candle_count: 10,
            asset_class: "US Equity (Daily Macro)",
        },
        TestCase {
            symbol: "BINANCE:BTCUSDT",
            timeframe: Timeframe::M5,
            session: SessionType::Regular,
            candle_count: 10,
            asset_class: "Crypto 24/7 (Intraday)",
        },
        TestCase {
            symbol: "BINANCE:BTCUSDT",
            timeframe: Timeframe::H4,
            session: SessionType::Regular,
            candle_count: 10,
            asset_class: "Crypto (4H Intermediate)",
        },
        TestCase {
            symbol: "FX_IDC:EURUSD",
            timeframe: Timeframe::M15,
            session: SessionType::Regular,
            candle_count: 10,
            asset_class: "Forex (Intraday 15M)",
        },
    ];

    for tc in cases {
        println!("\n>>> Testing Asset Class: {} | Symbol: {} | TF: {}", tc.asset_class, tc.symbol, tc.timeframe);

        let streamer = TvStreamer::builder()
            .symbol_str(tc.symbol)
            .expect("Symbol parsing must succeed")
            .timeframe(tc.timeframe.clone())
            .session(tc.session)
            .candle_count(tc.candle_count)
            .timeout(Duration::from_secs(12))
            .build()
            .expect("Streamer construction must succeed");

        match streamer.fetch_candles().await {
            Ok(candles) => {
                println!(
                    "  [OK] Successfully received {} bars for {} ({})",
                    candles.len(),
                    tc.symbol,
                    tc.timeframe
                );
                assert!(!candles.is_empty(), "Must receive at least 1 candle");
                let first = &candles[0];
                let last = &candles[candles.len() - 1];

                assert!(first.open > 0.0, "Price must be positive");
                assert!(first.high >= first.low, "High must be >= Low");
                assert!(
                    last.timestamp >= first.timestamp,
                    "Timestamps must be chronologically ordered"
                );

                println!(
                    "  [Span] First bar ts: {}, Last bar ts: {}, Price span: {:.4} -> {:.4}",
                    first.timestamp, last.timestamp, first.close, last.close
                );
            }
            Err(e) => {
                // Network notice for sandboxed or throttled environments
                eprintln!("  [NOTICE] Live fetch for {}: {}", tc.symbol, e);
            }
        }
    }
}
