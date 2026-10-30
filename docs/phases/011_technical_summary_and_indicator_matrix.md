# Phase 011: Technical Summary Ratings & Indicator Matrix

> **Status**: `PLANNED`  
> **Milestone**: `v0.11.0`  
> **Target Release**: Q2 2027  
> **Primary Focus**: Server-side technical ratings (`Recommend.All`, `Recommend.MA`, `Recommend.Oscillators`) and multi-timeframe indicator snapshots.

---

## 1. Objective & Problem Statement
TradingView continuously evaluates and updates comprehensive technical analysis ratings for every asset across multiple timeframes (`1m`, `5m`, `15m`, `1h`, `4h`, `1d`, `1W`, `1M`).
Instead of calculating complex technical indicator algorithms locally on client devices, applications can fetch server-computed technical evaluations (`Strong Buy`, `Buy`, `Neutral`, `Sell`, `Strong Sell`) and exact indicator values (RSI, Stochastic, MACD, Ichimoku, EMA/SMA ribbons) in a single HTTP request.

Phase 011 implements this capability in Rust, providing instant technical rating analysis and multi-timeframe comparison matrices.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Technical Ratings Model | `src/technicals/rating.rs` | Strongly typed enum `TechnicalRating` (`StrongBuy`, `Buy`, `Neutral`, `Sell`, `StrongSell`) with numerical score mapping (-1.0 to 1.0). |
| Indicator Snapshot Model | `src/technicals/matrix.rs` | Struct containing Oscillators (RSI, Stoch, CCI, ADX, MACD) and Moving Averages (EMA 10..200, SMA 10..200, Ichimoku, VWMA). |
| Technical Client | `src/technicals/client.rs` | HTTP client querying `https://scanner.tradingview.com/symbol?symbol={exchange}:{symbol}&fields={fields}` with automated timeframe formatting (`|1`, `|5`, `|60`, `|1D`). |
| Multi-Timeframe Matrix | `src/technicals/multi_tf.rs` | Unified struct aggregating technical ratings across multiple timeframes simultaneously for consensus analysis. |
| Technicals Test Suite | `tests/technicals_e2e.rs` | Verification of rating parsing and multi-timeframe indicator deserialization. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Retrieve `Recommend.All`, `Recommend.MA`, and `Recommend.Other` ratings mapped to typed enums with underlying consensus counts (e.g. 16 Buy, 8 Neutral, 2 Sell).
- [ ] **AC-02**: Support querying all 80+ native TradingView indicator fields without manual calculation.
- [ ] **AC-03**: Support single-call multi-timeframe evaluation matrix (`1m`, `5m`, `15m`, `1h`, `4h`, `1d`).
- [ ] **AC-04**: 1-line developer interface: `Ticker::new("AAPL").technical_rating("1d").await?`.
- [ ] **AC-05**: Hermetic test validating JSON parsing and timeframe parameter encoding.

---

## 4. Verification Command
```bash
cargo test --test technicals_e2e
```
