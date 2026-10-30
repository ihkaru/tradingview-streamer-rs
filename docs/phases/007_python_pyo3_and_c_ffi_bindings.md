# Phase 007: C-ABI Foreign Function Interface & Multi-Language Bindings

> **Status**: `COMPLETED`  
> **Milestone**: `v0.7.0`  
> **Target Release**: 2028  
> **Primary Focus**: Delivering high-performance C-ABI foreign function exports (`extern "C"`) enabling universal integration with Python (`ctypes`/`cffi`), C/C++, Go, and Zig.

---

## 1. Objective & Problem Statement
Python and native quantitative engines are widely used in financial data science. Quantitative systems need direct C-ABI foreign function interop to ingest TradingView real-time and historical data (including Extended Hours) with zero garbage collection overhead.

Phase 007 delivers high-performance C-ABI exports (`src/ffi/`) exposing thread-safe functions: `tv_streamer_version`, `tv_streamer_download_sync`, `tv_streamer_quote_sync`, and `tv_streamer_free_string`.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| C-ABI Exports | `src/ffi/mod.rs` | Exported `extern "C"` functions for C/C++, Python (`ctypes`), and Go (`cgo`). |
| C-Compatible Structs | `src/ffi/types.rs` | `#[repr(C)]` data structures: `FfiCandle`, `FfiQuote`, `FfiArbitrage`. |
| Memory Management | `src/ffi/mod.rs` | Safe C string conversion and deallocation via `tv_streamer_free_string`. |
| C-ABI Test Suite | `tests/test_c_ffi.rs` | Live verification of C-ABI download, quote retrieval, and null safety. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: C-ABI functions callable via standard dynamic linking or foreign function interfaces.
- [x] **AC-02**: `tv_streamer_download_sync` writes native candles into a caller-provided memory buffer.
- [x] **AC-03**: `tv_streamer_quote_sync` populates real-time Level 1 quote snapshots synchronously.
- [x] **AC-04**: 100% memory-safe execution verified with null-pointer checks and boundary tests.

---

## 4. Verification Command
```bash
cargo test --test test_c_ffi
```
