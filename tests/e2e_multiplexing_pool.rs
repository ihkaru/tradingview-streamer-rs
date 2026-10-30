use std::time::Duration;
use tokio::sync::mpsc;
use tradingview_streamer::pool::{ConnectionPool, PoolConfig, SymbolRouter};
use tradingview_streamer::prelude::*;

#[tokio::test]
async fn test_hermetic_symbol_router_demultiplexing() {
    let router = SymbolRouter::new();
    let (global_tx, mut global_rx) = mpsc::channel(32);
    router.set_global_sender(global_tx);

    let (aapl_tx, mut aapl_rx) = mpsc::channel(16);
    let (btc_tx, mut btc_rx) = mpsc::channel(16);

    router.subscribe("NASDAQ:AAPL", aapl_tx);
    router.subscribe("BINANCE:BTCUSDT", btc_tx);

    let mut q1 = QuoteUpdate::default();
    q1.symbol = Some("NASDAQ:AAPL".into());
    q1.last_price = Some(195.50);

    let mut q2 = QuoteUpdate::default();
    q2.symbol = Some("BINANCE:BTCUSDT".into());
    q2.last_price = Some(68500.0);

    router.dispatch("NASDAQ:AAPL", q1.clone()).await;
    router.dispatch("BINANCE:BTCUSDT", q2.clone()).await;

    // Verify symbol-specific channels received correct updates
    let recv_aapl = aapl_rx.recv().await.expect("AAPL channel should receive quote");
    assert_eq!(recv_aapl.symbol.as_deref(), Some("NASDAQ:AAPL"));
    assert_eq!(recv_aapl.last_price, Some(195.50));

    let recv_btc = btc_rx.recv().await.expect("BTC channel should receive quote");
    assert_eq!(recv_btc.symbol.as_deref(), Some("BINANCE:BTCUSDT"));
    assert_eq!(recv_btc.last_price, Some(68500.0));

    // Verify global channel received both events in sequence
    let g1 = global_rx.recv().await.expect("Global channel event 1");
    assert_eq!(g1.symbol, "NASDAQ:AAPL");
    assert_eq!(g1.quote.last_price, Some(195.50));

    let g2 = global_rx.recv().await.expect("Global channel event 2");
    assert_eq!(g2.symbol, "BINANCE:BTCUSDT");
    assert_eq!(g2.quote.last_price, Some(68500.0));
}

#[tokio::test]
async fn test_live_connection_pool_portfolio() {
    let config = PoolConfig::default().max_symbols_per_socket(5);
    let pool = ConnectionPool::new(config);
    let mut rx = pool.stream().await;

    pool.subscribe_many(&["NASDAQ:AAPL", "BINANCE:BTCUSDT"]).await
        .expect("Failed to subscribe symbols to pool");

    assert_eq!(pool.active_symbol_count().await, 2);
    assert!(pool.worker_count().await >= 1);

    // Wait for at least one live quote update from TradingView
    let timeout = Duration::from_secs(12);
    let result = tokio::time::timeout(timeout, async {
        while let Some(evt) = rx.recv().await {
            println!("Received pooled quote for {}: price=${:.2}",
                evt.symbol, evt.quote.price());
            return true;
        }
        false
    }).await;

    assert!(result.is_ok(), "Timed out waiting for pooled quote stream");
    assert!(result.unwrap(), "Stream ended without quotes");

    // Test dynamic unsubscribe
    pool.unsubscribe("NASDAQ:AAPL").await.expect("Failed to unsubscribe");
    assert_eq!(pool.active_symbol_count().await, 1);
}

#[tokio::test]
async fn test_convenience_portfolio_quotes_1_liner() {
    let (pool, mut rx) = tradingview_streamer::portfolio_quotes(&["BINANCE:BTCUSDT"]).await
        .expect("Failed to create portfolio stream");

    let timeout = Duration::from_secs(10);
    let res = tokio::time::timeout(timeout, rx.recv()).await;
    assert!(res.is_ok(), "Timed out waiting for convenience portfolio quote");
    let evt = res.unwrap().expect("Quote event expected");
    assert_eq!(evt.symbol, "BINANCE:BTCUSDT");
    assert!(evt.quote.price() > 0.0);

    drop(pool);
}
