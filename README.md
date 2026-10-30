# 📈 TradingView Streamer (`tradingview-streamer`)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Pure Rust](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![arc42 Freshness](https://img.shields.io/badge/arc42-Verified%20Fresh-brightgreen.svg)](docs/architecture/)

High-performance, pure async Rust client and real-time streaming engine for TradingView WebSocket market data.

Built from first principles to solve upstream issue **[mnwato/tradingview-scraper#63](https://github.com/mnwato/tradingview-scraper/issues/63)** (*"[Feature Request] Support for Extended Hours (Pre-Market & After-Hours) OHLCV Data in Streamer"*).

---

## 🌟 Why This Crate?

Upstream Python clients (like `mnwato/tradingview-scraper`) omit session parameters during symbol resolution:
```python
# Upstream limitation in streamer.py:
resolve_symbol = json.dumps({"adjustment": "splits", "symbol": exchange_symbol})
```
By default, TradingView interprets this as `"session": "regular"`, strictly filtering out:
- **Pre-Market Trading**: `04:00 AM - 09:30 AM EST`
- **Post-Market (After-Hours) Trading**: `04:00 PM - 08:00 PM EST`

`tradingview-streamer` fixes this limitation at the wire level by supporting **Extended Trading Hours (ETH)** directly:
```rust
let streamer = TvStreamer::builder()
    .symbol_str("NASDAQ:AAPL")?
    .timeframe(Timeframe::M1)
    .session(SessionType::Extended) // <--- Enables pre-market & after-hours bars
    .candle_count(100)
    .build()?;
```

---

## ✨ Features

- ⚡ **Pure Async & Low Latency**: 100% async Rust using `tokio` and `tokio-tungstenite`.
- 🌙 **Extended Trading Hours (ETH)**: Seamlessly toggle between `SessionType::Regular` and `SessionType::Extended`.
- 💓 **Automatic Keep-Alive**: Automatically echoes TradingView heartbeat ping frames (`~h~<seq>`) to prevent timeouts.
- 🛡️ **Zero Production Mocks**: 100% real protocol handlers and verified against live TradingView endpoints.
- 🧩 **SOLID & Modularity**: All source files `< 350 lines` and functions `< 50 lines`.
- 🏛️ **Living arc42 Architecture**: Complete 12-section ISO/IEC/IEEE 42010 aligned architecture under `docs/architecture/` with Git provenance.

---

## 📦 Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
tradingview-streamer = "0.1"
tokio = { version = "1.43", features = ["full"] }
```

Or install via `cargo add`:
```bash
cargo add tradingview-streamer
```

---

## 🚀 Quickstart (yfinance-Grade Ergonomics)

### 1. The 1-Liner: Download Recent Bars Instantly
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Zero-boilerplate: smart-detects "AAPL" -> "NASDAQ:AAPL"
    let bars = tradingview_streamer::download("AAPL").await?;
    println!("Downloaded {} bars for AAPL (Latest close: ${:.2})", bars.len(), bars.last().unwrap().close);
    Ok(())
}
```

### 2. The 2-Liner: Ticker with Extended Hours (Issue #63)
Mirroring `yfinance.Ticker("AAPL")`:
```rust
use tradingview_streamer::Ticker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Includes pre-market (04:00-09:30) & after-hours (16:00-20:00 EST)
    let bars = Ticker::new("AAPL").history_extended("1m", 100).await?;
    println!("Received {} 1-min bars with Extended Trading Hours", bars.len());
    Ok(())
}
```

### 3. Real-Time Level 1 Quotes & Ticker Tape Stream
Instantly fetch price snapshots or stream sub-millisecond bid/ask and change updates:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1-Liner Quote Snapshot:
    let q = tradingview_streamer::quote("BTCUSDT").await?;
    println!("BTC/USDT: ${:.2} (24h Change: {:.2}%)", q.price(), q.change_percentage.unwrap_or(0.0));

    // Ticker with Extended Hours Quotes (pre/post-market price shifts):
    let aapl_quote = tradingview_streamer::ticker("AAPL").quote_extended().await?;
    println!("AAPL Price: ${:.2} (Real-time Change: {:?}%)", aapl_quote.price(), aapl_quote.realtime_change_percentage);
    Ok(())
}
```

### 4. Smart Symbol Resolution (Zero Prefix Frustration)
Pass tickers directly; `tradingview-streamer` infers the exchange automatically:
- `"AAPL"` / `"TSLA"` $\rightarrow$ `NASDAQ:AAPL`
- `"BTCUSDT"` / `"ETHUSDT"` $\rightarrow$ `BINANCE:BTCUSDT`
- `"EURUSD"` / `"GBPJPY"` $\rightarrow$ `FX_IDC:EURUSD`
- Or use explicit prefixes whenever needed: `"NYSE:IBM"`, `"BINANCE:SOLUSDT"`.

### 4. Advanced: Fine-Grained Streamer Builder & Indicators
For customized session tokens, timeouts, or indicator integration:
```rust
use tradingview_streamer::{TvStreamer, SessionType, Timeframe};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let streamer = TvStreamer::builder()
        .symbol_str("NASDAQ:AAPL")?
        .timeframe(Timeframe::M1)
        .session(SessionType::Extended) // Pre & post market bars
        .candle_count(50)
        .build()?;

    let candles = streamer.fetch_candles().await?;
    println!("Fetched {} custom candles", candles.len());
    Ok(())
}
```

### 5. Live Real-Time Streaming via Async Channels
```rust
use tradingview_streamer::{Ticker, StreamEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rx = Ticker::new("BTCUSDT").stream("5m").await?;

    while let Some(event) = rx.recv().await {
        if let StreamEvent::Candles(bars) = event {
            println!("Tick update: latest bar close={:.2}", bars.last().unwrap().close);
        }
    }
    Ok(())
}
```

---

## 🔬 The Gauntlet: Verification & Testing

Run unit tests, hermetic in-process WebSocket integration tests, and live network tests:

```bash
# Run all tests in parallel
cargo test

# Run hermetic E2E test only
cargo test --test e2e_hermetic_streamer

# Run live TradingView network test
cargo test --test live_tradingview_e2e -- --nocapture

# Audit Zero-Mock compliance
detect-mocks .

# Verify arc42 architecture freshness
arc42-lint docs/architecture
```

---

## 🗺️ Roadmap & Graduated Phases

The evolution of `tradingview-streamer` toward enterprise and quantitative parity is managed via modular, numbered phase specifications:

- **[Master Strategic Roadmap](ROADMAP.md)**: Full timeline, release cadence, and phase matrix dashboard.
- **[Phase 001 - Core Engine & ETH (Completed)](docs/phases/001_core_streamer_and_extended_hours.md)**: Resolution of `mnwato/tradingview-scraper#63`.
- **[Phase 002 - Indicators & Studies (Planned)](docs/phases/002_technical_indicators_and_studies.md)**: Server-side Volume, RSI, MACD ingestion.
- **[Phase 003 - Historical Backfill (Planned)](docs/phases/003_deep_historical_backfill_and_paging.md)**: Backward timescale paging for >10,000 bars.
- **[Phase 004 - Symbol Multiplexing (Backlog)](docs/phases/004_multi_symbol_multiplexing_and_pool.md)**: Single-socket portfolio monitoring.
- **[Phase 005 - Zero-Copy SIMD & Polars/Arrow (Backlog)](docs/phases/005_simd_json_and_polars_arrow_export.md)**: High-frequency zero-copy analytical export.
- **[Phase 006 - Resilient Supervisor (Backlog)](docs/phases/006_resilient_supervisor_and_proxy_mesh.md)**: Autonomous reconnection and proxy mesh rotation.
- **[Phase 007 - Python PyO3 Bindings (Backlog)](docs/phases/007_python_pyo3_and_c_ffi_bindings.md)**: Native Python package (`pip install tradingview-streamer`).

See [`docs/phases/`](docs/phases/) for all numbered phase implementation documents.

---

## 🏛️ Architecture Documentation

Complete architecture documentation is organized according to the international standard **arc42**:

- [01 Introduction & Quality Goals](docs/architecture/01_introduction_and_goals.md)
- [02 Architecture Constraints](docs/architecture/02_architecture_constraints.md)
- [03 Context & Scope (IF-01, IF-02)](docs/architecture/03_context_and_scope.md)
- [04 Solution Strategy](docs/architecture/04_solution_strategy.md)
- [05 Building Block View](docs/architecture/05_building_block_view.md)
- [06 Runtime View](docs/architecture/06_runtime_view.md)
- [07 Deployment View](docs/architecture/07_deployment_view.md)
- [08 Cross-Cutting Concepts](docs/architecture/08_cross_cutting_concepts.md)
- [09 Architecture Decisions (ADR-01, ADR-02)](docs/architecture/09_architecture_decisions.md)
- [10 Quality Requirements (Q42)](docs/architecture/10_quality_requirements.md)
- [11 Risks & Technical Debt Register](docs/architecture/11_risks_and_technical_debt.md)
- [12 Glossary](docs/architecture/12_glossary.md)

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
