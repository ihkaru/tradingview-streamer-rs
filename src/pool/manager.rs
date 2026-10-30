use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use super::multiplexer::SymbolRouter;
use super::types::{MultiQuoteEvent, PoolConfig};
use super::worker::PoolWorker;
use crate::error::Result;
use crate::types::Symbol;

/// Dynamic connection pool distributing multi-symbol quote subscriptions across managed WebSockets.
#[derive(Debug, Clone)]
pub struct ConnectionPool {
    config: PoolConfig,
    router: SymbolRouter,
    state: Arc<Mutex<PoolState>>,
}

#[derive(Debug, Default)]
struct PoolState {
    workers: Vec<Arc<PoolWorker>>,
    symbol_to_worker: HashMap<String, usize>,
    worker_loads: Vec<usize>,
}

impl ConnectionPool {
    #[must_use]
    pub fn new(config: PoolConfig) -> Self {
        Self {
            config,
            router: SymbolRouter::new(),
            state: Arc::new(Mutex::new(PoolState::default())),
        }
    }

    /// Subscribes a single symbol to the connection pool.
    pub async fn subscribe(&self, symbol: &str) -> Result<()> {
        let sym_obj = Symbol::resolve(symbol);
        let normalized = sym_obj.to_exchange_symbol();

        let mut state = self.state.lock().await;
        if state.symbol_to_worker.contains_key(&normalized) {
            return Ok(());
        }

        let worker_idx = self.get_or_create_worker(&mut state).await?;
        let worker = Arc::clone(&state.workers[worker_idx]);

        worker.subscribe(&normalized).await?;
        state.symbol_to_worker.insert(normalized, worker_idx);
        state.worker_loads[worker_idx] += 1;

        Ok(())
    }

    /// Subscribes a batch of symbols across the connection pool.
    pub async fn subscribe_many(&self, symbols: &[impl AsRef<str>]) -> Result<()> {
        for s in symbols {
            self.subscribe(s.as_ref()).await?;
        }
        Ok(())
    }

    /// Unsubscribes a symbol from the connection pool.
    pub async fn unsubscribe(&self, symbol: &str) -> Result<()> {
        let sym_obj = Symbol::resolve(symbol);
        let normalized = sym_obj.to_exchange_symbol();

        let mut state = self.state.lock().await;
        if let Some(worker_idx) = state.symbol_to_worker.remove(&normalized) {
            if let Some(worker) = state.workers.get(worker_idx) {
                let _ = worker.unsubscribe(&normalized).await;
            }
            if let Some(load) = state.worker_loads.get_mut(worker_idx) {
                *load = load.saturating_sub(1);
            }
            self.router.unsubscribe(&normalized);
        }

        Ok(())
    }

    /// Opens a unified real-time stream receiving quotes from all pool subscriptions.
    pub async fn stream(&self) -> mpsc::Receiver<MultiQuoteEvent> {
        let (tx, rx) = mpsc::channel(1024);
        self.router.set_global_sender(tx);
        rx
    }

    /// Returns the total number of active symbols managed by the pool.
    pub async fn active_symbol_count(&self) -> usize {
        self.state.lock().await.symbol_to_worker.len()
    }

    /// Returns the total number of underlying WebSocket workers spawned.
    pub async fn worker_count(&self) -> usize {
        self.state.lock().await.workers.len()
    }

    async fn get_or_create_worker(&self, state: &mut PoolState) -> Result<usize> {
        // Find existing worker with spare capacity
        for (idx, &load) in state.worker_loads.iter().enumerate() {
            if load < self.config.max_symbols_per_socket {
                return Ok(idx);
            }
        }

        // Spawn new worker
        let next_id = state.workers.len();
        let worker = PoolWorker::spawn(next_id, self.config.clone(), self.router.clone()).await?;
        state.workers.push(Arc::new(worker));
        state.worker_loads.push(0);

        Ok(next_id)
    }
}
