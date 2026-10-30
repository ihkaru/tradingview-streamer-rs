use serde::{Deserialize, Serialize};

/// Sorting criteria for published trade ideas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum IdeaSort {
    #[default]
    Popular,
    Recent,
}

impl IdeaSort {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Popular => "popular",
            Self::Recent => "recent",
        }
    }
}

/// Sorting criteria for community minds discussions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MindsSort {
    #[default]
    Recent,
    Popular,
    Trending,
}

impl MindsSort {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recent => "recent",
            Self::Popular => "popular",
            Self::Trending => "trending",
        }
    }
}

/// Directional sentiment bias classified from title and text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum SentimentBias {
    Bullish,
    Bearish,
    #[default]
    Neutral,
}

/// Published trading setup / idea from TradingView community.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeIdea {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub author: String,
    pub chart_url: String,
    pub comments_count: u64,
    pub views_count: u64,
    pub likes_count: u64,
    pub timestamp: i64,
    pub preview_image: Option<String>,
    pub sentiment: SentimentBias,
}

/// Author details of a Minds discussion post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindAuthor {
    pub username: String,
    pub profile_url: String,
    pub is_broker: bool,
}

/// Community Minds post or micro-discussion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindPost {
    pub uid: String,
    pub text: String,
    pub url: String,
    pub author: MindAuthor,
    pub created: String,
    pub symbols: Vec<String>,
    pub total_likes: u64,
    pub total_comments: u64,
}

/// Aggregated community crowd sentiment score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentReport {
    pub symbol: String,
    pub total_sample_count: usize,
    pub bullish_count: usize,
    pub bearish_count: usize,
    pub neutral_count: usize,
    pub score: f64,
    pub consensus: SentimentBias,
}
