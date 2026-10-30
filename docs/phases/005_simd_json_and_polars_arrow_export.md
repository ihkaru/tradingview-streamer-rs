# Phase 005: Zero-Copy SIMD Deserialization & Polars/Arrow Integration

> **Status**: `BACKLOG`  
> **Milestone**: `v0.5.0`  
> **Target Release**: Q3 2027  
> **Primary Focus**: Addressing `DEBT-01` via SIMD-accelerated JSON parsing and direct export to Apache Arrow / Polars DataFrames.

---

## 1. Objective & Problem Statement
Currently, inbound TradingView packets are parsed into generic `serde_json::Value` trees before being mapped to [`OhlcvCandle`]. This introduces allocation overhead that, while acceptable for normal streaming, becomes a bottleneck during deep backfills (100,000+ candles) or microsecond-sensitive algorithmic pipelines.

Phase 005 resolves architectural debt **`DEBT-01`** by implementing a zero-copy SIMD JSON parser and adding optional feature flags (`features = ["polars", "arrow"]`) for direct zero-copy columnar ingestion.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Zero-Copy Parser | `src/protocol/simd_parser.rs` | Direct slice parsing into contiguous memory buffers without intermediate JSON AST allocation. |
| Arrow Columnar Buffer | `src/export/arrow.rs` | Builds Apache Arrow RecordBatches directly from incoming timescale arrays. |
| Polars DataFrame Adapter | `src/export/polars.rs` | Provides `.to_polars_df()` method for instant analytical manipulation. |
| Feature Flags | `Cargo.toml` | Optional `polars = { version = "0.45", optional = true }` and `arrow` features. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Ingestion throughput reaches > 500,000 candles per second in zero-copy benchmark.
- [ ] **AC-02**: Architectural debt `DEBT-01` closed and verified in Section 11 of arc42 documentation.
- [ ] **AC-03**: Enabling feature `polars` allows converting a candle collection directly to a Polars `DataFrame`.
- [ ] **AC-04**: 100% memory-safe execution verified with Miri and ASan.

---

## 4. Verification Command
```bash
cargo test --features polars,arrow --test test_arrow_export
cargo bench --bench parser_benchmark
```
