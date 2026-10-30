pub mod actor;
pub mod backoff;
pub mod circuit;
pub mod proxy;

pub use actor::{SupervisedStreamer, SupervisorConfig};
pub use backoff::ExponentialBackoff;
pub use circuit::{CircuitBreaker, CircuitState};
pub use proxy::ProxyPool;

use tokio::sync::mpsc;
use crate::types::{SessionType, StreamEvent, Timeframe};

/// Convenience 1-liner to establish a resilient, auto-reconnecting stream for a symbol.
pub fn stream_resilient(
    symbol: impl Into<String>,
    timeframe: Timeframe,
    session: SessionType,
) -> mpsc::Receiver<StreamEvent> {
    SupervisedStreamer::new(symbol)
        .timeframe(timeframe)
        .session(session)
        .stream_resilient()
}
