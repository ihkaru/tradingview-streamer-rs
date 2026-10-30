# Phase 005: Zero-Copy Columnar Ingestion & Analytical Export

> **Status**: `COMPLETED`  
> **Milestone**: `v0.5.0`  
> **Target Release**: Q3 2027  
> **Primary Focus**: Addressing `DEBT-01` via cache-friendly Structure-of-Arrays (SoA) columnar data structures and zero-allocation analytical exports.

---

## 1. Objective & Problem Statement
Currently, inbound TradingView packets are parsed into generic `serde_json::Value` trees before being mapped to [`OhlcvCandle`]. This introduces allocation overhead that, while acceptable for normal streaming, becomes a bottleneck during deep backfills (100,000+ candles) or microsecond-sensitive algorithmic pipelines.

Phase 005 resolves architectural debt **`DEBT-01`** by implementing a cache-aligned columnar Structure-of-Arrays (`ColumnarCandles`) format directly transposable into Apache Arrow buffers, Polars DataFrames, and vectorized CSV outputs.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Columnar Structure | `src/export/columnar.rs` | High-performance Structure-of-Arrays (SoA) layout (`ColumnarCandles`). |
| Export Facade | `src/export/mod.rs` | Unified export module re-exporting columnar structures. |
| CSV / Buffer Export | `src/export/columnar.rs` | Vectorized `.to_csv()` and Arrow-compatible columnar slices. |
| Export Test Suite | `tests/test_columnar_export.rs` | Verification of row-to-columnar transformation and live download export. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: High-throughput row-to-columnar transposition without redundant heap copies.
- [x] **AC-02**: Architectural debt `DEBT-01` addressed and columnar buffers provided.
- [x] **AC-03**: Export collection directly into vectorized CSV formatted output for external analytical tools.
- [x] **AC-04**: Hermetic and live verification of columnar conversion and field integrity.

---

## 4. Verification Command
```bash
cargo test --test test_columnar_export
```
