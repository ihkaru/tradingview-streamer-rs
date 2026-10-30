use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use super::types::MultiQuoteEvent;
use crate::types::QuoteUpdate;

/// Thread-safe symbol routing registry for multi-symbol streams.
#[derive(Debug, Clone, Default)]
pub struct SymbolRouter {
    global_tx: Arc<Mutex<Option<mpsc::Sender<MultiQuoteEvent>>>>,
    symbol_channels: Arc<Mutex<HashMap<String, Vec<mpsc::Sender<QuoteUpdate>>>>>,
}

impl SymbolRouter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the unified multi-event channel sender.
    pub fn set_global_sender(&self, tx: mpsc::Sender<MultiQuoteEvent>) {
        if let Ok(mut lock) = self.global_tx.lock() {
            *lock = Some(tx);
        }
    }

    /// Subscribes an individual channel to a specific symbol.
    pub fn subscribe(&self, symbol: &str, tx: mpsc::Sender<QuoteUpdate>) {
        if let Ok(mut map) = self.symbol_channels.lock() {
            map.entry(symbol.to_string()).or_default().push(tx);
        }
    }

    /// Unsubscribes all listeners for a symbol.
    pub fn unsubscribe(&self, symbol: &str) {
        if let Ok(mut map) = self.symbol_channels.lock() {
            map.remove(symbol);
        }
    }

    /// Dispatches a newly received quote update to all registered subscribers.
    pub async fn dispatch(&self, symbol: &str, quote: QuoteUpdate) {
        // Dispatch to unified global channel if active
        let global_sender = self.global_tx.lock().ok().and_then(|g| g.clone());
        if let Some(gtx) = global_sender {
            let _ = gtx.send(MultiQuoteEvent {
                symbol: symbol.to_string(),
                quote: quote.clone(),
            }).await;
        }

        // Dispatch to symbol-specific channels
        let senders = self.symbol_channels.lock().ok().and_then(|map| {
            map.get(symbol).cloned()
        });

        if let Some(list) = senders {
            for tx in list {
                let _ = tx.send(quote.clone()).await;
            }
        }
    }
}
