# GEMINI.md - Architecture, Conventions & Verification Standards

## 1. Project Overview & Scope
`tradingview-streamer` is a high-performance, pure async Rust library for ingesting and streaming real-time market data from TradingView WebSockets.
It specifically addresses and resolves upstream limitation **Issue #63** (`mnwato/tradingview-scraper`) by introducing first-class support for **Extended Trading Hours (ETH)**, allowing applications to retrieve pre-market (04:00 - 09:30 EST) and after-hours (16:00 - 20:00 EST) OHLCV candles.

## 2. Architectural Principles & SOLID Guidelines
- **Single Responsibility Principle (SRP)**:
  - Transport layer (`src/client.rs`): Handles WebSocket connection, TLS setup, keep-alive heartbeats, and raw frame transmission.
  - Protocol layer (`src/protocol/`): Pure message serialization, deserialization, and framing (`~m~<length>~m~`).
  - Session layer (`src/session.rs`): Manages unique chart/quote session tokens (`cs_...`, `qs_...`).
  - Domain layer (`src/types.rs`): Strongly typed representations for sessions, timeframes, symbols, and candles.
  - Facade layer (`src/streamer.rs`): Developer-friendly builder and async API.
- **Line & Complexity Limits**:
  - All source files strictly `< 350 lines`.
  - Individual functions strictly `< 50 lines`.
- **Zero Mock Policy**:
  - No mock frameworks or fake stubs in production paths (`src/`).
  - Hermetic tests (`tests/e2e_hermetic_streamer.rs`) run an in-process WebSocket server speaking genuine wire protocol.
  - Live tests (`tests/live_tradingview_e2e.rs`) verify end-to-end connectivity against TradingView's production servers.

## 3. Protocol Wire Behavior & Extended Hours Invariant (Issue #63)
- In the TradingView WebSocket protocol, omitting the `"session"` property inside `resolve_symbol` defaults to `"session": "regular"` (Regular Trading Hours only).
- To stream pre-market and after-hours data, `resolve_symbol` must specify `"session": "extended"`:
  ```json
  {
    "m": "resolve_symbol",
    "p": [
      "cs_xxxxxxxxxxxx",
      "sds_sym_1",
      "={\"symbol\":\"NASDAQ:AAPL\",\"adjustment\":\"splits\",\"session\":\"extended\"}"
    ]
  }
  ```
- Keep-alive heartbeat frames matching `~m~<len>~m~~h~<seq>` must be answered immediately with an echo frame `~m~<len>~m~~h~<seq>`.

## 4. Verification Commands & The Gauntlet
Run the test suite in parallel:
```bash
cargo test --all-targets
```

Check formatting and linting:
```bash
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Build release documentation:
```bash
cargo doc --no-deps
```
