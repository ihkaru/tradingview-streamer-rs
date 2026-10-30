# 04 Solution Strategy

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `bf0aeee` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 4.1 Solution Strategy Overview
To deliver sub-millisecond market data ingestion while guaranteeing full Extended Trading Hours (ETH) fidelity, the architecture adopts several key strategies:

1. **Protocol Framing Engine (`src/protocol/framing.rs`)**:
   Implements a zero-copy framing scanner that parses `~m~<length>~m~<payload>` byte chunks directly, seamlessly extracting multi-message sequences packed into single WebSocket network packets.

2. **Explicit Session Parameter Injection (Issue #63 Resolution)**:
   In `src/protocol/messages.rs`, the `resolve_symbol` command serializes an inner JSON structure containing `"session": "extended"`. This instructs TradingView's calculation engine to include pre-market and after-hours bars, eliminating the limitation observed in `mnwato/tradingview-scraper`.

3. **Autonomous Heartbeat Keep-Alive Loop**:
   Heartbeat markers (`~h~<seq>`) are detected immediately during framing and answered in-band with matching sequence frames, preventing connection termination during quiet market periods.

4. **Layered Separation of Concerns (SOLID & SRP)**:
   - Low-level network I/O (`client.rs`) is decoupled from high-level user coordination (`streamer.rs`).
   - Domain models (`types.rs`) remain free of network dependencies.
   - Session identifiers (`session.rs`) are generated hermetically without global static state.
