use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Thread-safe round-robin proxy rotator for SOCKS5 and HTTP proxies.
#[derive(Debug, Clone, Default)]
pub struct ProxyPool {
    proxies: Arc<Vec<String>>,
    cursor: Arc<AtomicUsize>,
}

impl ProxyPool {
    #[must_use]
    pub fn new(proxies: Vec<String>) -> Self {
        Self {
            proxies: Arc::new(proxies),
            cursor: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Fetches the next proxy URL in round-robin order.
    #[must_use]
    pub fn next_proxy(&self) -> Option<String> {
        if self.proxies.is_empty() {
            return None;
        }
        let idx = self.cursor.fetch_add(1, Ordering::Relaxed) % self.proxies.len();
        Some(self.proxies[idx].clone())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.proxies.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.proxies.len()
    }
}
