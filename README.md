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

### 4. Technical Summary Ratings & Indicators (1-Liner)
Fetch server-evaluated consensus ratings (`Strong Buy`, `Buy`, `Neutral`, `Sell`, `Strong Sell`) and indicator values across multiple timeframes:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let summary = tradingview_streamer::technical_rating("AAPL").await?;
    println!("AAPL Overall Rating: {}", summary.all.as_str());
    println!("Moving Averages: {}, Oscillators: {}", summary.moving_averages.as_str(), summary.oscillators.as_str());
    println!("RSI: {:?}, MACD: {:?}", summary.rsi(), summary.macd());
    Ok(())
}
```

### 5. Pre-Market & After-Hours Movers Rankings
Instantly discover pre-market liquidity surges, top gainers, and most active assets:
```rust
use tradingview_streamer::movers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pre_market = movers::pre_market_gainers(10).await?;
    for stock in pre_market {
        println!("{}: ${:.2} (+{:.2}%) Vol: {}", stock.symbol, stock.price, stock.premarket_change.unwrap_or(0.0), stock.volume);
    }
    Ok(())
}
```

### 6. Deep Historical Backfill & Automated Paging (>10,000 Bars)
Iteratively page backwards using TradingView's `request_more_data` RPC:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bars = tradingview_streamer::backfill("AAPL", "1m", 10_000).await?;
    println!("Backfilled {} 1-minute bars backwards from genesis", bars.len());
    Ok(())
}
```

### 7. Financial Scanner & Multi-Market Screener
Run type-safe queries across 19 global asset classes with rich AST filters:
```rust
use tradingview_streamer::scanner::{Filter, ScannerBuilder, ScannerMarket, SortOrder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let results = ScannerBuilder::new(ScannerMarket::America)
        .filter(Filter::new("close").greater(100.0))
        .filter(Filter::new("RSI").less(35.0))
        .sort_by("volume", SortOrder::Desc)
        .limit(20)
        .execute()
        .await?;

    println!("Found {} instruments matching oversold criteria", results.total_count);
    for item in results.items {
        println!("{} Close: ${:.2}", item.symbol, item.close().unwrap_or(0.0));
    }
    Ok(())
}
```

### 8. Symbol Fundamentals & Corporate Profile (yfinance .info Parity)
Retrieve company profile, capital structure, valuation multiples, and statement metrics:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let overview = tradingview_streamer::overview("AAPL").await?;
    println!("Company: {}, Sector: {:?}", overview.profile.company_name, overview.profile.sector);
    println!("P/E: {:?}, Market Cap: {:?}", overview.valuation.price_earnings_ttm, overview.valuation.market_cap);
    println!("Revenue: {:?}, Net Income: {:?}", overview.financials.total_revenue, overview.financials.net_income_fy);
    Ok(())
}
```

### 9. Corporate Calendars (Earnings & Dividends)
Stream corporate action schedules, dividend records, and quarterly earnings surprises:
```rust
use tradingview_streamer::calendar;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let divs = calendar::dividends(5).await?;
    for d in divs {
        println!("{}: Amount ${:?}, Yield: {:?}%", d.symbol, d.amount, d.yield_percent);
    }

    let earnings = calendar::earnings(5).await?;
    for e in earnings {
        println!("{}: EPS Actual: {:?}, Surprise: {:?}%", e.symbol, e.eps_actual, e.eps_surprise_percent);
    }
    Ok(())
}
```

### 10. Real-Time News & Institutional Wires
Fetch headline stories and article links from publisher wires (Reuters, Bloomberg, Seeking Alpha):
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let articles = tradingview_streamer::news("AAPL").await?;
    for story in articles {
        println!("[{}] {} ({})", story.provider, story.title, story.source);
    }
    Ok(())
}
```

### 11. Multi-Exchange Venue Discovery & Arbitrage Spreads
Scan all global venues trading an asset and detect live price discrepancies:
```rust
use tradingview_streamer::discovery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let venues = discovery::venues("BTCUSD").await?;
    println!("Discovered {} venues for BTCUSD", venues.len());

    if let Some(arb) = discovery::arbitrage("BTCUSD").await? {
        println!("BTC Arbitrage: High on {} (${:.2}) vs Low on {} (${:.2}) | Spread: {:.2}%",
            arb.highest_venue, arb.highest_price, arb.lowest_venue, arb.lowest_price, arb.spread_percent);
    }
    Ok(())
}
```

### 12. Community Trader Sentiment, Ideas & Micro-Posts
Ingest published trade ideas, community sentiment indicators, and Minds micro-discussions:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1-Liner: Quantitative crowd sentiment score (-1.0 Bearish to +1.0 Bullish)
    let sentiment = tradingview_streamer::sentiment("BTCUSD").await?;
    println!("BTC Sentiment: {:?}, Score: {:.2} (Sample size: {})",
        sentiment.consensus, sentiment.score, sentiment.total_sample_count);

    // Published community trade setups & chart analysis
    let ideas = tradingview_streamer::ideas("BTCUSD", 1).await?;
    for idea in ideas.iter().take(3) {
        println!("[{:?}] {} by @{} (Likes: {})", idea.sentiment, idea.title, idea.author, idea.likes_count);
    }

    // Minds community micro-discussions
    let minds = tradingview_streamer::minds("NASDAQ:AAPL", 5).await?;
    for post in minds {
        println!("@{} posted: {} (Likes: {})", post.author.username, post.text, post.total_likes);
    }
    Ok(())
}
```

### 13. Multi-Symbol Connection Pool & Portfolio Streaming
Stream 50+ tickers over managed, load-balanced WebSocket connections without socket waste:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbols = &["NASDAQ:AAPL", "NASDAQ:MSFT", "BINANCE:BTCUSDT", "FX_IDC:EURUSD"];
    let (pool, mut rx) = tradingview_streamer::portfolio_quotes(symbols).await?;

    println!("Monitoring portfolio across {} symbols...", pool.active_symbol_count().await);
    while let Some(evt) = rx.recv().await {
        println!("[Portfolio] {}: price=${:.2}, change={:.2}%",
            evt.symbol, evt.quote.price(), evt.quote.change_percentage.unwrap_or(0.0));
    }
    Ok(())
}
```

### 14. Smart Symbol Resolution (Zero Prefix Frustration)
Pass tickers directly; `tradingview-streamer` infers the exchange automatically:
- `"AAPL"` / `"TSLA"` $\rightarrow$ `NASDAQ:AAPL`
- `"BTCUSDT"` / `"ETHUSDT"` $\rightarrow$ `BINANCE:BTCUSDT`
- `"EURUSD"` / `"GBPJPY"` $\rightarrow$ `FX_IDC:EURUSD`
- Or use explicit prefixes whenever needed: `"NYSE:IBM"`, `"BINANCE:SOLUSDT"`.

### 15. Advanced: Fine-Grained Streamer Builder & Indicators
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
- **[Phase 002 - Indicators & Studies (Completed)](docs/phases/002_technical_indicators_and_studies.md)**: Server-side Volume, RSI, MACD study ingestion (`du`).
- **[Phase 003 - Historical Backfill (Completed)](docs/phases/003_deep_historical_backfill_and_paging.md)**: Backward timescale paging for >10,000 bars.
- **[Phase 008 - Quote Session & Level 1 Tape (Completed)](docs/phases/008_quote_session_and_ticker_tape.md)**: Real-time ticker tape (`lp`, `bid`, `ask`, `rtc`, `chp`).
- **[Phase 009 - Financial Scanner & Screener (Completed)](docs/phases/009_scanner_and_screener_engine.md)**: 19 markets with type-safe filter expressions.
- **[Phase 010 - Market Movers & Extended Hours (Completed)](docs/phases/010_market_movers_and_extended_hours_leaders.md)**: Pre-market and after-hours leaders.
- **[Phase 011 - Technical Summary Ratings (Completed)](docs/phases/011_technical_summary_and_indicator_matrix.md)**: Server-evaluated ratings (`Recommend.All`, 80+ indicator fields).
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
