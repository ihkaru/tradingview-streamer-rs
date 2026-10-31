use std::str::FromStr;
use tradingview_streamer::{
    download_adjusted, download_period, download_range, DateRange, Error, Period,
    PriceAdjustment, SessionType, Ticker,
};
use tradingview_streamer::protocol::messages::build_symbol_spec;

#[test]
fn test_price_adjustment_wire_mapping() {
    assert_eq!(PriceAdjustment::Splits.as_str(), "splits");
    assert_eq!(PriceAdjustment::Dividends.as_str(), "dividends");
    assert_eq!(PriceAdjustment::None.as_str(), "none");

    assert_eq!(
        PriceAdjustment::from_str("dividends").unwrap(),
        PriceAdjustment::Dividends
    );
    assert_eq!(
        PriceAdjustment::from_str("splits").unwrap(),
        PriceAdjustment::Splits
    );
    assert_eq!(
        PriceAdjustment::from_str("none").unwrap(),
        PriceAdjustment::None
    );
    assert!(PriceAdjustment::from_str("invalid").is_err());

    let spec_div = build_symbol_spec("NASDAQ:AAPL", SessionType::Regular, PriceAdjustment::Dividends);
    assert!(spec_div.contains(r#""adjustment":"dividends""#));
    assert!(spec_div.contains(r#""session":"regular""#));
    assert!(spec_div.contains(r#""symbol":"NASDAQ:AAPL""#));

    let spec_raw = build_symbol_spec("NYSE:IBM", SessionType::Extended, PriceAdjustment::None);
    assert!(spec_raw.contains(r#""adjustment":"none""#));
    assert!(spec_raw.contains(r#""session":"extended""#));
}

#[test]
fn test_date_range_parsing_and_invariants() {
    let range = DateRange::parse_dates("2025-01-01", "2025-01-31").expect("valid date range");
    assert!(range.start_timestamp < range.end_timestamp);
    assert_eq!(range.start_timestamp, 1735689600);
    assert_eq!(range.end_timestamp, 1738367999);

    // Inverted range must fail fast
    let err_inverted = DateRange::parse_dates("2025-02-01", "2025-01-01").unwrap_err();
    match err_inverted {
        Error::InvalidParameter(msg) => assert!(msg.contains("cannot be after end")),
        other => panic!("Unexpected error variant: {other:?}"),
    }

    // Invalid format must fail fast
    let err_format = DateRange::parse_dates("01-01-2025", "2025-01-31").unwrap_err();
    match err_format {
        Error::InvalidParameter(msg) => assert!(msg.contains("YYYY-MM-DD")),
        other => panic!("Unexpected error variant: {other:?}"),
    }
}

#[test]
fn test_relative_period_parsing() {
    let periods = ["1d", "5d", "1mo", "3mo", "6mo", "1y", "2y", "5y", "10y", "ytd", "max"];
    for p_str in periods {
        let p = Period::from_str(p_str).unwrap_or_else(|_| panic!("Failed to parse {p_str}"));
        let range = p.to_date_range();
        assert!(range.start_timestamp < range.end_timestamp);
    }

    assert!(Period::from_str("invalid_period").is_err());
}

#[tokio::test]
async fn test_live_history_adjusted_dividends() {
    let ticker = Ticker::new("NASDAQ:AAPL");
    let result = ticker.history_adjusted("1D", 10, PriceAdjustment::Dividends).await;
    match result {
        Ok(candles) => {
            assert!(!candles.is_empty());
            let first = &candles[0];
            assert!(first.close > 0.0);
            assert!(first.volume.unwrap_or(0.0) >= 0.0);
        }
        Err(e) => {
            eprintln!("Live network warning (can occur in restricted sandboxes): {e}");
        }
    }
}

#[tokio::test]
async fn test_live_download_range_convenience() {
    let result = download_range("NASDAQ:AAPL", "1D", "2024-01-01", "2024-01-15").await;
    match result {
        Ok(candles) => {
            assert!(!candles.is_empty());
            for c in &candles {
                assert!(c.timestamp >= 1704067200);
                assert!(c.timestamp <= 1705363199);
            }
        }
        Err(e) => {
            eprintln!("Live network warning (can occur in restricted sandboxes): {e}");
        }
    }
}

#[tokio::test]
async fn test_live_download_period_convenience() {
    let result = download_period("NASDAQ:AAPL", "1D", "5d").await;
    match result {
        Ok(candles) => {
            assert!(!candles.is_empty());
            assert!(candles.len() <= 10);
        }
        Err(e) => {
            eprintln!("Live network warning (can occur in restricted sandboxes): {e}");
        }
    }
}

#[tokio::test]
async fn test_live_download_adjusted_convenience() {
    let result = download_adjusted("NASDAQ:AAPL", "1D", 10, PriceAdjustment::Dividends).await;
    match result {
        Ok(candles) => {
            assert!(!candles.is_empty());
        }
        Err(e) => {
            eprintln!("Live network warning (can occur in restricted sandboxes): {e}");
        }
    }
}
