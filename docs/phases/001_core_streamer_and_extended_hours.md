# Phase 001: Core WebSocket Engine & Extended Trading Hours (ETH)

> **Status**: `COMPLETED`  
> **Milestone**: `v0.1.0`  
> **Target Release**: Baseline Crate  
> **Primary Focus**: Upstream parity with `mnwato/tradingview-scraper#63` resolution.

---

## 1. Objective & Problem Statement
In existing open-source scrapers (notably Python's `mnwato/tradingview-scraper`), the WebSocket client defaults to regular market hours because the `"session"` key is omitted during symbol resolution. This drops crucial volatility and price discovery during US pre-market (04:00 - 09:30 EST) and post-market (16:00 - 20:00 EST) sessions.

Phase 001 delivers a pure async Rust foundation that implements the TradingView wire protocol (`~m~<len>~m~`), automated keep-alive heartbeat echoing (`~h~<seq>`), and first-class `SessionType::Extended` support.

---

## 2. Deliverables & Implemented Modules

| Module | File Location | Responsibility |
|:---|:---|:---|
| Protocol Framing | `src/protocol/framing.rs` | Zero-copy byte scanner for `~m~<len>~m~` and heartbeat ping/pong formatters. |
| Message Builders | `src/protocol/messages.rs` | Wire command encoders with explicit `"session": "extended"` JSON serialization. |
| Inbound Parser | `src/protocol/parser.rs` | Deserialization of `timescale_update`, `du`, and `qsd` packets into typed domain models. |
| Low-Level Client | `src/client.rs` | Async Tokio WebSocket transport with embedded Mozilla TLS root certificates (`webpki-roots`). |
| High-Level Facade | `src/streamer.rs` | Ergonomic `TvStreamerBuilder` and channel-based streaming interfaces. |
| In-Process Mock E2E | `tests/e2e_hermetic_streamer.rs` | Hermetic WebSocket test validating wire frame generation and extended session parameters. |
| Multi-Ticker Matrix | `tests/multi_ticker_timeframe_e2e.rs` | Real-world validation across Equities, Crypto, and Forex across multiple timeframes. |

---

## 3. Acceptance Criteria & Empirical Verification

- [x] **AC-01**: Framing engine correctly parses single and multi-packet concatenated WebSocket frames.
- [x] **AC-02**: Heartbeat keep-alive (`~h~<num>`) is answered in-band without dropping connection.
- [x] **AC-03**: Setting `SessionType::Extended` explicitly sends `"session":"extended"` in `resolve_symbol`.
- [x] **AC-04**: Hermetic E2E test runs in parallel without external internet dependencies.
- [x] **AC-05**: Live multi-ticker test successfully fetches real-time bars from `data.tradingview.com` for `NASDAQ:AAPL`, `BINANCE:BTCUSDT`, and `FX_IDC:EURUSD`.
- [x] **AC-06**: Strict line limits (`< 350 lines` per file) and Zero-Mock compliance verified.

---

## 4. Verification Command
```bash
cargo test --test e2e_hermetic_streamer
cargo test --test multi_ticker_timeframe_e2e -- --nocapture
```
