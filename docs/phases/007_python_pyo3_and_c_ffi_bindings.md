# Phase 007: Python (PyO3) & C-ABI Foreign Function Interface

> **Status**: `BACKLOG`  
> **Milestone**: `v0.7.0`  
> **Target Release**: 2028  
> **Primary Focus**: Delivering a blazing-fast Python package (`pip install tradingview-streamer`) powered by Rust.

---

## 1. Objective & Problem Statement
Python is the lingua franca of financial data science and quantitative research. Python libraries like `yfinance` are widely used despite performance and concurrency limitations. 

Phase 007 introduces native Python bindings via `pyo3` and `maturin`, enabling Python developers to ingest TradingView real-time and historical data (including Extended Hours) with Rust's concurrency and zero-GIL overhead.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| PyO3 Extension | `crates/tradingview-py/src/lib.rs` | Python module exposing `TvStreamer` and `fetch_candles()` yielding Pandas / Polars DataFrames. |
| Async Bridge | `crates/tradingview-py/src/async_bridge.rs` | Seamlessly bridges Tokio async runtime with Python's `asyncio` event loop. |
| C-ABI Exports | `src/ffi/mod.rs` | Exported `extern "C"` functions for C/C++ quantitative execution engines. |
| Maturin Packaging | `pyproject.toml` | Build and wheel packaging supporting Python 3.10 through 3.14 on Linux/macOS/Windows. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Python users can install via `pip install tradingview-streamer`.
- [ ] **AC-02**: Python code `streamer.fetch_candles(session="extended")` matches Rust output 100%.
- [ ] **AC-03**: Returns native Arrow / Polars / Pandas structures with zero memory copy.
- [ ] **AC-04**: CI publishes multi-platform wheels to PyPI using GitHub Actions.

---

## 4. Verification Command
```bash
maturin develop
pytest tests/python/
```
