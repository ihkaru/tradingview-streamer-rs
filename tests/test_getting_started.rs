//! Tests validating the minimal-line Getting Started ergonomics (mirroring Python's yfinance).

use tradingview_streamer::prelude::*;

#[test]
fn test_smart_symbol_resolution() {
    // 1. Explicit symbols preserve exact exchange
    let explicit_crypto = Symbol::smart_parse("BINANCE:BTCUSDT").unwrap();
    assert_eq!(explicit_crypto.exchange, "BINANCE");
    assert_eq!(explicit_crypto.ticker, "BTCUSDT");

    let explicit_equity = Symbol::smart_parse("NYSE:IBM").unwrap();
    assert_eq!(explicit_equity.exchange, "NYSE");
    assert_eq!(explicit_equity.ticker, "IBM");

    // 2. Inferred Crypto without exchange prefix
    let auto_crypto = Symbol::smart_parse("BTCUSDT").unwrap();
    assert_eq!(auto_crypto.exchange, "BINANCE");
    assert_eq!(auto_crypto.ticker, "BTCUSDT");

    let auto_eth = Symbol::smart_parse("ETHUSDT").unwrap();
    assert_eq!(auto_eth.exchange, "BINANCE");

    // 3. Inferred Forex 6-character currency pair
    let auto_forex = Symbol::smart_parse("EURUSD").unwrap();
    assert_eq!(auto_forex.exchange, "FX_IDC");
    assert_eq!(auto_forex.ticker, "EURUSD");

    let auto_gbpjpy = Symbol::smart_parse("GBPJPY").unwrap();
    assert_eq!(auto_gbpjpy.exchange, "FX_IDC");

    // 4. Inferred US Equities (default to NASDAQ)
    let auto_aapl = Symbol::smart_parse("AAPL").unwrap();
    assert_eq!(auto_aapl.exchange, "NASDAQ");
    assert_eq!(auto_aapl.ticker, "AAPL");

    let auto_tsla = Symbol::smart_parse("TSLA").unwrap();
    assert_eq!(auto_tsla.exchange, "NASDAQ");
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
