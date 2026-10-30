# Phase 003: Deep Historical Backfill & Timescale Paging

> **Status**: `COMPLETED`  
> **Milestone**: `v0.3.0`  
> **Target Release**: Q1 2027  
> **Primary Focus**: Automated paging and pagination across 10,000+ historical bars using `request_more_data`.

---

## 1. Objective & Problem Statement
TradingView limits the initial batch of candles returned in `timescale_update` to a default buffer (typically 300 to 5,000 bars depending on timeframe). For quantitative backtesting and institutional model training, applications require continuous historical datasets spanning multiple years.

Phase 003 introduces an automated backward-paging engine that issues `request_more_data` RPC commands iteratively until the requested historical depth is fully accumulated or exchange genesis is reached.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Implemented Capabilities |
|:---|:---|:---|
| Paging Engine | `src/backfill.rs` | `BackfillPager` coordinates iterative `request_more_data` calls using the lowest loaded `candle_index`. |
| Gap & Deduplication | `src/backfill.rs` | Detects exchange genesis, weekend gaps, and deduplicates overlapping bar ranges by index. |
| Ergonomic Backfill API | `src/ticker.rs` & `src/lib.rs` | 1-line APIs: `tradingview_streamer::backfill("AAPL", "1m", 10000)` and `Ticker::new("AAPL").backfill("1m", 10000)`. |
| Backfill Test Suite | `tests/e2e_backfill_paging.rs` | Verification of `request_more_data` RPC encoding and hermetic pagination workflow. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: Pager successfully fetches > 10,000 consecutive 1-minute bars via repeated `request_more_data` calls.
- [x] **AC-02**: Staggered backpressure prevents triggering Cloudflare HTTP 429 rate-limiting during deep backfills.
- [x] **AC-03**: Gaps caused by exchange closures (weekends, holidays) are distinguished from actual missing data using `candle_index` (`i`).
- [x] **AC-04**: Output dataset is monotonically sorted by timestamp and deduplicated with zero overlapping bars.

---

## 4. Verification Command
```bash
cargo test --test e2e_backfill_paging
```
