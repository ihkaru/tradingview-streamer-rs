//! End-to-end tests for Financial Scanner & Multi-Market Screener Engine (Phase 009).

use tradingview_streamer::scanner::{
    Filter, FilterOperation, ScannerMarket, SortConfig, SortOrder,
};
use tradingview_streamer::ScannerBuilder;

#[test]
fn test_filter_dsl_construction() {
    let f1 = Filter::new("close").greater(150.0);
    assert_eq!(f1.left, "close");
    assert_eq!(f1.operation, FilterOperation::Greater);
    assert_eq!(f1.right, serde_json::json!(150.0));

    let f2 = Filter::new("volume").equal_or_greater(1_000_000);
    assert_eq!(f2.left, "volume");
    assert_eq!(f2.operation, FilterOperation::EqualOrGreater);

    let f3 = Filter::new("RSI").in_range(30.0, 70.0);
    assert_eq!(f3.left, "RSI");
    assert_eq!(f3.operation, FilterOperation::InRange);
    assert_eq!(f3.right, serde_json::json!([30.0, 70.0]));
}

#[test]
fn test_scanner_market_urls() {
    assert_eq!(
        ScannerMarket::America.scan_url(),
        "https://scanner.tradingview.com/america/scan"
    );
    assert_eq!(
        ScannerMarket::Crypto.scan_url(),
        "https://scanner.tradingview.com/crypto/scan"
    );
    assert_eq!(
        ScannerMarket::Forex.scan_url(),
        "https://scanner.tradingview.com/forex/scan"
    );
    assert_eq!(
        ScannerMarket::Global.scan_url(),
        "https://scanner.tradingview.com/global/scan"
    );
}

#[test]
fn test_sort_config_serialization() {
    let sort = SortConfig::desc("volume");
    assert_eq!(sort.sort_by, "volume");
    assert_eq!(sort.sort_order, SortOrder::Desc);

    let serialized = serde_json::to_string(&sort).unwrap();
    assert!(serialized.contains(r#""sortBy":"volume""#));
    assert!(serialized.contains(r#""sortOrder":"desc""#));
}

#[tokio::test]
async fn test_live_scanner_query_us_stocks() {
    let response = ScannerBuilder::new(ScannerMarket::America)
        .filter(Filter::new("close").greater(50.0))
        .filter(Filter::new("volume").greater(100_000.0))
        .sort_by("volume", SortOrder::Desc)
        .limit(5)
        .execute()
        .await;

    match response {
        Ok(res) => {
            assert!(res.total_count > 0);
            assert!(!res.items.is_empty());
            assert!(res.items.len() <= 5);

            for item in &res.items {
                assert!(!item.symbol.is_empty());
                let close = item.close().unwrap_or(0.0);
                let volume = item.volume().unwrap_or(0.0);
                assert!(close >= 50.0, "Expected close >= 50, got {}", close);
                assert!(volume >= 100_000.0, "Expected volume >= 100k, got {}", volume);
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live scanner test skipped or failed due to network: {e}");
        }
    }
}
