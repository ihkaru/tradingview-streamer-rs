//! Models for editorial financial news and wire intelligence.

use serde::{Deserialize, Serialize};

/// Financial news article headline and metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewsArticle {
    /// Unique story identifier.
    pub id: String,
    /// Article headline.
    pub title: String,
    /// Primary news provider identifier (e.g. `"seekingalpha"`, `"reuters"`).
    pub provider: String,
    /// Publication source name (e.g. `"Seeking Alpha"`, `"Reuters"`).
    pub source: String,
    /// UNIX timestamp of publication.
    pub published: i64,
    /// Editorial urgency rating (lower numbers indicate higher urgency).
    pub urgency: i32,
    /// Web link to full article content.
    pub link: String,
    /// Related instruments tagged in story.
    pub related_symbols: Vec<String>,
}

impl NewsArticle {
    /// Formats publication date to human-readable RFC2822 or relative timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> i64 {
        self.published
    }
}
