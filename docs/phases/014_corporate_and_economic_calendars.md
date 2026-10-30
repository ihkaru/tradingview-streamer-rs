# Phase 014: Corporate Calendars (Earnings, Dividends & Macro Events)

> **Status**: `BACKLOG`  
> **Milestone**: `v0.14.0`  
> **Target Release**: Q4 2027  
> **Primary Focus**: Historical and upcoming earnings reports, dividend distributions, and macroeconomic economic calendars.

---

## 1. Objective & Problem Statement
Event-driven trading strategies depend on timely corporate action calendars (earnings surprises, dividend record dates) and macroeconomic release schedules (FOMC, CPI, unemployment).
Upstream Python `tradingview-scraper` (`cal.py`) extracts dividend and earnings events via specialized scanner queries.

Phase 014 brings structured corporate and economic calendars into Rust, providing typed event timelines with consensus estimates, actual reports, and surprise calculations.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Earnings Event Model | `src/calendar/earnings.rs` | Struct `EarningsEvent` (report date, fiscal quarter, estimated EPS, reported EPS, surprise percentage, market status). |
| Dividend Event Model | `src/calendar/dividends.rs` | Struct `DividendEvent` (ex-dividend date, payment date, record date, amount, yield). |
| Economic Calendar Model | `src/calendar/economic.rs` | Struct `EconomicRelease` (event country, importance/impact rating, previous, forecast, actual). |
| Calendar Client | `src/calendar/client.rs` | Async query engine for upcoming and historical calendar ranges. |
| Calendar Test Suite | `tests/calendar_e2e.rs` | Hermetic verification of calendar parsing and date conversions. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Retrieve upcoming and historical earnings announcements with estimated vs reported EPS.
- [ ] **AC-02**: Retrieve dividend calendars with ex-date and annualized yield calculations.
- [ ] **AC-03**: Support date range filtering (`start_date`, `end_date`) and market filtering (`stocks-usa`, `stocks-uk`, `crypto`).
- [ ] **AC-04**: Hermetic test validating date parsing and event deserialization.

---

## 4. Verification Command
```bash
cargo test --test calendar_e2e
```
