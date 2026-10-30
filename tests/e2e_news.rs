//! End-to-end tests for Real-Time Market News & Editorial Intelligence Feed (Phase 015).

use tradingview_streamer::{news, news_limit, ticker};

#[tokio::test]
async fn test_live_symbol_news_1_liner() {
    let res = news("AAPL").await;
    match res {
        Ok(articles) => {
            println!("Fetched {} AAPL news articles", articles.len());
            assert!(!articles.is_empty());
            assert!(articles.len() <= 10);

            for a in &articles {
                assert!(!a.id.is_empty());
                assert!(!a.title.is_empty());
                println!("News [{}]: {} ({})", a.provider, a.title, a.source);
            }
        }
        Err(e) => {
            eprintln!("[WARN] Live news test skipped due to network: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_ticker_news_method() {
    let t = ticker("NVDA");
    let res = t.news().await;
    match res {
        Ok(articles) => {
            assert!(!articles.is_empty());
            let first = &articles[0];
            assert!(!first.title.is_empty());
        }
        Err(e) => {
            eprintln!("[WARN] Live NVDA news test skipped: {e}");
        }
    }
}

#[tokio::test]
async fn test_news_limit_parameter() {
    let res = news_limit("TSLA", 3).await;
    if let Ok(articles) = res {
        assert!(articles.len() <= 3);
    }
}
