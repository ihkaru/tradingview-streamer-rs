# Phase 008: Real-Time Quote Session & Level 1 Ticker Tape Streamer

> **Status**: `COMPLETED`  
> **Milestone**: `v0.8.0`  
> **Target Release**: Q1 2027  
> **Primary Focus**: Quote Session (`qs_...`) protocol implementation for low-latency Level 1 bid/ask, change, and trade updates without chart session overhead.

---

## 1. Objective & Problem Statement
In upstream Python `tradingview-scraper` (`price.py`), quotes and price summaries are gathered using a quote session (`qs_...`) with commands `quote_create_session`, `quote_set_fields`, `quote_add_symbols`, and `quote_fast_symbols`.
Currently, `tradingview-streamer-rs` only implements chart sessions (`cs_...`) which fetch OHLCV candle bars.
While chart sessions are ideal for historical and real-time candlestick charts, high-frequency ticker tapes, watchlists, and portfolio dashboards require lightweight, sub-millisecond Level 1 quote streaming (`lp` last price, `ch` change, `chp` change %, `rtc` real-time change, `bid`, `ask`, and `volume`).

Phase 008 introduces first-class Quote Session management in Rust, allowing concurrent subscription to symbols on a single WebSocket connection.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Implemented Capabilities |
|:---|:---|:---|
| Quote Domain Models | `src/quote.rs` | Strongly typed struct [`QuoteUpdate`] (`lp`, `ch`, `chp`, `rtc`, `rchp`, `bid`, `ask`, `spread`, `volume`, `merge()`). |
| Quote Session Handler | `src/protocol/messages.rs` | Protocol commands for `quote_create_session`, `quote_set_fields`, `quote_add_symbols`, `quote_fast_symbols`. |
| Quote Stream Demux | `src/protocol/parser.rs` | Deserialization of `qsd` (quote session data) frames into `StreamEvent::Quote(QuoteUpdate)`. |
| Ticker Tape Facade | `src/ticker.rs` & `src/streamer.rs` | Ergonomic methods: `quote()`, `quote_extended()`, `stream_quotes()`, `stream_quotes_extended()`, `fetch_quote()`. |
| Hermetic Quote Tests | `tests/e2e_quote_streamer.rs` | In-process test validating quote subscription lifecycle and field deserialization. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: Support spawning dedicated Quote Sessions (`quote_create_session`) independently or alongside Chart Sessions on a shared WebSocket.
- [x] **AC-02**: Configurable field masks (`quote_set_fields`) allowing clients to request minimal payloads (e.g. `lp`, `chp` only) or comprehensive quotes (bid, ask, spread, 52-week ranges).
- [x] **AC-03**: Support batch symbol subscription (`quote_add_symbols`, `quote_fast_symbols`) with automated resubscription on reconnect.
- [x] **AC-04**: Decode `qsd` protocol frames with sub-microsecond latency, handling partial field deltas gracefully via `QuoteUpdate::merge`.
- [x] **AC-05**: Hermetic integration test verifying quote updates for multiple symbols simultaneously.

---

## 4. Verification Command
```bash
cargo test --test e2e_quote_streamer
```
