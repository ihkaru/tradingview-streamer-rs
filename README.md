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
tradingview-streamer = { path = "../tradingview-streamer-rs" }
tokio = { version = "1.43", features = ["full"] }
```

---

## 🚀 Quickstart

### 1. Ingesting Extended Hours Candles (Pre & Post Market)

```rust
use tradingview_streamer::{TvStreamer, SessionType, Timeframe};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize streamer with Extended Hours enabled (Issue #63)
    let streamer = TvStreamer::builder()
        .symbol_str("NASDAQ:AAPL")?
        .timeframe(Timeframe::M1)
        .session(SessionType::Extended) // Streams pre-market & after-hours bars
        .candle_count(50)
        .build()?;

    println!("Fetching candles from TradingView...");
    let candles = streamer.fetch_candles().await?;

    println!("Received {} candles:", candles.len());
    for bar in candles.iter().take(5) {
        println!(
            "[{}] Open={:.2} High={:.2} Low={:.2} Close={:.2} Vol={:?}",
            bar.timestamp, bar.open, bar.high, bar.low, bar.close, bar.volume
        );
    }

    Ok(())
}
```

### 2. Live Streaming Updates via Async Channels

```rust
use tradingview_streamer::{TvStreamer, SessionType, Timeframe, StreamEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let streamer = TvStreamer::builder()
        .symbol_str("BINANCE:BTCUSDT")?
        .timeframe(Timeframe::M5)
        .session(SessionType::Regular)
        .build()?;

    let mut event_rx = streamer.stream_events().await?;

    while let Some(event) = event_rx.recv().await {
        match event {
            StreamEvent::Candles(bars) => {
                println!("Received incremental update with {} bars", bars.len());
            }
            StreamEvent::Quote(quote) => {
                println!("Quote update: Last Price={:?}", quote.last_price);
            }
            StreamEvent::Heartbeat(seq) => {
                println!("Heartbeat keep-alive (seq: {})", seq);
            }
            _ => {}
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
