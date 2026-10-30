//! Technical analysis recommendation ratings.

use serde::{Deserialize, Serialize};

/// Categorical summary rating evaluated by TradingView server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TechnicalRating {
    /// Strong bullish consensus (score > 0.5).
    StrongBuy,
    /// Bullish consensus (0.1 < score <= 0.5).
    Buy,
    /// Neutral or mixed consensus (-0.1 <= score <= 0.1).
    Neutral,
    /// Bearish consensus (-0.5 <= score < -0.1).
    Sell,
    /// Strong bearish consensus (score < -0.5).
    StrongSell,
}

impl TechnicalRating {
    /// Maps a raw numerical score (-1.0 to 1.0) into a categorical rating.
    #[must_use]
    pub fn from_score(score: f64) -> Self {
        if score > 0.5 {
            Self::StrongBuy
        } else if score > 0.1 {
            Self::Buy
        } else if score < -0.5 {
            Self::StrongSell
        } else if score < -0.1 {
            Self::Sell
        } else {
            Self::Neutral
        }
    }

    /// Returns human-readable representation.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::StrongBuy => "Strong Buy",
            Self::Buy => "Buy",
            Self::Neutral => "Neutral",
            Self::Sell => "Sell",
            Self::StrongSell => "Strong Sell",
        }
    }

    /// Returns true if rating represents a bullish sentiment.
    #[must_use]
    pub const fn is_bullish(&self) -> bool {
        matches!(self, Self::StrongBuy | Self::Buy)
    }

    /// Returns true if rating represents a bearish sentiment.
    #[must_use]
    pub const fn is_bearish(&self) -> bool {
        matches!(self, Self::StrongSell | Self::Sell)
    }
}
