# 04 Solution Strategy

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `1a57c8d` (branch: `main`)
> - **Verified Timestamp**: `2026-10-31T08:44:44+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 4.1 Solution Strategy Overview
To deliver sub-millisecond market data ingestion while guaranteeing full Extended Trading Hours (ETH) fidelity and parity with upstream scrapers, the architecture adopts several key strategies:

1. **Protocol Framing Engine (`src/protocol/framing.rs`)**:
   Implements a zero-copy framing scanner that parses `~m~<length>~m~<payload>` byte chunks directly, seamlessly extracting multi-message sequences packed into single WebSocket network packets.

2. **Explicit Session Parameter Injection (Issue #63 Resolution)**:
   In `src/protocol/messages.rs`, the `resolve_symbol` command serializes an inner JSON structure containing `"session": "extended"`. This instructs TradingView's calculation engine to include pre-market and after-hours bars, eliminating the limitation observed in `mnwato/tradingview-scraper`.

3. **Dual-Engine Market Architecture**:
   - **Real-Time WebSocket Engine**: Chart Session (`cs_...`) for streaming OHLCV candlestick series and technical studies, plus Quote Session (`qs_...`) for high-frequency ticker tapes (bid, ask, real-time changes).
   - **High-Throughput REST Scanner Engine**: Type-safe query client targeting `scanner.tradingview.com` across 19 global asset markets with automated connection reuse.

4. **Autonomous Heartbeat Keep-Alive Loop**:
   Heartbeat markers (`~h~<seq>`) are detected immediately during framing and answered in-band with matching sequence frames, preventing connection termination during quiet market periods.

5. **Layered Separation of Concerns (SOLID & SRP)**:
   - Low-level network I/O (`client.rs`) is decoupled from high-level user coordination (`streamer.rs`, `ticker.rs`).
   - Domain models (`types.rs`) remain free of network dependencies.
   - Session identifiers (`session.rs`) are generated hermetically without global static state.

6. **yfinance-Grade Ergonomic Facade Pattern**:
   Provides 1-line instant helpers (`tradingview_streamer::download("AAPL")`) and intuitive object models (`Ticker::new("AAPL")`) with smart exchange inference, eliminating boilerplate for downstream consumers.

7. **Graduated Milestone Phase Execution**:
   Engineering roadmap execution follows numbered modular specifications (`docs/phases/001_` through `docs/phases/016_`), ensuring every architectural capability is delivered with hermetic and empirical test suites before release.
