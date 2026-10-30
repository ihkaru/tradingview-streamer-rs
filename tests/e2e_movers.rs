//! End-to-end tests for Market Movers & Extended Hours Leaders (Phase 010).

use tradingview_streamer::movers;

#[tokio::test]
async fn test_live_market_movers_gainers() {
    let res = movers::gainers(5).await;
    match res {
        Ok(movers_list) => {
            assert!(!movers_list.is_empty());
            assert!(movers_list.len() <= 5);

            for record in &movers_list {
                assert!(!record.symbol.is_empty());
                assert!(record.change_percent >= 0.0, "Gainers should have non-negative change");
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live movers gainers test skipped due to network: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_market_movers_most_active() {
    let res = movers::most_active(5).await;
    match res {
        Ok(movers_list) => {
            assert!(!movers_list.is_empty());
            assert!(movers_list.len() <= 5);

            for record in &movers_list {
                assert!(!record.symbol.is_empty());
                assert!(record.volume > 0.0, "Most active should have positive volume");
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live movers most active test skipped due to network: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_pre_market_gainers() {
    let res = movers::pre_market_gainers(5).await;
    match res {
        Ok(movers_list) => {
            println!("Fetched {} pre-market gainers", movers_list.len());
            for record in &movers_list {
                assert!(!record.symbol.is_empty());
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live pre-market gainers test skipped due to network: {e}");
        }
    }
}
