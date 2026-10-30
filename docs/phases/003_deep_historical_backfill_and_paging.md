# Phase 003: Deep Historical Backfill & Timescale Paging

> **Status**: `PLANNED`  
> **Milestone**: `v0.3.0`  
> **Target Release**: Q1 2027  
> **Primary Focus**: Automated paging and pagination across 10,000+ historical bars using `request_more_data`.

---

## 1. Objective & Problem Statement
TradingView limits the initial batch of candles returned in `timescale_update` to a default buffer (typically 300 to 5,000 bars depending on timeframe). For quantitative backtesting and institutional model training, applications require continuous historical datasets spanning multiple years.

Phase 003 introduces an automated backward-paging engine that issues `request_more_data` RPC commands iteratively until the requested historical depth is fully accumulated or exchange genesis is reached.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Paging Engine | `src/backfill/pager.rs` | Coordinates iterative `request_more_data` calls using the lowest loaded `candle_index`. |
| Gap & Deduplication | `src/backfill/dedup.rs` | Detects exchange holidays, weekend gaps, and deduplicates overlapping bar ranges. |
| Progress Telemetry | `src/backfill/telemetry.rs` | Yields progress percentage callbacks (`bars_loaded`, `oldest_timestamp`, `is_genesis`). |
| Batch Backfill API | `src/streamer.rs` | Streamer API method: `streamer.backfill_candles(start_epoch, end_epoch).await`. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Pager successfully fetches > 10,000 consecutive 1-minute bars via repeated `request_more_data` calls.
- [ ] **AC-02**: Staggered backpressure prevents triggering Cloudflare HTTP 429 rate-limiting during deep backfills.
- [ ] **AC-03**: Gaps caused by exchange closures (weekends, holidays) are distinguished from actual missing data using `candle_index` (`i`).
- [ ] **AC-04**: Output dataset is monotonically sorted by timestamp and deduplicated with zero overlapping bars.

---

## 4. Verification Command
```bash
cargo test --test e2e_backfill_paging
```
