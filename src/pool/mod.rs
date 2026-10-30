pub mod manager;
pub mod multiplexer;
pub mod types;
pub mod worker;

pub use manager::ConnectionPool;
pub use multiplexer::SymbolRouter;
pub use types::{MultiQuoteEvent, PoolConfig};

use tokio::sync::mpsc;
use crate::error::Result;

/// Convenience function to stream real-time quote updates for a portfolio of symbols across a pooled connection.
pub async fn portfolio_quotes(
    symbols: &[impl AsRef<str>],
) -> Result<(ConnectionPool, mpsc::Receiver<MultiQuoteEvent>)> {
    let pool = ConnectionPool::new(PoolConfig::default());
    let rx = pool.stream().await;
    pool.subscribe_many(symbols).await?;
    Ok((pool, rx))
}
