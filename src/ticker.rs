//! Ergonomic Ticker interface modeling python's `yfinance.Ticker`.

use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc;
use crate::backfill::BackfillPager;
use crate::discovery::{DiscoveryClient, MarketVenue};
use crate::error::Result;
use crate::fundamentals::{
    FinancialMetrics, FundamentalOverview, FundamentalsClient, ValuationMetrics,
};
use crate::news::{NewsArticle, NewsClient};
use crate::social::{IdeaSort, MindPost, MindsSort, SentimentReport, SocialClient, TradeIdea};
use crate::streamer::TvStreamer;
use crate::technicals::{TechnicalSummary, TechnicalsClient};
use crate::types::{OhlcvCandle, PriceAdjustment, QuoteUpdate, SessionType, StreamEvent, Symbol, Timeframe};

/// Ergonomic ticker client mirroring `yfinance.Ticker` for minimal-line integration.
#[derive(Debug, Clone)]
pub struct Ticker {
    symbol_str: String,
    default_exchange: Option<String>,
    strict: bool,
}

impl Ticker {
    /// Creates a new Ticker for the given symbol (e.g. `"NASDAQ:AAPL"`, `"NYSE:IBM"`).
    pub fn new(symbol: impl Into<String>) -> Self {
        Self { symbol_str: symbol.into(), default_exchange: None, strict: false }
    }

    /// Creates a strict Ticker requiring an explicit exchange prefix (e.g. `"NYSE:IBM"`).
    pub fn new_strict(symbol: impl Into<String>) -> Self {
        Self { symbol_str: symbol.into(), default_exchange: None, strict: true }
    }

    /// Creates a Ticker with a consumer-configured default exchange for un-prefixed symbols.
    pub fn new_with_default_exchange(
        symbol: impl Into<String>,
        default_exchange: impl Into<String>,
    ) -> Self {
        Self { symbol_str: symbol.into(), default_exchange: Some(default_exchange.into()), strict: false }
    }

    /// Asynchronously resolves the primary trading venue via TradingView's Global Scanner,
    /// returning a Ticker configured with the verified canonical exchange symbol (e.g. `"NYSE:IBM"`).
    pub async fn resolve_dynamic(symbol: &str) -> Result<Self> {
        let sym = DiscoveryClient::new().resolve_primary_venue(symbol).await?;
        Ok(Self::new(sym.to_exchange_symbol()))
    }

    /// Resolves the canonical [`Symbol`] for this ticker respecting explicit, default,
    /// or dynamic scanner resolution rules per `CON-08`.
    pub async fn resolve_symbol(&self) -> Result<Symbol> {
        if let Ok(sym) = Symbol::parse_strict(&self.symbol_str) {
            return Ok(sym);
        }

        if self.strict {
            return Err(crate::error::Error::MissingExchange {
                symbol: self.symbol_str.clone(),
            });
        }

        if let Some(ref def) = self.default_exchange {
            return Symbol::parse_with_default(&self.symbol_str, def);
        }

        DiscoveryClient::new()
            .resolve_primary_venue(&self.symbol_str)
            .await
    }

    /// Fetches historical OHLCV candles using regular market hours.
    pub async fn history(&self, timeframe: impl AsRef<str>, count: u32) -> Result<Vec<OhlcvCandle>> {
        self.fetch_internal(timeframe.as_ref(), count, SessionType::Regular, PriceAdjustment::Splits).await
    }

    /// Fetches historical OHLCV candles including pre-market and after-hours (Extended Trading Hours).
    pub async fn history_extended(&self, timeframe: impl AsRef<str>, count: u32) -> Result<Vec<OhlcvCandle>> {
        self.fetch_internal(timeframe.as_ref(), count, SessionType::Extended, PriceAdjustment::Splits).await
    }

    /// Fetches historical OHLCV candles with explicit price adjustment (e.g. `PriceAdjustment::Dividends`).
    pub async fn history_adjusted(
        &self,
        timeframe: impl AsRef<str>,
        count: u32,
        adj: PriceAdjustment,
    ) -> Result<Vec<OhlcvCandle>> {
        self.fetch_internal(timeframe.as_ref(), count, SessionType::Regular, adj).await
    }

    /// Fetches historical OHLCV candles within date boundaries `[start, end]` (e.g. `"2025-01-01"`).
    pub async fn history_range(
        &self,
        timeframe: impl AsRef<str>,
        start: &str,
        end: &str,
    ) -> Result<Vec<OhlcvCandle>> {
        self.history_range_adjusted(timeframe, start, end, PriceAdjustment::Splits).await
    }

    /// Fetches historical OHLCV candles within date boundaries with explicit price adjustment.
    pub async fn history_range_adjusted(
        &self,
        timeframe: impl AsRef<str>,
        start: &str,
        end: &str,
        adj: PriceAdjustment,
    ) -> Result<Vec<OhlcvCandle>> {
        let sym = self.resolve_symbol().await?;
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        let range = crate::range::DateRange::parse_dates(start, end)?;
        let pager = BackfillPager::new_with_adjustment(sym, tf, SessionType::Regular, adj, Duration::from_secs(30));
        pager.fetch_date_range(range.start_timestamp, range.end_timestamp).await
    }

    /// Fetches historical OHLCV candles for a relative period (e.g. `"1d"`, `"1mo"`, `"1y"`).
    pub async fn history_period(
        &self,
        timeframe: impl AsRef<str>,
        period: impl AsRef<str>,
    ) -> Result<Vec<OhlcvCandle>> {
        let p = crate::range::Period::from_str(period.as_ref())?;
        let range = p.to_date_range();
        let sym = self.resolve_symbol().await?;
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        let pager = BackfillPager::new(sym, tf, SessionType::Regular, Duration::from_secs(30));
        pager.fetch_date_range(range.start_timestamp, range.end_timestamp).await
    }

    /// Deep historical backfill fetching thousands of bars backwards using automated pagination.
    pub async fn backfill(&self, timeframe: impl AsRef<str>, target_count: usize) -> Result<Vec<OhlcvCandle>> {
        let sym = self.resolve_symbol().await?;
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        let pager = BackfillPager::new(sym, tf, SessionType::Regular, Duration::from_secs(30));
        pager.fetch_deep_history(target_count).await
    }

    /// Deep historical backfill with Extended Trading Hours.
    pub async fn backfill_extended(&self, timeframe: impl AsRef<str>, target_count: usize) -> Result<Vec<OhlcvCandle>> {
        let sym = self.resolve_symbol().await?;
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        let pager = BackfillPager::new(sym, tf, SessionType::Extended, Duration::from_secs(30));
        pager.fetch_deep_history(target_count).await
    }

    /// Fetches server-evaluated technical rating consensus and indicators for the ticker.
    pub async fn technical_rating(&self, timeframe: impl AsRef<str>) -> Result<TechnicalSummary> {
        let sym = self.resolve_symbol().await?;
        TechnicalsClient::new().summary(sym.to_exchange_symbol(), timeframe).await
    }

    /// Fetches specific indicator values for the ticker on the given timeframe.
    pub async fn indicators(
        &self,
        indicators: &[impl AsRef<str>],
        timeframe: impl AsRef<str>,
    ) -> Result<std::collections::HashMap<String, f64>> {
        let sym = self.resolve_symbol().await?;
        TechnicalsClient::new()
            .indicators(sym.to_exchange_symbol(), timeframe, indicators)
            .await
    }

    /// Fetches comprehensive company profile and fundamental statistics.
    pub async fn overview(&self) -> Result<FundamentalOverview> {
        let sym = self.resolve_symbol().await?;
        FundamentalsClient::new().overview(sym.to_exchange_symbol()).await
    }

    /// Fetches valuation multiples (P/E, P/B, P/S, EV/EBITDA).
    pub async fn valuation(&self) -> Result<ValuationMetrics> {
        let o = self.overview().await?;
        Ok(o.valuation)
    }

    /// Fetches key financial profitability and liquidity ratios.
    pub async fn financials(&self) -> Result<FinancialMetrics> {
        let o = self.overview().await?;
        Ok(o.financials)
    }

    /// Fetches recent news stories and wire headlines for the ticker.
    pub async fn news(&self) -> Result<Vec<NewsArticle>> {
        let sym = self.resolve_symbol().await?;
        NewsClient::new().fetch_symbol_news(sym.to_exchange_symbol(), 10).await
    }

    /// Discovers all trading venues and global markets where the ticker trades.
    pub async fn venues(&self) -> Result<Vec<MarketVenue>> {
        let ticker = if let Ok(s) = Symbol::parse_strict(&self.symbol_str) {
            s.ticker
        } else {
            self.symbol_str.clone()
        };
        DiscoveryClient::new().discover_venues_exact(&ticker).await
    }

    /// Fetches published community trade ideas for the ticker.
    pub async fn ideas(&self, page: u32) -> Result<Vec<TradeIdea>> {
        SocialClient::new().fetch_ideas(&self.symbol_str, page, IdeaSort::Popular).await
    }

    /// Fetches community Minds micro-discussions for the ticker.
    pub async fn minds(&self, limit: u32) -> Result<Vec<MindPost>> {
        SocialClient::new().fetch_minds(&self.symbol_str, limit, MindsSort::Recent).await
    }

    /// Computes aggregated crowd sentiment score for the ticker.
    pub async fn sentiment(&self) -> Result<SentimentReport> {
        SocialClient::new().fetch_sentiment(&self.symbol_str).await
    }

    /// Returns a live real-time event stream for this ticker.
    pub async fn stream(&self, timeframe: impl AsRef<str>) -> Result<mpsc::Receiver<StreamEvent>> {
        let sym = self.resolve_symbol().await?;
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        TvStreamer::builder()
            .symbol(sym)
            .timeframe(tf)
            .session(SessionType::Regular)
            .build()?
            .stream_events()
            .await
    }

    /// Returns a live real-time event stream with Extended Trading Hours.
    pub async fn stream_extended(&self, timeframe: impl AsRef<str>) -> Result<mpsc::Receiver<StreamEvent>> {
        let sym = self.resolve_symbol().await?;
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        TvStreamer::builder()
            .symbol(sym)
            .timeframe(tf)
            .session(SessionType::Extended)
            .build()?
            .stream_events()
            .await
    }

    /// Returns an autonomous, resilient self-healing event stream with automatic reconnection and backoff.
    pub fn stream_resilient(&self, timeframe: impl AsRef<str>) -> Result<mpsc::Receiver<StreamEvent>> {
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        Ok(crate::supervisor::stream_resilient(&self.raw_symbol_spec(), tf, SessionType::Regular))
    }

    /// Returns a resilient self-healing event stream with Extended Trading Hours (ETH).
    pub fn stream_resilient_extended(&self, timeframe: impl AsRef<str>) -> Result<mpsc::Receiver<StreamEvent>> {
        let tf = Timeframe::from_str(timeframe.as_ref())?;
        Ok(crate::supervisor::stream_resilient(&self.raw_symbol_spec(), tf, SessionType::Extended))
    }

    /// Fetches the latest real-time Level 1 quote for this ticker during regular market hours.
    pub async fn quote(&self) -> Result<QuoteUpdate> {
        self.quote_internal(SessionType::Regular).await
    }

    /// Fetches the latest real-time Level 1 quote including pre-market and after-hours (Extended Trading Hours).
    pub async fn quote_extended(&self) -> Result<QuoteUpdate> {
        self.quote_internal(SessionType::Extended).await
    }

    /// Streams real-time Level 1 quote updates for this ticker.
    pub async fn stream_quotes(&self) -> Result<mpsc::Receiver<QuoteUpdate>> {
        self.stream_quotes_internal(SessionType::Regular).await
    }

    /// Streams real-time Level 1 quote updates with Extended Trading Hours.
    pub async fn stream_quotes_extended(&self) -> Result<mpsc::Receiver<QuoteUpdate>> {
        self.stream_quotes_internal(SessionType::Extended).await
    }

    fn raw_symbol_spec(&self) -> String {
        Symbol::parse_strict(&self.symbol_str)
            .map(|s| s.to_exchange_symbol())
            .unwrap_or_else(|_| {
                self.default_exchange
                    .as_deref()
                    .map(|d| format!("{d}:{}", self.symbol_str))
                    .unwrap_or_else(|| self.symbol_str.clone())
            })
    }

    async fn quote_internal(&self, session: SessionType) -> Result<QuoteUpdate> {
        let sym = self.resolve_symbol().await?;
        let streamer = TvStreamer::builder()
            .symbol(sym)
            .session(session)
            .build()?;
        streamer.fetch_quote().await
    }

    async fn stream_quotes_internal(&self, session: SessionType) -> Result<mpsc::Receiver<QuoteUpdate>> {
        let sym = self.resolve_symbol().await?;
        let streamer = TvStreamer::builder()
            .symbol(sym)
            .session(session)
            .build()?;
        let mut event_rx = streamer.stream_events().await?;
        let (quote_tx, quote_rx) = mpsc::channel(256);

        tokio::spawn(async move {
            while let Some(evt) = event_rx.recv().await {
                if let StreamEvent::Quote(q) = evt {
                    if quote_tx.send(q).await.is_err() {
                        break;
                    }
                }
            }
        });

        Ok(quote_rx)
    }

    async fn fetch_internal(
        &self,
        tf_str: &str,
        count: u32,
        session: SessionType,
        adj: PriceAdjustment,
    ) -> Result<Vec<OhlcvCandle>> {
        let sym = self.resolve_symbol().await?;
        let tf = Timeframe::from_str(tf_str)?;
        let streamer = TvStreamer::builder()
            .symbol(sym)
            .timeframe(tf)
            .session(session)
            .adjustment(adj)
            .candle_count(count)
            .build()?;

        streamer.fetch_candles().await
    }
}
