# Phase 010: Market Movers & Extended Hours Leaders (ETH)

> **Status**: `PLANNED`  
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

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Mover Categories | `src/movers/types.rs` | Strongly typed enum `MoverCategory` (`PreMarketGainers`, `PreMarketLosers`, `AfterHoursGainers`, `AfterHoursLosers`, `MostActive`, `UnusualVolume`, `PennyStocks`). |
| Market Movers Client | `src/movers/client.rs` | Optimized query dispatcher targeting regional endpoints (`stocks-usa`, `stocks-uk`, `crypto`, `forex`). |
| Mover Record Model | `src/movers/record.rs` | Deserialized instrument record containing symbol, price, change abs/percent, volume, pre/post-market volume ratio, and market cap. |
| Mover Facade | `src/lib.rs` & `src/ticker.rs` | 1-line helper: `tradingview_streamer::movers::pre_market_gainers(20).await?`. |
| Movers Test Suite | `tests/movers_e2e.rs` | Hermetic and live verification of pre-market and after-hours movers retrieval. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Support all major mover categories: Top Gainers, Top Losers, Most Active, Pre-Market Gainers, Pre-Market Losers, After-Hours Gainers, and After-Hours Losers.
- [ ] **AC-02**: Support filtering by minimum price or minimum liquidity threshold to avoid illiquid micro-caps.
- [ ] **AC-03**: Sub-100ms deserialization and structured response mapping without intermediate disk I/O.
- [ ] **AC-04**: Hermetic test suite verifying JSON payload mapping and error recovery.

---

## 4. Verification Command
```bash
cargo test --test movers_e2e
```
