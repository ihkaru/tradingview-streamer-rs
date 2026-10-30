# Phase 015: Real-Time Market News & Editorial Intelligence Feed

> **Status**: `BACKLOG`  
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

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| News Models | `src/news/types.rs` | Structs `NewsHeadline`, `NewsArticle`, `NewsProvider`, `NewsUrgency`. |
| News Client | `src/news/client.rs` | HTTP client fetching latest news by symbol or market sector. |
| News Article Parser | `src/news/parser.rs` | HTML/JSON extractor extracting clean body text, tags, and associated symbols. |
| Ticker Integration | `src/ticker.rs` | `Ticker::new("AAPL").news(limit).await?` method returning recent headlines. |
| News Test Suite | `tests/news_e2e.rs` | Hermetic and live verification of news feeds. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Retrieve news headlines for specific symbols or global market categories.
- [ ] **AC-02**: Support filtering by news provider, urgency, and language.
- [ ] **AC-03**: Clean extraction of published timestamp, title, summary, source, and referenced symbols.
- [ ] **AC-04**: Hermetic test validating article metadata parsing.

---

## 4. Verification Command
```bash
cargo test --test news_e2e
```
