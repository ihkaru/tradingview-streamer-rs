# Phase 009: Financial Scanner & Multi-Market Screener Engine

> **Status**: `PLANNED`  
> **Milestone**: `v0.9.0`  
> **Target Release**: Q1 2027  
> **Primary Focus**: Type-safe REST client for TradingView's Scanner API (`scanner.tradingview.com/{market}/scan`) with rich filter expressions and multi-asset support.

---

## 1. Objective & Problem Statement
TradingView maintains an enterprise-grade stock, crypto, forex, futures, and bond scanner engine capable of filtering tens of thousands of global assets in real time.
Upstream Python `tradingview-scraper` (`screener.py`) provides a basic wrapper around this API, but lacks compile-time filter safety, zero-copy deserialization, and async streaming pagination.

Phase 009 implements an idiomatic Rust client and type-safe query builder for the TradingView Scanner API, enabling quantitative traders and algorithms to run sophisticated screeners (e.g. "Find all US stocks with RSI < 30, Market Cap > $10B, and Volume > 2x 30-day average").

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Screener Filter DSL | `src/scanner/filter.rs` | Type-safe AST builder: `Filter::new("close").greater(100.0)`, `Filter::new("RSI").less(30.0)`, `crosses_above`, `in_range`. |
| Scanner Markets | `src/scanner/market.rs` | Strongly typed enum for 19 markets (`America`, `Crypto`, `Forex`, `Germany`, `UK`, `Futures`, `Bonds`, `Global`). |
| Scanner HTTP Client | `src/scanner/client.rs` | Async REST client using `reqwest` or custom HTTP transport with keep-alive connection pooling. |
| Result Paginator | `src/scanner/paginator.rs` | Async stream automatically requesting subsequent chunks (`range: [0, 50]`, `[50, 100]`) on demand. |
| Screener Test Suite | `tests/scanner_e2e.rs` | Integration tests verifying screener queries, sorting, and filter parsing against live scanner endpoints. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Strongly typed filter operations supporting `greater`, `less`, `equal`, `in_range`, `crosses`, `crosses_above`, and `crosses_below`.
- [ ] **AC-02**: Support all 19 TradingView market scanning endpoints (`america`, `crypto`, `forex`, `bonds`, `cfd`, `global`, etc.).
- [ ] **AC-03**: Compile-time safety for column selections (preventing invalid column requests) while supporting raw dynamic field queries.
- [ ] **AC-04**: Async pagination abstraction providing `futures::Stream<Item = Result<ScreenedSymbol, TvStreamerError>>`.
- [ ] **AC-05**: Zero production mocks with robust error handling for HTTP rate limits (429) or schema drifts.

---

## 4. Verification Command
```bash
cargo test --test scanner_e2e
```
