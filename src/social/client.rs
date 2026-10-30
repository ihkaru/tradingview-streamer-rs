use std::collections::HashMap;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use serde_json::Value;

use super::sentiment::{aggregate_sentiment, classify_sentiment};
use super::types::{
    IdeaSort, MindAuthor, MindPost, MindsSort, SentimentReport, TradeIdea,
};
use crate::error::{Error, Result};

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

#[derive(Debug, Deserialize)]
struct RawIdeasResponse {
    #[serde(default)]
    data: Option<RawIdeasWrapper>,
}

#[derive(Debug, Deserialize)]
struct RawIdeasWrapper {
    #[serde(default)]
    ideas: Option<RawIdeasInner>,
}

#[derive(Debug, Deserialize)]
struct RawIdeasInner {
    #[serde(default)]
    data: Option<RawIdeasData>,
}

#[derive(Debug, Deserialize)]
struct RawIdeasData {
    #[serde(default)]
    items: Vec<RawIdeaItem>,
}

#[derive(Debug, Deserialize)]
struct RawIdeaItem {
    id: Option<Value>,
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    chart_url: String,
    #[serde(default)]
    comments_count: u64,
    #[serde(default)]
    views_count: u64,
    #[serde(default)]
    likes_count: u64,
    #[serde(default)]
    date_timestamp: i64,
    #[serde(default)]
    user: Option<RawIdeaUser>,
    #[serde(default)]
    image: Option<RawIdeaImage>,
}

#[derive(Debug, Deserialize)]
struct RawIdeaUser {
    #[serde(default)]
    username: String,
}

#[derive(Debug, Deserialize)]
struct RawIdeaImage {
    #[serde(default)]
    big: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawMindsResponse {
    #[serde(default)]
    results: Vec<RawMindItem>,
}

#[derive(Debug, Deserialize)]
struct RawMindItem {
    #[serde(default)]
    uid: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    created: String,
    #[serde(default)]
    total_likes: u64,
    #[serde(default)]
    total_comments: u64,
    #[serde(default)]
    author: Option<RawMindAuthor>,
    #[serde(default)]
    symbols: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct RawMindAuthor {
    #[serde(default)]
    username: String,
    #[serde(default)]
    uri: String,
    #[serde(default)]
    is_broker: bool,
}

/// Client for scraping TradingView social ideas, community discussions, and crowd sentiment.
#[derive(Debug, Clone)]
pub struct SocialClient {
    client: reqwest::Client,
}

impl Default for SocialClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SocialClient {
    #[must_use]
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { client }
    }

    /// Retrieve published trading ideas for a given symbol and page.
    pub async fn fetch_ideas(
        &self,
        symbol: &str,
        page: u32,
        sort: IdeaSort,
    ) -> Result<Vec<TradeIdea>> {
        let clean_sym = if let Some((_, t)) = symbol.split_once(':') {
            t.trim()
        } else {
            symbol.trim()
        };
        let url = if page <= 1 {
            format!("https://www.tradingview.com/symbols/{clean_sym}/ideas/?component-data-only=1")
        } else {
            format!("https://www.tradingview.com/symbols/{clean_sym}/ideas/page-{page}/?component-data-only=1")
        };

        let mut req = self.client.get(&url);
        if sort == IdeaSort::Recent {
            req = req.query(&[("sort", "recent")]);
        }

        let resp = req.send().await.map_err(|e| Error::Protocol(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(Error::Protocol(format!(
                "TradingView ideas returned HTTP {}",
                resp.status()
            )));
        }

        let payload: RawIdeasResponse = resp.json().await.map_err(|e| Error::Protocol(e.to_string()))?;
        let items = payload
            .data
            .and_then(|w| w.ideas)
            .and_then(|i| i.data)
            .map(|d| d.items)
            .unwrap_or_default();

        let mut ideas = Vec::with_capacity(items.len());
        for item in items {
            let author = item.user.map(|u| u.username).unwrap_or_default();
            let preview_image = item.image.and_then(|img| img.big);
            let id = item.id.map(|v| match v {
                Value::Number(n) => n.to_string(),
                Value::String(s) => s,
                other => other.to_string(),
            });
            let sentiment = classify_sentiment(&item.name, &item.description);

            ideas.push(TradeIdea {
                id,
                title: item.name,
                description: item.description,
                author,
                chart_url: item.chart_url,
                comments_count: item.comments_count,
                views_count: item.views_count,
                likes_count: item.likes_count,
                timestamp: item.date_timestamp,
                preview_image,
                sentiment,
            });
        }

        Ok(ideas)
    }

    /// Retrieve Minds community micro-discussions for a given symbol.
    pub async fn fetch_minds(
        &self,
        symbol: &str,
        limit: u32,
        sort: MindsSort,
    ) -> Result<Vec<MindPost>> {
        let target_symbol = symbol.trim();
        let url = "https://www.tradingview.com/api/v1/minds/";

        let resp = self
            .client
            .get(url)
            .query(&[
                ("symbol", target_symbol),
                ("limit", &limit.to_string()),
                ("sort", sort.as_str()),
            ])
            .send()
            .await
            .map_err(|e| Error::Protocol(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(Error::Protocol(format!(
                "TradingView minds returned HTTP {}",
                resp.status()
            )));
        }

        let payload: RawMindsResponse = resp.json().await.map_err(|e| Error::Protocol(e.to_string()))?;
        let mut minds = Vec::with_capacity(payload.results.len());

        for item in payload.results {
            let (username, profile_url, is_broker) = match item.author {
                Some(a) => {
                    let p_url = format!("https://www.tradingview.com{}", a.uri);
                    (a.username, p_url, a.is_broker)
                }
                None => (String::new(), String::new(), false),
            };

            let symbols = item
                .symbols
                .map(|m| m.into_values().collect())
                .unwrap_or_default();

            minds.push(MindPost {
                uid: item.uid,
                text: item.text,
                url: item.url,
                author: MindAuthor {
                    username,
                    profile_url,
                    is_broker,
                },
                created: item.created,
                symbols,
                total_likes: item.total_likes,
                total_comments: item.total_comments,
            });
        }

        Ok(minds)
    }

    /// Retrieve crowd sentiment report by sampling ideas for a symbol.
    pub async fn fetch_sentiment(&self, symbol: &str) -> Result<SentimentReport> {
        let ideas = self.fetch_ideas(symbol, 1, IdeaSort::Popular).await?;
        Ok(aggregate_sentiment(symbol, &ideas))
    }
}
