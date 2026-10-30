# 01 Introduction and Goals

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `e19a65f` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 1.1 Requirements Overview
`tradingview-streamer` is a pure async Rust library engineered to interface with TradingView's private real-time WebSocket protocol (`wss://data.tradingview.com/socket.io/websocket`).
The library provides:
1. First-class resolution for **Issue #63** (`mnwato/tradingview-scraper`): enabling Extended Trading Hours (ETH) to ingest pre-market (04:00 - 09:30 EST) and post-market (16:00 - 20:00 EST) OHLCV candles without omission.
2. Low-overhead real-time message demultiplexing, heartbeat keep-alive automation (`~m~<len>~m~~h~<seq>`), and resilient stream consumption.
3. Strongly typed domain primitives ([`Symbol`], [`Timeframe`], [`SessionType`], [`OhlcvCandle`]) designed for seamless embedding into quantitative finance engines, algorithmic execution loops, and data ingestion pipelines.

## 1.2 Quality Goals

| Priority | Quality Goal Tag | Goal Description |
|:---:|:---|:---|
| 1 | `Q42-PERF` | Sub-millisecond packet framing and parsing with zero heap allocation in fast-path message routing. |
| 2 | `Q42-RELIAB` | Continuous connection stability via proactive heartbeat pong handling and graceful reconnection handling. |
| 3 | `Q42-INTEROP` | Flawless protocol compatibility with TradingView wire standards, explicitly fixing session parameters (Issue #63). |

## 1.3 Stakeholders

| Role | Expectation |
|:---|:---|
| Quantitative Researchers | Accurate pre-market and after-hours OHLCV bars without timestamp skew or missing volatility gaps. |
| Systems Engineers | Low memory footprint (<10MB RAM), async concurrency with Tokio, and hermetic parallel testing. |
| Downstream Crate Consumers | Simple, builder-based facade requiring minimal boilerplate to stream candles. |
