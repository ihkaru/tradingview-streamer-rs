//! Deep Historical Backfill & Timescale Paging Engine (Phase 003).
//!
//! Provides automated paging across 10,000+ historical bars using `request_more_data` RPC.

use std::time::Duration;
use crate::client::{ClientConfig, TvWebSocketClient};
use crate::error::{Error, Result};
use crate::protocol::messages::request_more_data;
use crate::session::SessionTokens;
use crate::types::{OhlcvCandle, SessionType, StreamEvent, Symbol, Timeframe};

/// Paging coordinator for retrieving extensive historical datasets.
#[derive(Debug, Clone)]
pub struct BackfillPager {
    symbol: Symbol,
    timeframe: Timeframe,
    session_type: SessionType,
    timeout: Duration,
    client: TvWebSocketClient,
}

impl BackfillPager {
    /// Creates a new backfill pager with the given settings.
    pub fn new(
        symbol: Symbol,
        timeframe: Timeframe,
        session_type: SessionType,
        timeout: Duration,
    ) -> Self {
        let config = ClientConfig {
            connect_timeout: timeout,
            tokens: SessionTokens::new(),
            ..Default::default()
        };
        Self {
            symbol,
            timeframe,
            session_type,
            timeout,
            client: TvWebSocketClient::new(config),
        }
    }

    /// Attaches a custom WebSocket client configuration (e.g. for mock servers or custom endpoints).
    pub fn with_client(mut self, client: TvWebSocketClient) -> Self {
        self.client = client;
        self
    }

    /// Iteratively fetches historical candles backwards until `target_count` is reached or genesis is reached.
    pub async fn fetch_deep_history(&self, target_count: usize) -> Result<Vec<OhlcvCandle>> {
        let initial_batch = (target_count as u32).min(5000);
        let (cmd_tx, mut event_rx, tokens) = self
            .client
            .start_interactive_streaming(
                &self.symbol,
                &self.timeframe,
                self.session_type,
                initial_batch,
            )
            .await?;

        let mut collected: Vec<OhlcvCandle> = Vec::with_capacity(target_count);
        let mut stagnant_rounds = 0;
        let mut last_min_index: Option<i64> = None;
        let deadline = tokio::time::Instant::now() + self.timeout;

        while collected.len() < target_count && stagnant_rounds < 3 {
            let got_new = self.drain_candle_stream(&mut event_rx, &mut collected).await;
            self.dedup_and_sort(&mut collected);

            if collected.len() >= target_count {
                break;
            }

            let current_min_index = collected.first().map(|c| c.index);
            if current_min_index == last_min_index && !got_new {
                stagnant_rounds += 1;
            } else {
                stagnant_rounds = 0;
                last_min_index = current_min_index;
            }

            if tokio::time::Instant::now() >= deadline {
                break;
            }

            let remaining = (target_count - collected.len()) as u32;
            let page_size = remaining.min(5000);
            let cmd = request_more_data(&tokens.chart_session, &tokens.series_id, page_size);
            if cmd_tx.send(cmd).await.is_err() {
                break;
            }
        }

        if collected.is_empty() {
            return Err(Error::DataNotFound(format!(
                "No historical candles received for {}",
                self.symbol
            )));
        }

        Ok(collected)
    }

    async fn drain_candle_stream(
        &self,
        event_rx: &mut tokio::sync::mpsc::Receiver<StreamEvent>,
        collected: &mut Vec<OhlcvCandle>,
    ) -> bool {
        let mut got_candles = false;
        let round_timeout = Duration::from_millis(1500);

        while let Ok(Some(evt)) = tokio::time::timeout(round_timeout, event_rx.recv()).await {
            if let StreamEvent::Candles(candles) = evt {
                if !candles.is_empty() {
                    got_candles = true;
                    collected.extend(candles);
                }
            }
        }
        got_candles
    }

    fn dedup_and_sort(&self, collected: &mut Vec<OhlcvCandle>) {
        collected.sort_by_key(|c| c.index);
        collected.dedup_by_key(|c| c.index);
    }
}
