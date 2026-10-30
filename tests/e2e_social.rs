use tradingview_streamer::prelude::*;
use tradingview_streamer::social::{
    aggregate_sentiment, classify_sentiment, IdeaSort, MindsSort, SentimentBias,
    SocialClient, TradeIdea,
};

#[test]
fn test_sentiment_classification_and_aggregation() {
    let bias_bull = classify_sentiment(
        "BITCOIN Massive Breakout Imminent! Strong Buy Signal",
        "Target up to 120k rally with strong volume accumulation",
    );
    assert_eq!(bias_bull, SentimentBias::Bullish);

    let bias_bear = classify_sentiment(
        "Crash Alert: Severe Breakdown in Progress, Sell Now",
        "Target down to 40k as distribution continues under resistance",
    );
    assert_eq!(bias_bear, SentimentBias::Bearish);

    let bias_neutral = classify_sentiment("AAPL Consolidated Range", "Waiting for catalyst");
    assert_eq!(bias_neutral, SentimentBias::Neutral);

    let ideas = vec![
        TradeIdea {
            id: Some("1".into()),
            title: "Bullish Setup".into(),
            description: "Buy rally".into(),
            author: "trader1".into(),
            chart_url: "https://chart/1".into(),
            comments_count: 5,
            views_count: 100,
            likes_count: 20,
            timestamp: 1700000000,
            preview_image: None,
            sentiment: SentimentBias::Bullish,
        },
        TradeIdea {
            id: Some("2".into()),
            title: "Rally to moon".into(),
            description: "Accumulate more".into(),
            author: "trader2".into(),
            chart_url: "https://chart/2".into(),
            comments_count: 2,
            views_count: 50,
            likes_count: 10,
            timestamp: 1700000010,
            preview_image: None,
            sentiment: SentimentBias::Bullish,
        },
        TradeIdea {
            id: Some("3".into()),
            title: "Neutral range".into(),
            description: "Holding".into(),
            author: "trader3".into(),
            chart_url: "https://chart/3".into(),
            comments_count: 1,
            views_count: 25,
            likes_count: 3,
            timestamp: 1700000020,
            preview_image: None,
            sentiment: SentimentBias::Neutral,
        },
    ];

    let report = aggregate_sentiment("BTCUSD", &ideas);
    assert_eq!(report.symbol, "BTCUSD");
    assert_eq!(report.total_sample_count, 3);
    assert_eq!(report.bullish_count, 2);
    assert_eq!(report.bearish_count, 0);
    assert_eq!(report.neutral_count, 1);
    assert!(report.score > 0.0);
    assert_eq!(report.consensus, SentimentBias::Bullish);
}

#[tokio::test]
async fn test_live_ideas_fetch() {
    let client = SocialClient::new();
    let ideas = client.fetch_ideas("BTCUSD", 1, IdeaSort::Popular).await;
    assert!(ideas.is_ok(), "Failed to fetch ideas: {:?}", ideas.err());
    let ideas = ideas.unwrap();
    assert!(!ideas.is_empty(), "Ideas list should not be empty");

    let first = &ideas[0];
    assert!(!first.title.is_empty(), "Idea title should not be empty");
    assert!(!first.author.is_empty(), "Author should not be empty");
    assert!(first.timestamp > 0, "Timestamp should be valid");

    // Test convenience 1-liner
    let convenience_ideas = tradingview_streamer::ideas("BTCUSD", 1).await;
    assert!(convenience_ideas.is_ok());
}

#[tokio::test]
async fn test_live_minds_fetch() {
    let client = SocialClient::new();
    let minds = client.fetch_minds("NASDAQ:AAPL", 5, MindsSort::Recent).await;
    assert!(minds.is_ok(), "Failed to fetch minds: {:?}", minds.err());
    let minds = minds.unwrap();
    assert!(!minds.is_empty(), "Minds list should not be empty");

    let first = &minds[0];
    assert!(!first.uid.is_empty(), "Mind UID should not be empty");
    assert!(!first.author.username.is_empty(), "Author username should not be empty");

    // Test convenience 1-liner
    let convenience_minds = tradingview_streamer::minds("NASDAQ:AAPL", 5).await;
    assert!(convenience_minds.is_ok());
}

#[tokio::test]
async fn test_live_sentiment_and_ticker_integration() {
    let t = Ticker::new("BTCUSD");
    let sentiment = t.sentiment().await;
    assert!(sentiment.is_ok(), "Failed to fetch sentiment: {:?}", sentiment.err());
    let report = sentiment.unwrap();
    assert_eq!(report.symbol, "BTCUSD");
    assert!(report.total_sample_count > 0);

    let ideas = t.ideas(1).await;
    assert!(ideas.is_ok());
    assert!(!ideas.unwrap().is_empty());

    let minds = t.minds(5).await;
    assert!(minds.is_ok());
    assert!(!minds.unwrap().is_empty());
}
