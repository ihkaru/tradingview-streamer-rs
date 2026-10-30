# Phase 014: Corporate Calendars (Earnings, Dividends & Macro Events)

> **Status**: `COMPLETED`  
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

| Planned Module | Target File | Implemented Capabilities |
|:---|:---|:---|
| Dividend & Earnings Models | `src/calendar/types.rs` | Strongly typed representations for `DividendEvent` and `EarningsEvent`. |
| Calendar Client | `src/calendar/client.rs` | REST client querying `calendar-dividends` and `calendar-earnings` endpoints. |
| Calendar Facade | `src/calendar/mod.rs` & `src/lib.rs` | 1-liners: `tradingview_streamer::calendar::dividends(50)`, `earnings(50)`. |
| Calendar Test Suite | `tests/e2e_calendar.rs` | Verification of dividend and earnings calendar ingestion. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: Retrieve upcoming and historical earnings announcements with estimated vs reported EPS.
- [x] **AC-02**: Retrieve dividend calendars with ex-date and annualized yield calculations.
- [x] **AC-03**: Support market filtering (`america`, global markets).
- [x] **AC-04**: Hermetic and live test validating date parsing and event deserialization.

---

## 4. Verification Command
```bash
cargo test --test e2e_calendar
```
