# Phase 010: Market Movers & Extended Hours Leaders (ETH)

> **Status**: `COMPLETED`  
> **Milestone**: `v0.10.0`  
> **Target Release**: Q2 2027  
> **Primary Focus**: Out-of-the-box market movers rankings with specialized focus on Pre-Market and After-Hours volume/price movers.

---

## 1. Objective & Problem Statement
Active traders require immediate discovery of liquidity shifts and volatile instruments before market open and after market close.
Upstream Python `tradingview-scraper` (`market_movers.py`) wraps predefined scanner queries for `pre-market-gainers`, `pre-market-losers`, `after-hours-gainers`, `after-hours-losers`, `most-active`, and `penny-stocks`.

Phase 010 brings these capabilities natively into Rust, integrating seamlessly with our core Extended Trading Hours (ETH) engine to provide 1-line access to pre-market and post-market market leaders.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Implemented Capabilities |
|:---|:---|:---|
| Mover Categories | `src/movers/types.rs` | Strongly typed enum `MoverCategory` (`Gainers`, `Losers`, `MostActive`, `PennyStocks`, `PreMarketGainers`, `PreMarketLosers`, `AfterHoursGainers`, `AfterHoursLosers`). |
| Market Movers Client | `src/movers/client.rs` | Query dispatcher and record mapper translating scanner queries into typed `MoverRecord`s. |
| Mover Record Model | `src/movers/types.rs` | Deserialized instrument record containing symbol, price, change, volume, pre/post-market metrics. |
| Mover Facade | `src/movers/mod.rs` & `src/lib.rs` | 1-line helpers: `tradingview_streamer::movers::pre_market_gainers(20).await?`. |
| Movers Test Suite | `tests/e2e_movers.rs` | Live verification of pre-market, top gainers, and most active movers retrieval. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: Support all major mover categories: Top Gainers, Top Losers, Most Active, Pre-Market Gainers, Pre-Market Losers, After-Hours Gainers, and After-Hours Losers.
- [x] **AC-02**: Support filtering by minimum price or minimum liquidity threshold to avoid illiquid micro-caps.
- [x] **AC-03**: Sub-100ms deserialization and structured response mapping without intermediate disk I/O.
- [x] **AC-04**: Hermetic and live test suite verifying JSON payload mapping and error recovery.

---

## 4. Verification Command
```bash
cargo test --test e2e_movers
```
