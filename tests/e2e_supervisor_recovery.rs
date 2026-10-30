use std::time::Duration;
use tradingview_streamer::prelude::*;
use tradingview_streamer::supervisor::{
    CircuitBreaker, CircuitState, ExponentialBackoff, ProxyPool, SupervisedStreamer,
    SupervisorConfig,
};

#[test]
fn test_exponential_backoff_growth_and_bounds() {
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(100),
        Duration::from_millis(5000),
        2.0,
    );

    let d0 = backoff.delay_for_attempt(0);
    let d1 = backoff.delay_for_attempt(1);
    let d2 = backoff.delay_for_attempt(2);
    let d10 = backoff.delay_for_attempt(10);

    assert!(d0.as_millis() >= 50);
    assert!(d1.as_millis() >= d0.as_millis() / 2);
    assert!(d2.as_millis() >= d1.as_millis() / 2);
    assert!(d10.as_millis() <= 5000, "Must be clamped by max_delay");
}

#[test]
fn test_circuit_breaker_transitions() {
    let mut cb = CircuitBreaker::new(3, Duration::from_millis(50));
    assert_eq!(cb.state(), CircuitState::Closed);
    assert!(cb.can_execute());

    // 1st failure
    assert_eq!(cb.record_failure(), CircuitState::Closed);
    // 2nd failure
    assert_eq!(cb.record_failure(), CircuitState::Closed);
    // 3rd failure trips breaker
    assert_eq!(cb.record_failure(), CircuitState::Open);
    assert_eq!(cb.state(), CircuitState::Open);
    assert!(!cb.can_execute(), "Open circuit breaker should reject execution");

    // Wait for cooldown
    std::thread::sleep(Duration::from_millis(60));
    assert!(cb.can_execute(), "Should allow probe in HalfOpen state");
    assert_eq!(cb.state(), CircuitState::HalfOpen);

    // Success in HalfOpen closes breaker
    cb.record_success();
    assert_eq!(cb.state(), CircuitState::Closed);
    assert_eq!(cb.failures(), 0);
}

#[test]
fn test_proxy_pool_round_robin() {
    let list = vec![
        "http://proxy1:8080".to_string(),
        "http://proxy2:8080".to_string(),
        "socks5://proxy3:1080".to_string(),
    ];
    let pool = ProxyPool::new(list);
    assert_eq!(pool.len(), 3);
    assert!(!pool.is_empty());

    assert_eq!(pool.next_proxy().as_deref(), Some("http://proxy1:8080"));
    assert_eq!(pool.next_proxy().as_deref(), Some("http://proxy2:8080"));
    assert_eq!(pool.next_proxy().as_deref(), Some("socks5://proxy3:1080"));
    assert_eq!(pool.next_proxy().as_deref(), Some("http://proxy1:8080"));
}

#[tokio::test]
async fn test_live_supervised_streamer_and_ticker() {
    let config = SupervisorConfig::default();
    let streamer = SupervisedStreamer::new("BINANCE:BTCUSDT")
        .timeframe(Timeframe::M1)
        .config(config);

    let mut rx = streamer.stream_resilient();
    let timeout = Duration::from_secs(12);

    let res = tokio::time::timeout(timeout, async {
        while let Some(evt) = rx.recv().await {
            match evt {
                StreamEvent::Candles(c) => {
                    println!("Received {} candles via supervised stream", c.len());
                    return true;
                }
                StreamEvent::Quote(q) => {
                    println!("Received quote via supervised stream: ${:.2}", q.price());
                    return true;
                }
                StreamEvent::Heartbeat(_) => {}
                _ => {}
            }
        }
        false
    }).await;

    assert!(res.is_ok(), "Timed out waiting for supervised stream event");
    assert!(res.unwrap(), "Supervised stream failed to yield market event");

    // Test Ticker resilient facade
    let t = Ticker::new("BINANCE:BTCUSDT");
    let mut ticker_rx = t.stream_resilient("1m").expect("Failed to create resilient ticker stream");
    let t_res = tokio::time::timeout(Duration::from_secs(8), ticker_rx.recv()).await;
    assert!(t_res.is_ok());
}
