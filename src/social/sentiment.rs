use super::types::{SentimentBias, SentimentReport, TradeIdea};

const BULLISH_KEYWORDS: &[&str] = &[
    "bullish", "breakout", "buy", "long", "rally", "target up",
    "pump", "higher high", "accumulate", "accumulation", "moon",
    "reversal up", "bottomed", "uptrend", "support held",
];

const BEARISH_KEYWORDS: &[&str] = &[
    "bearish", "breakdown", "sell", "short", "dump", "target down",
    "crash", "lower low", "distribute", "distribution", "drop",
    "reversal down", "topped", "downtrend", "resistance held",
];

/// Classify directional sentiment from title and description.
#[must_use]
pub fn classify_sentiment(title: &str, description: &str) -> SentimentBias {
    let lower_title = title.to_ascii_lowercase();
    let lower_desc = description.to_ascii_lowercase();

    let mut bull_score = 0;
    let mut bear_score = 0;

    for &kw in BULLISH_KEYWORDS {
        if lower_title.contains(kw) {
            bull_score += 2;
        }
        if lower_desc.contains(kw) {
            bull_score += 1;
        }
    }

    for &kw in BEARISH_KEYWORDS {
        if lower_title.contains(kw) {
            bear_score += 2;
        }
        if lower_desc.contains(kw) {
            bear_score += 1;
        }
    }

    if bull_score > bear_score && bull_score >= 2 {
        SentimentBias::Bullish
    } else if bear_score > bull_score && bear_score >= 2 {
        SentimentBias::Bearish
    } else {
        SentimentBias::Neutral
    }
}

/// Calculate aggregated crowd sentiment metrics over a collection of ideas.
#[must_use]
pub fn aggregate_sentiment(symbol: &str, ideas: &[TradeIdea]) -> SentimentReport {
    let total = ideas.len();
    if total == 0 {
        return SentimentReport {
            symbol: symbol.to_string(),
            total_sample_count: 0,
            bullish_count: 0,
            bearish_count: 0,
            neutral_count: 0,
            score: 0.0,
            consensus: SentimentBias::Neutral,
        };
    }

    let mut bullish = 0;
    let mut bearish = 0;
    let mut neutral = 0;

    for idea in ideas {
        match idea.sentiment {
            SentimentBias::Bullish => bullish += 1,
            SentimentBias::Bearish => bearish += 1,
            SentimentBias::Neutral => neutral += 1,
        }
    }

    let score = (bullish as f64 - bearish as f64) / (total as f64);
    let consensus = if score >= 0.15 {
        SentimentBias::Bullish
    } else if score <= -0.15 {
        SentimentBias::Bearish
    } else {
        SentimentBias::Neutral
    };

    SentimentReport {
        symbol: symbol.to_string(),
        total_sample_count: total,
        bullish_count: bullish,
        bearish_count: bearish,
        neutral_count: neutral,
        score: (score * 100.0).round() / 100.0,
        consensus,
    }
}
