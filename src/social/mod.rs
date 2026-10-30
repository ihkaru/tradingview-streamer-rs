pub mod client;
pub mod sentiment;
pub mod types;

pub use client::SocialClient;
pub use sentiment::{aggregate_sentiment, classify_sentiment};
pub use types::{
    IdeaSort, MindAuthor, MindPost, MindsSort, SentimentBias, SentimentReport,
    TradeIdea,
};

use crate::error::Result;

/// Convenience 1-liner to retrieve community trading ideas for a symbol.
pub async fn ideas(symbol: &str, page: u32) -> Result<Vec<TradeIdea>> {
    let client = SocialClient::new();
    client.fetch_ideas(symbol, page, IdeaSort::Popular).await
}

/// Convenience 1-liner to retrieve Minds community discussions for a symbol.
pub async fn minds(symbol: &str, limit: u32) -> Result<Vec<MindPost>> {
    let client = SocialClient::new();
    client.fetch_minds(symbol, limit, MindsSort::Recent).await
}

/// Convenience 1-liner to retrieve aggregated crowd sentiment for a symbol.
pub async fn sentiment(symbol: &str) -> Result<SentimentReport> {
    let client = SocialClient::new();
    client.fetch_sentiment(symbol).await
}
