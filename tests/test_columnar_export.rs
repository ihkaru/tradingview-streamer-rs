use tradingview_streamer::prelude::*;
use tradingview_streamer::export::ColumnarCandles;

#[test]
fn test_columnar_conversion_and_csv_export() {
    let candles = vec![
        OhlcvCandle {
            index: 1,
            timestamp: 1700000000,
            open: 150.0,
            high: 155.0,
            low: 149.0,
            close: 154.0,
            volume: Some(1000.0),
        },
        OhlcvCandle {
            index: 2,
            timestamp: 1700000060,
            open: 154.0,
            high: 158.0,
            low: 153.5,
            close: 157.0,
            volume: Some(2500.0),
        },
        OhlcvCandle {
            index: 3,
            timestamp: 1700000120,
            open: 157.0,
            high: 157.5,
            low: 152.0,
            close: 153.0,
            volume: None,
        },
    ];

    let col = ColumnarCandles::from_candles(&candles);
    assert_eq!(col.len(), 3);
    assert!(!col.is_empty());
    assert_eq!(col.timestamps, vec![1700000000, 1700000060, 1700000120]);
    assert_eq!(col.opens, vec![150.0, 154.0, 157.0]);
    assert_eq!(col.highs, vec![155.0, 158.0, 157.5]);
    assert_eq!(col.lows, vec![149.0, 153.5, 152.0]);
    assert_eq!(col.closes, vec![154.0, 157.0, 153.0]);
    assert_eq!(col.volumes, vec![Some(1000.0), Some(2500.0), None]);

    let csv = col.to_csv();
    assert!(csv.starts_with("timestamp,open,high,low,close,volume\n"));
    assert!(csv.contains("1700000000,150,155,149,154,1000\n"));
    assert!(csv.contains("1700000120,157,157.5,152,153,\n"));
}

#[tokio::test]
async fn test_live_download_to_columnar() {
    let candles = tradingview_streamer::download("AAPL").await.expect("Failed to download AAPL bars");
    assert!(!candles.is_empty());

    let col = ColumnarCandles::from_candles(&candles);
    assert_eq!(col.len(), candles.len());
    assert_eq!(col.timestamps.len(), candles.len());
    assert_eq!(col.closes.len(), candles.len());

    let csv = col.to_csv();
    assert!(csv.lines().count() > 50);
}
