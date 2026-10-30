# Phase 016: Community Trader Sentiment, Ideas & Micro-Posts

> **Status**: `BACKLOG`  
> **Milestone**: `v0.16.0`  
> **Target Release**: Q1 2028  
> **Primary Focus**: Qualitative crowd sentiment, technical setup ideas, and community trading strategies.

---

## 1. Objective & Problem Statement
Quantitative models increasingly incorporate social sentiment and retail trader positioning as contrarian or momentum signals.
Upstream Python `tradingview-scraper` (`ideas.py` and `minds.py`) extracts published trader ideas, strategy charts, bullish/bearish indicators, and social micro-posts.

Phase 016 implements social intelligence ingestion in Rust, parsing community sentiment, trade setups (entry, stop loss, take profit targets), and idea popularity metrics.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Idea Domain Models | `src/social/ideas.rs` | Struct `TradeIdea` (author, title, sentiment: Bullish/Bearish, target price, timeframe, boosts count, image URL). |
| Minds Post Model | `src/social/minds.rs` | Struct `MindsPost` (user, content, timestamp, symbol mentions, engagement). |
| Social Scraper Client | `src/social/client.rs` | HTTP scraper extracting ideas by symbol sorted by `popular` or `recent`. |
| Sentiment Aggregator | `src/social/sentiment.rs` | Computes bullish vs bearish ratio over recent community publications. |
| Social Test Suite | `tests/social_e2e.rs` | Verification of idea parsing and sentiment computation. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Retrieve community trading ideas for a given symbol with pagination (`start_page`, `end_page`).
- [ ] **AC-02**: Extract directional sentiment bias (Bullish, Bearish, or Neutral) and target levels when specified.
- [ ] **AC-03**: Compute crowd sentiment score based on recent idea distribution and engagement.
- [ ] **AC-04**: Hermetic test validating idea structure and social post deserialization.

---

## 4. Verification Command
```bash
cargo test --test social_e2e
```
