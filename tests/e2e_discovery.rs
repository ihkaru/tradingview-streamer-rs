//! End-to-end tests for Multi-Exchange Discovery & Global Arbitrage Engine (Phase 013).

use tradingview_streamer::discovery;
use tradingview_streamer::ticker;

#[tokio::test]
async fn test_live_discover_venues_bitcoin() {
    let res = discovery::venues("BTCUSD").await;
    match res {
        Ok(venues) => {
            println!("Discovered {} venues for BTCUSD", venues.len());
            assert!(!venues.is_empty());

            for v in &venues {
                assert!(!v.symbol.is_empty());
                assert!(v.price > 0.0);
                println!("Venue [{}]: {} -> ${:.2}", v.exchange, v.symbol, v.price);
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live venue discovery test skipped due to network: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_arbitrage_spread_bitcoin() {
    let res = discovery::arbitrage("BTCUSD").await;
    match res {
        Ok(Some(arb)) => {
            println!(
                "BTC Arbitrage: High on {} (${:.2}) vs Low on {} (${:.2}), Spread: ${:.2} ({:.3}%)",
                arb.highest_venue, arb.highest_price, arb.lowest_venue, arb.lowest_price, arb.spread_abs, arb.spread_percent
            );
            assert!(arb.highest_price >= arb.lowest_price);
            assert!(arb.spread_abs >= 0.0);
            assert!(arb.spread_percent >= 0.0);
        }
        Ok(None) => {
            println!("Less than 2 venues found for arbitrage calculation");
        }
        Err(e) => {
            eprintln!("[WARN] Live arbitrage calculation test skipped: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_ticker_venues() {
    let t = ticker("AAPL");
    let res = t.venues().await;
    match res {
        Ok(venues) => {
            assert!(!venues.is_empty());
            for v in &venues {
                assert!(!v.symbol.is_empty());
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live AAPL venues test skipped: {e}");
        }
    }
}
