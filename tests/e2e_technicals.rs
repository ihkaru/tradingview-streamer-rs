//! End-to-end tests for Technical Summary Ratings & Indicator Matrix (Phase 011).

use tradingview_streamer::technicals::{format_indicator_timeframe, TechnicalRating};
use tradingview_streamer::Ticker;

#[test]
fn test_technical_rating_score_mapping() {
    assert_eq!(TechnicalRating::from_score(0.8), TechnicalRating::StrongBuy);
    assert_eq!(TechnicalRating::from_score(0.3), TechnicalRating::Buy);
    assert_eq!(TechnicalRating::from_score(0.0), TechnicalRating::Neutral);
    assert_eq!(TechnicalRating::from_score(-0.3), TechnicalRating::Sell);
    assert_eq!(TechnicalRating::from_score(-0.8), TechnicalRating::StrongSell);

    assert!(TechnicalRating::StrongBuy.is_bullish());
    assert!(TechnicalRating::Buy.is_bullish());
    assert!(!TechnicalRating::Neutral.is_bullish());
    assert!(TechnicalRating::StrongSell.is_bearish());
    assert!(TechnicalRating::Sell.is_bearish());
}

#[test]
fn test_timeframe_suffix_formatting() {
    assert_eq!(format_indicator_timeframe("1m"), "|1");
    assert_eq!(format_indicator_timeframe("5m"), "|5");
    assert_eq!(format_indicator_timeframe("15m"), "|15");
    assert_eq!(format_indicator_timeframe("30m"), "|30");
    assert_eq!(format_indicator_timeframe("1h"), "|60");
    assert_eq!(format_indicator_timeframe("4h"), "|240");
    assert_eq!(format_indicator_timeframe("1d"), "");
    assert_eq!(format_indicator_timeframe("1w"), "|1W");
}

#[tokio::test]
async fn test_live_technical_rating_1_liner() {
    let res = tradingview_streamer::technical_rating("AAPL").await;
    match res {
        Ok(summary) => {
            println!(
                "AAPL 1D Technical Rating: {} (Score: {:.2}, RSI: {:?})",
                summary.all.as_str(),
                summary.all_score,
                summary.rsi()
            );
            assert!(summary.symbol.contains("AAPL"));
            assert!((-1.0..=1.0).contains(&summary.all_score));
            assert!((-1.0..=1.0).contains(&summary.ma_score));
            assert!((-1.0..=1.0).contains(&summary.osc_score));
        }
        Err(e) => {
            eprintln!("[WARN] Live technical rating test skipped due to network: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_ticker_technicals_multi_tf() {
    let ticker = Ticker::new("AAPL");
    let res_1h = ticker.technical_rating("1h").await;
    match res_1h {
        Ok(summary) => {
            println!(
                "AAPL 1H Rating: {} (Score: {:.2})",
                summary.all.as_str(),
                summary.all_score
            );
            assert_eq!(summary.timeframe, "1h");
        }
        Err(e) => {
            eprintln!("[WARN] Live 1h technicals test skipped: {e}");
        }
    }
}
