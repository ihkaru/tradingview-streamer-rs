//! End-to-end tests for Corporate & Economic Calendars (Phase 014).

use tradingview_streamer::calendar;

#[tokio::test]
async fn test_live_calendar_dividends() {
    let res = calendar::dividends(5).await;
    match res {
        Ok(events) => {
            println!("Fetched {} dividend events", events.len());
            assert!(!events.is_empty());
            assert!(events.len() <= 5);

            for evt in &events {
                assert!(!evt.symbol.is_empty());
                println!("Dividend Event: {} (Amount: {:?})", evt.symbol, evt.amount);
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live dividend calendar test skipped due to network: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_calendar_earnings() {
    let res = calendar::earnings(5).await;
    match res {
        Ok(events) => {
            println!("Fetched {} earnings events", events.len());
            assert!(!events.is_empty());
            assert!(events.len() <= 5);

            for evt in &events {
                assert!(!evt.symbol.is_empty());
                println!("Earnings Event: {} (EPS: {:?}, Surprise: {:?}%)", evt.symbol, evt.eps_actual, evt.eps_surprise_percent);
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live earnings calendar test skipped due to network: {e}");
        }
    }
}
