# Phase 015: Real-Time Market News & Editorial Intelligence Feed

> **Status**: `COMPLETED`  
> **Milestone**: `v0.15.0`  
> **Target Release**: Q4 2027  
> **Primary Focus**: Structured symbol-specific and macroeconomic news ingestion from institutional news wires.

---

## 1. Objective & Problem Statement
Market volatility is heavily driven by breaking headlines, institutional press releases, and earnings announcements.
Upstream Python `tradingview-scraper` (`news.py`) scrapes published news articles, story breadcrumbs, publication timestamps, and related symbol associations.

Phase 015 implements news ingestion in Rust, providing strongly typed headline and article streams from TradingView's institutional news network (Reuters, Dow Jones, CoinDesk, etc.).

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Implemented Capabilities |
|:---|:---|:---|
| News Models | `src/news/types.rs` | Struct `NewsArticle` (`id`, `title`, `provider`, `source`, `published`, `urgency`, `link`, `related_symbols`). |
| News Client | `src/news/client.rs` | HTTP client fetching latest news headlines by symbol from `news-headlines.tradingview.com`. |
| News Facade | `src/news/mod.rs` & `src/ticker.rs` | 1-liners: `tradingview_streamer::news("AAPL")`, `Ticker::new("AAPL").news().await?`. |
| News Test Suite | `tests/e2e_news.rs` | Live verification of news headlines and symbol tagging. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: Retrieve news headlines for specific symbols or global market categories.
- [x] **AC-02**: Support filtering by story limit and language.
- [x] **AC-03**: Clean extraction of published timestamp, title, provider, source, and referenced symbols.
- [x] **AC-04**: Hermetic and live test validating article metadata parsing.

---

## 4. Verification Command
```bash
cargo test --test e2e_news
```
