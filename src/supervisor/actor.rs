use std::time::Duration;
use tokio::sync::mpsc;

use super::backoff::ExponentialBackoff;
use super::circuit::CircuitBreaker;
use super::proxy::ProxyPool;
use crate::streamer::{TvStreamer, TvStreamerBuilder};
use crate::types::{SessionType, StreamEvent, Timeframe};

/// Supervisor configuration controlling reconnection, backoff, and circuit breaking.
#[derive(Debug, Clone)]
pub struct SupervisorConfig {
    pub max_reconnect_attempts: u32,
    pub backoff: ExponentialBackoff,
    pub circuit_breaker: CircuitBreaker,
    pub proxy_pool: ProxyPool,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        Self {
            max_reconnect_attempts: 10,
            backoff: ExponentialBackoff::default(),
            circuit_breaker: CircuitBreaker::default(),
            proxy_pool: ProxyPool::default(),
        }
    }
}

/// Resilient supervised streamer automatically recovering from network drops and rate limits.
#[derive(Debug)]
pub struct SupervisedStreamer {
    symbol_str: String,
    timeframe: Timeframe,
    session: SessionType,
    candle_count: u32,
    config: SupervisorConfig,
}

impl SupervisedStreamer {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol_str: symbol.into(),
            timeframe: Timeframe::D1,
            session: SessionType::Regular,
            candle_count: 100,
            config: SupervisorConfig::default(),
        }
    }

    #[must_use]
    pub fn timeframe(mut self, tf: Timeframe) -> Self {
        self.timeframe = tf;
        self
    }

    #[must_use]
    pub fn session(mut self, session: SessionType) -> Self {
        self.session = session;
        self
    }

    #[must_use]
    pub fn config(mut self, config: SupervisorConfig) -> Self {
        self.config = config;
        self
    }

    /// Spawns the resilient supervisor task and returns a reliable stream of events.
    pub fn stream_resilient(self) -> mpsc::Receiver<StreamEvent> {
        let (out_tx, out_rx) = mpsc::channel(512);

        tokio::spawn(async move {
            run_supervisor_loop(
                self.symbol_str,
                self.timeframe,
                self.session,
                self.candle_count,
                self.config,
                out_tx,
            )
            .await;
        });

        out_rx
    }
}

async fn run_supervisor_loop(
    symbol_str: String,
    timeframe: Timeframe,
    session: SessionType,
    candle_count: u32,
    mut config: SupervisorConfig,
    out_tx: mpsc::Sender<StreamEvent>,
) {
    let mut attempt = 0;

    while attempt < config.max_reconnect_attempts {
        if !config.circuit_breaker.can_execute() {
            tokio::time::sleep(Duration::from_millis(500)).await;
            continue;
        }

        let mut builder = TvStreamer::builder()
            .symbol_str(&symbol_str)
            .unwrap_or_else(|_| TvStreamerBuilder::default())
            .timeframe(timeframe.clone())
            .session(session)
            .candle_count(candle_count);

        if let Some(proxy_url) = config.proxy_pool.next_proxy() {
            builder = builder.ws_url(proxy_url);
        }

        let streamer = match builder.build() {
            Ok(s) => s,
            Err(_) => break,
        };

        match streamer.stream_events().await {
            Ok(mut in_rx) => {
                config.circuit_breaker.record_success();
                attempt = 0; // Reset reconnection counter upon healthy connection

                while let Some(evt) = in_rx.recv().await {
                    if out_tx.send(evt).await.is_err() {
                        return; // Consumer disconnected
                    }
                }
                // Socket disconnected, proceed to reconnect
            }
            Err(_) => {
                config.circuit_breaker.record_failure();
            }
        }

        attempt += 1;
        let delay = config.backoff.delay_for_attempt(attempt);
        tokio::time::sleep(delay).await;
    }
}
