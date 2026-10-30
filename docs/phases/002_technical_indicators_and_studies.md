# Phase 002: Technical Indicators & Built-in Studies Ingestion

> **Status**: `PLANNED`  
> **Milestone**: `v0.2.0`  
> **Target Release**: Q4 2026  
> **Primary Focus**: Automated study creation and multi-indicator streaming alongside OHLCV candles.

---

## 1. Objective & Problem Statement
While raw OHLCV candles provide core price action, algorithmic trading systems require real-time technical indicators computed server-side by TradingView (such as Volume Profile, RSI, MACD, and Bollinger Bands) to eliminate redundant client-side calculations and avoid indicator lag.

Phase 002 implements declarative indicator study subscriptions via the `create_study` WebSocket command, parsing incoming `du` (data update) events into strongly typed indicator point streams.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Indicator Types | `src/indicators/types.rs` | Strongly typed definitions for standard indicators (RSI, MACD, Bollinger, EMA/SMA). |
| Study Builder | `src/indicators/builder.rs` | Protocol builder for `create_study` (`[chart_session, study_id, "st1", series_id, study_type, inputs]`). |
| Study Stream Demux | `src/protocol/parser.rs` | Extended deserializer extracting multi-study values from `du` packets into `StreamEvent::Study`. |
| Indicator Facade | `src/streamer.rs` | Fluent builder methods: `.with_indicator(Indicator::RSI(14))` and `.with_study(study)`. |
| Hermetic Study Tests | `tests/e2e_indicators.rs` | In-process test validating study creation, parameter serialization, and indicator updates. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Support subscribing to standard built-in indicators (`Volume@tv-basicstudies-246`, `RSI@tv-basicstudies-246`, `MACD@tv-basicstudies-246`).
- [ ] **AC-02**: Multiple simultaneous studies can be attached to a single chart session without study ID collision.
- [ ] **AC-03**: Inbound `du` events are parsed into typed floating-point indicator series indexed by candle timestamp.
- [ ] **AC-04**: Hermetic test demonstrates parallel candle and indicator data arrival within a single event loop.

---

## 4. Verification Command
```bash
cargo test --test e2e_indicators
```
