# Phase 008: Real-Time Quote Session & Level 1 Ticker Tape Streamer

> **Status**: `PLANNED`  
> **Milestone**: `v0.8.0`  
> **Target Release**: Q1 2027  
> **Primary Focus**: Quote Session (`qs_...`) protocol implementation for low-latency Level 1 bid/ask, change, and trade updates without chart session overhead.

---

## 1. Objective & Problem Statement
In upstream Python `tradingview-scraper` (`price.py`), quotes and price summaries are gathered using a quote session (`qs_...`) with commands `quote_create_session`, `quote_set_fields`, `quote_add_symbols`, and `quote_fast_symbols`.
Currently, `tradingview-streamer-rs` only implements chart sessions (`cs_...`) which fetch OHLCV candle bars.
While chart sessions are ideal for historical and real-time candlestick charts, high-frequency ticker tapes, watchlists, and portfolio dashboards require lightweight, sub-millisecond Level 1 quote streaming (`lp` last price, `ch` change, `chp` change %, `rtc` real-time change, `bid`, `ask`, and `volume`).

Phase 008 introduces first-class Quote Session management in Rust, allowing concurrent subscription to hundreds of symbols on a single WebSocket connection.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Quote Domain Models | `src/quote/types.rs` | Strongly typed structs for `QuoteSnapshot`, `QuoteUpdate`, `QuoteField` (`lp`, `ch`, `chp`, `rtc`, `rchp`, `bid`, `ask`). |
| Quote Session Handler | `src/quote/session.rs` | Protocol state machine for `quote_create_session`, `quote_set_fields`, `quote_add_symbols`, `quote_remove_symbols`. |
| Quote Stream Demux | `src/protocol/parser.rs` | Deserialization of `qsd` (quote session data) frames into `StreamEvent::Quote(QuoteUpdate)`. |
| Ticker Tape Facade | `src/ticker.rs` & `src/streamer.rs` | Ergonomic builder methods: `stream_quotes(symbols)` returning a `Stream<Item = Result<QuoteUpdate, TvStreamerError>>`. |
| Hermetic Quote Tests | `tests/e2e_quote_streamer.rs` | In-process test validating quote subscription lifecycle and field deserialization. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Support spawning dedicated Quote Sessions (`quote_create_session`) independently or alongside Chart Sessions on a shared WebSocket.
- [ ] **AC-02**: Configurable field masks (`quote_set_fields`) allowing clients to request minimal payloads (e.g. `lp`, `chp` only) or comprehensive quotes (bid, ask, spread, 52-week ranges).
- [ ] **AC-03**: Support batch symbol subscription (`quote_add_symbols`, `quote_fast_symbols`) with automated resubscription on reconnect.
- [ ] **AC-04**: Decode `qsd` protocol frames with sub-microsecond latency, handling partial field deltas gracefully.
- [ ] **AC-05**: Hermetic integration test verifying quote updates for multiple symbols simultaneously.

---

## 4. Verification Command
```bash
cargo test --test e2e_quote_streamer
```
