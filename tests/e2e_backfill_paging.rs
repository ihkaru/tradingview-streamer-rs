//! End-to-end and hermetic tests for Deep Historical Backfill & Timescale Paging (Phase 003).

use std::time::Duration;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

use tradingview_streamer::backfill::BackfillPager;
use tradingview_streamer::client::{ClientConfig, TvWebSocketClient};
use tradingview_streamer::protocol::messages::request_more_data;
use tradingview_streamer::session::SessionTokens;
use tradingview_streamer::types::{SessionType, Symbol, Timeframe};

#[test]
fn test_request_more_data_rpc_encoding() {
    let msg = request_more_data("cs_sample123", "sds_1", 2000);
    assert!(msg.starts_with("~m~"));
    assert!(msg.contains("request_more_data"));
    assert!(msg.contains("cs_sample123"));
    assert!(msg.contains("sds_1"));
    assert!(msg.contains("2000"));
}

#[tokio::test]
async fn test_hermetic_backfill_paging_workflow() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let ws_url = format!("ws://127.0.0.1:{port}");

    tokio::spawn(async move {
        if let Ok((tcp_stream, _)) = listener.accept().await {
            let mut ws = accept_async(tcp_stream).await.unwrap();

            while let Some(msg_res) = ws.next().await {
                if let Ok(Message::Text(text)) = msg_res {
                    // Initial subscription creates series -> emit initial batch of 2 candles (index 10, 11)
                    if text.contains("create_series") {
                        let payload = r#"{"m":"timescale_update","p":["cs_test",{"sds_1":{"s":[{"i":10,"v":[1700001000,150.0,152.0,149.0,151.0,10000.0]},{"i":11,"v":[1700002000,151.0,153.0,150.0,152.0,12000.0]}]}}]}"#;
                        let framed = format!("~m~{}~m~{}", payload.len(), payload);
                        let _ = ws.send(Message::Text(framed.into())).await;
                    }
                    // When client sends request_more_data -> emit older batch of 2 candles (index 8, 9)
                    if text.contains("request_more_data") {
                        let payload = r#"{"m":"timescale_update","p":["cs_test",{"sds_1":{"s":[{"i":8,"v":[1700000000,148.0,150.0,147.0,149.0,8000.0]},{"i":9,"v":[1700000500,149.0,151.0,148.0,150.0,9000.0]}]}}]}"#;
                        let framed = format!("~m~{}~m~{}", payload.len(), payload);
                        let _ = ws.send(Message::Text(framed.into())).await;
                    }
                }
            }
        }
    });

    let config = ClientConfig {
        ws_url,
        connect_timeout: Duration::from_secs(3),
        tokens: SessionTokens::new(),
        ..Default::default()
    };
    let client = TvWebSocketClient::new(config);

    let pager = BackfillPager::new(
        Symbol::new("NASDAQ", "AAPL"),
        Timeframe::M1,
        SessionType::Regular,
        Duration::from_secs(4),
    )
    .with_client(client);

    let bars = pager
        .fetch_deep_history(4)
        .await
        .expect("Should backfill 4 bars across pagination");

    assert_eq!(bars.len(), 4);
    assert_eq!(bars[0].index, 8);
    assert_eq!(bars[1].index, 9);
    assert_eq!(bars[2].index, 10);
    assert_eq!(bars[3].index, 11);

    // Verify chronological order
    assert!(bars[0].timestamp < bars[1].timestamp);
    assert!(bars[1].timestamp < bars[2].timestamp);
    assert!(bars[2].timestamp < bars[3].timestamp);
}
