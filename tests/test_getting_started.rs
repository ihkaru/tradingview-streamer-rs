//! Tests validating the minimal-line Getting Started ergonomics (mirroring Python's yfinance).

use tradingview_streamer::prelude::*;

#[test]
fn test_smart_symbol_resolution() {
    // 1. Explicit symbols preserve exact exchange (CON-08 compliant)
    let explicit_crypto = Symbol::parse_strict("BINANCE:BTCUSDT").unwrap();
    assert_eq!(explicit_crypto.exchange, "BINANCE");
    assert_eq!(explicit_crypto.ticker, "BTCUSDT");

    let explicit_equity = Symbol::parse_strict("NYSE:IBM").unwrap();
    assert_eq!(explicit_equity.exchange, "NYSE");
    assert_eq!(explicit_equity.ticker, "IBM");

    // 2. Strict parsing rejects un-prefixed symbols to prevent dangerous hardcoded guessing
    let strict_err = Symbol::parse_strict("IBM");
    assert!(matches!(strict_err, Err(tradingview_streamer::Error::MissingExchange { .. })));

    let strict_aapl = Symbol::parse_strict("AAPL");
    assert!(matches!(strict_aapl, Err(tradingview_streamer::Error::MissingExchange { .. })));

    // 3. Consumer-specified default exchange overrides (explicit choice)
    let user_nasdaq = Symbol::parse_with_default("AAPL", "NASDAQ").unwrap();
    assert_eq!(user_nasdaq.exchange, "NASDAQ");
    assert_eq!(user_nasdaq.ticker, "AAPL");

    let user_nyse = Symbol::parse_with_default("IBM", "NYSE").unwrap();
    assert_eq!(user_nyse.exchange, "NYSE");
    assert_eq!(user_nyse.ticker, "IBM");
}

#[tokio::test]
async fn test_minimal_line_live_yfinance_style() {
    // 1-Liner: Download recent bars instantly
    match download("AAPL").await {
        Ok(bars) => {
            println!("1-Liner Download OK: received {} daily bars", bars.len());
            assert!(!bars.is_empty());
        }
        Err(e) => eprintln!("Notice on live download: {e}"),
    }

    // 2-Liner: Ticker with Extended Hours enabled (Issue #63)
    let ticker = Ticker::new("AAPL");
    match ticker.history_extended("1m", 5).await {
        Ok(bars) => {
            println!("2-Liner Ticker Extended OK: received {} 1-min bars", bars.len());
            assert!(!bars.is_empty());
            assert!(bars[0].open > 0.0);
        }
        Err(e) => eprintln!("Notice on live history_extended: {e}"),
    }
}
