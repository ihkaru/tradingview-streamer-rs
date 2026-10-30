# 01 Introduction and Goals

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `4e180a7` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 1.1 Requirements Overview
`tradingview-streamer` is a high-performance, pure async Rust ecosystem engineered to interface with TradingView's private real-time WebSocket protocol (`wss://data.tradingview.com/socket.io/websocket`) and financial Scanner REST APIs (`https://scanner.tradingview.com`).
The library provides:
1. **Extended Trading Hours (ETH) Invariant**: Native resolution for **Issue #63** (`mnwato/tradingview-scraper`), streaming pre-market (04:00 - 09:30 EST) and post-market (16:00 - 20:00 EST) OHLCV bars without omission.
2. **Dual-Engine Market Architecture**:
   - **Real-Time Streaming Engine**: Chart Session (`cs_...`) for historical/live OHLCV & indicators, and Quote Session (`qs_...`) for high-frequency Level 1 bid/ask, last price (`lp`), 24h change (`chp`), and real-time trade updates (`rtc`).
   - **Global Scanner & Screener Engine**: REST query engine for 19 global asset markets with rich filter predicates (`greater`, `less`, `in_range`, `crosses`), top gainers/losers, pre/post-market leaders, and multi-timeframe technical indicator ratings (`Recommend.All`).
3. **Institutional-Grade Modularity & Ergonomics**:
   - 1-line yfinance-grade getting started API (`download()`, `download_extended()`, `Ticker::new("AAPL")`).
   - Strongly typed domain models ([`Symbol`], [`Timeframe`], [`SessionType`], [`OhlcvCandle`], `QuoteSnapshot`, `ScreenedSymbol`, `TechnicalRating`).
   - Zero-mock policy, files `< 350 lines`, functions `< 50 lines`.

## 1.2 Quality Goals

| Priority | Quality Goal Tag | Goal Description |
|:---:|:---|:---|
| 1 | `Q42-PERF` | Sub-millisecond packet framing and parsing with zero heap allocation in fast-path message routing. |
| 2 | `Q42-RELIAB` | Continuous connection stability via proactive heartbeat pong handling and graceful reconnection handling. |
| 3 | `Q42-INTEROP` | Flawless protocol compatibility with TradingView wire standards, explicitly fixing session parameters (Issue #63). |
| 4 | `Q42-COMPLETENESS` | Comprehensive functional parity with upstream scrapers across real-time streaming, scanner, movers, technicals, and corporate calendars. |

## 1.3 Stakeholders

| Role | Expectation |
|:---|:---|
| Quantitative Researchers | Accurate pre-market and after-hours OHLCV bars without timestamp skew or missing volatility gaps. |
| High-Frequency Traders | Sub-millisecond quote tape updates (`qs_...`) with minimal memory allocation and zero buffer copy. |
| Screener & Algo Developers | Declarative filtering and ranking across 19 global markets with compile-time type safety. |
| Systems Engineers | Low memory footprint (<10MB RAM), async concurrency with Tokio, and hermetic parallel testing. |
| Downstream Crate Consumers | Simple, builder-based facade requiring minimal boilerplate to stream candles and query screeners. |

## 1.4 Strategic Roadmap Alignment
Long-term feature evolution, indicator ingestion, historical paging, scanner integration, and FFI bindings are governed by the graduated milestone phases defined in [ROADMAP.md](../../ROADMAP.md) and detailed in [docs/phases/](../phases/README.md).
