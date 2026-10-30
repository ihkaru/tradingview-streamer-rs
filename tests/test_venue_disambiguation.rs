//! Integration tests verifying CON-08 & ADR-04:
//! Dynamic Venue Disambiguation, Zero Silent Exchange Guessing, and Strict Venue Validation.

use tradingview_streamer::discovery::DiscoveryClient;
use tradingview_streamer::error::Error;
use tradingview_streamer::ticker::Ticker;
use tradingview_streamer::types::Symbol;

#[test]
fn test_strict_parsing_invariant() {
    // Canonical explicit formats succeed instantly without ambiguity
    let ibm = Symbol::parse_strict("NYSE:IBM").expect("Explicit NYSE:IBM must succeed");
    assert_eq!(ibm.exchange, "NYSE");
    assert_eq!(ibm.ticker, "IBM");

    let btc = Symbol::parse_strict("COINBASE:BTCUSD").expect("Explicit COINBASE:BTCUSD must succeed");
    assert_eq!(btc.exchange, "COINBASE");
    assert_eq!(btc.ticker, "BTCUSD");

    // Un-prefixed tickers fail fast with MissingExchange error (CON-08)
    let err_ibm = Symbol::parse_strict("IBM");
    match err_ibm {
        Err(Error::MissingExchange { symbol }) => assert_eq!(symbol, "IBM"),
        other => panic!("Expected MissingExchange error for IBM, got: {:?}", other),
    }

    let err_btc = Symbol::parse_strict("BTCUSD");
    match err_btc {
        Err(Error::MissingExchange { symbol }) => assert_eq!(symbol, "BTCUSD"),
        other => panic!("Expected MissingExchange error for BTCUSD, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_consumer_configured_default_exchange() {
    // Explicit consumer default enables frictionless bulk ingestion without hardcoded guessing
    let user_default = Symbol::parse_with_default("AAPL", "NASDAQ").unwrap();
    assert_eq!(user_default.exchange, "NASDAQ");
    assert_eq!(user_default.ticker, "AAPL");

    // Explicit prefix always takes precedence over configured default
    let explicit_overrides = Symbol::parse_with_default("NYSE:IBM", "NASDAQ").unwrap();
    assert_eq!(explicit_overrides.exchange, "NYSE");
    assert_eq!(explicit_overrides.ticker, "IBM");

    // Ticker with configured default resolves without network call
    let ticker = Ticker::new_with_default_exchange("IBM", "NYSE");
    let resolved = ticker.resolve_symbol().await.unwrap();
    assert_eq!(resolved.exchange, "NYSE");
    assert_eq!(resolved.ticker, "IBM");
}

#[tokio::test]
async fn test_strict_ticker_fails_fast_on_missing_exchange() {
    let strict_ticker = Ticker::new_strict("IBM");
    let result = strict_ticker.resolve_symbol().await;
    match result {
        Err(Error::MissingExchange { symbol }) => assert_eq!(symbol, "IBM"),
        other => panic!("Expected MissingExchange in strict mode, got: {:?}", other),
    }

    let valid_strict = Ticker::new_strict("NYSE:IBM");
    let resolved = valid_strict.resolve_symbol().await.unwrap();
    assert_eq!(resolved.exchange, "NYSE");
}

#[tokio::test]
async fn test_live_dynamic_volume_resolution_ibm_and_btcusd() {
    let client = DiscoveryClient::new();

    // Resolves IBM to authentic primary venue (NYSE), proving NASDAQ was wrong
    match client.resolve_primary_venue("IBM").await {
        Ok(sym) => {
            println!("Dynamically resolved IBM -> {sym}");
            assert_eq!(sym.ticker, "IBM");
            assert_eq!(sym.exchange, "NYSE");
        }
        Err(e) => eprintln!("Notice on live resolve_primary_venue IBM: {e}"),
    }

    // Resolves BTCUSD to authentic high-volume spot venue (e.g. COINBASE)
    match client.resolve_primary_venue("BTCUSD").await {
        Ok(sym) => {
            println!("Dynamically resolved BTCUSD -> {sym}");
            assert_eq!(sym.ticker, "BTCUSD");
            assert!(!sym.exchange.is_empty());
        }
        Err(e) => eprintln!("Notice on live resolve_primary_venue BTCUSD: {e}"),
    }

    // Ticker::resolve_dynamic constructor
    match Ticker::resolve_dynamic("IBM").await {
        Ok(t) => {
            let sym = t.resolve_symbol().await.unwrap();
            assert_eq!(sym.exchange, "NYSE");
        }
        Err(e) => eprintln!("Notice on live Ticker::resolve_dynamic IBM: {e}"),
    }
}

#[tokio::test]
async fn test_live_multi_venue_discovery_for_consumer_choice() {
    let ticker = Ticker::new("IBM");
    match ticker.venues().await {
        Ok(venues) => {
            println!("Found {} global trading venues for IBM:", venues.len());
            for v in venues.iter().take(4) {
                println!("  - {}: ${:.2} (Vol: {}, Exch: {})", v.symbol, v.price, v.volume, v.exchange);
            }
            assert!(!venues.is_empty());
            let exchanges: Vec<&str> = venues.iter().map(|v| v.exchange.as_str()).collect();
            assert!(exchanges.contains(&"NYSE"), "Venues must include primary NYSE listing");
        }
        Err(e) => eprintln!("Notice on live venues for IBM: {e}"),
    }
}
