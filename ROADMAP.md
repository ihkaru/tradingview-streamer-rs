# 🗺️ Master Strategic Roadmap: `tradingview-streamer`

Master execution roadmap for `tradingview-streamer`. Designed to establish the definitive, high-performance async Rust standard for financial market data ingestion, comparable to the ubiquity of `yfinance` in Python and bridging all capabilities from `tradingview-scraper`.

All initiatives follow a **Modular Numbered Phase Architecture** located in [`docs/phases/`](docs/phases/).

---

## 🧭 Executive Horizon & Vision

```mermaid
flowchart TD
    subgraph S1["Phase 1: Real-Time Protocol Core (v0.1 - v0.3)"]
        P001["001: Core WebSocket Engine & Extended Hours (ETH) [COMPLETED]"]
        P008["008: Quote Session & Level 1 Ticker Tape Streamer [COMPLETED]"]
        P002["002: Technical Indicators & Built-in Studies Ingestion [COMPLETED]"]
        P003["003: Deep Historical Backfill & Timescale Paging [COMPLETED]"]
    end

    subgraph S2["Phase 2: Discovery & Screener Intelligence (v0.8 - v0.11)"]
        P009["009: Financial Scanner & Multi-Market Screener Engine [COMPLETED]"]
        P010["010: Market Movers & Extended Hours Leaders (ETH) [COMPLETED]"]
        P011["011: Technical Summary Ratings & Indicator Matrix [COMPLETED]"]
        P013["013: Multi-Exchange Instrument Resolution & Arbitrage [COMPLETED]"]
    end

    subgraph S3["Phase 3: High-Throughput & Analytical Pipelines (v0.4 - v0.5)"]
        P004["004: Multi-Symbol Multiplexing & Connection Pooling"]
        P005["005: Zero-Copy SIMD Deserialization & Polars/Arrow"]
        P012["012: Symbol Fundamentals & Financial Statements [COMPLETED]"]
    end

    subgraph S4["Phase 4: Fundamental & Market Intelligence (v0.14 - v0.16)"]
        P014["014: Corporate Calendars (Earnings, Dividends, Macro) [COMPLETED]"]
        P015["015: Real-Time Market News & Editorial Intelligence Feed [COMPLETED]"]
        P016["016: Community Trader Sentiment, Ideas & Micro-Posts [COMPLETED]"]
    end

    subgraph S5["Phase 5: Ecosystem & Enterprise Hardening (v0.6 - v0.7)"]
        P006["006: Resilient Supervisor, Circuit Breaker & Proxy Rotation"]
        P007["007: Python (PyO3) & C-ABI Foreign Function Interface"]
    end

    P001 --> P008 --> P002 --> P003
    P001 --> P009 --> P010 --> P011 --> P013
    P003 --> P004 --> P005 --> P012
    P009 --> P014 --> P015 --> P016
    P005 --> P006 --> P007
```

---

## 📊 Comprehensive Master Phase Matrix Dashboard

| Phase ID | Title | Target Milestone | Status | Key Deliverable & Upstream Parity |
|:---:|:---|:---:|:---:|:---|
| **[Phase 001](docs/phases/001_core_streamer_and_extended_hours.md)** | Core WebSocket Engine & Extended Trading Hours | `v0.1.0` | `COMPLETED` | Fix for `mnwato/tradingview-scraper#63` via `SessionType::Extended`. |
| **[Phase 002](docs/phases/002_technical_indicators_and_studies.md)** | Technical Indicators & Built-in Studies Ingestion | `v0.2.0` | `COMPLETED` | Automated `create_study` RPC and multi-indicator streaming (`du`). |
| **[Phase 003](docs/phases/003_deep_historical_backfill_and_paging.md)** | Deep Historical Backfill & Timescale Paging | `v0.3.0` | `COMPLETED` | Automated backward paging (`request_more_data`) for >10,000 bars. |
| **[Phase 004](docs/phases/004_multi_symbol_multiplexing_and_pool.md)** | Multi-Symbol Multiplexing & Connection Pooling | `v0.4.0` | `BACKLOG` | Single-socket multi-ticker dispatch and dynamic subscription pool. |
| **[Phase 005](docs/phases/005_simd_json_and_polars_arrow_export.md)** | Zero-Copy SIMD Deserialization & Polars/Arrow | `v0.5.0` | `BACKLOG` | Resolution of `DEBT-01` via SIMD JSON and Apache Arrow export. |
| **[Phase 006](docs/phases/006_resilient_supervisor_and_proxy_mesh.md)** | Resilient Supervisor & Proxy Mesh Rotation | `v0.6.0` | `BACKLOG` | Resolution of `RISK-01` via exponential jitter and SOCKS5 proxy pool. |
| **[Phase 007](docs/phases/007_python_pyo3_and_c_ffi_bindings.md)** | Python (PyO3) & C-ABI Foreign Function Interface | `v0.7.0` | `BACKLOG` | `pip install tradingview-streamer` native Python package. |
| **[Phase 008](docs/phases/008_quote_session_and_ticker_tape.md)** | Real-Time Quote Session & Level 1 Ticker Tape | `v0.8.0` | `COMPLETED` | High-frequency quote stream (`lp`, `bid`, `ask`, `rtc`, `chp`) via `qs_...`. |
| **[Phase 009](docs/phases/009_scanner_and_screener_engine.md)** | Financial Scanner & Multi-Market Screener Engine | `v0.9.0` | `COMPLETED` | REST scanner client for 19 markets with type-safe filter expressions. |
| **[Phase 010](docs/phases/010_market_movers_and_extended_hours_leaders.md)** | Market Movers & Extended Hours Leaders (ETH) | `v0.10.0` | `COMPLETED` | Top gainers/losers, most active, and pre/post-market market leaders. |
| **[Phase 011](docs/phases/011_technical_summary_and_indicator_matrix.md)** | Technical Summary Ratings & Indicator Matrix | `v0.11.0` | `COMPLETED` | Server-evaluated `Recommend.All` rating and 80+ indicator fields snapshot. |
| **[Phase 012](docs/phases/012_symbol_fundamentals_and_financials.md)** | Symbol Fundamentals & Financial Statements | `v0.12.0` | `COMPLETED` | Company profile, capital float, valuation multiples, and statement metrics. |
| **[Phase 013](docs/phases/013_multi_exchange_discovery_and_arbitrage.md)** | Multi-Exchange Discovery & Arbitrage Engine | `v0.13.0` | `COMPLETED` | Global cross-exchange symbol mapping and real-time venue spread detection. |
| **[Phase 014](docs/phases/014_corporate_and_economic_calendars.md)** | Corporate Calendars (Earnings, Dividends, Macro) | `v0.14.0` | `COMPLETED` | Earnings surprise, dividend yield calendars, and macroeconomic releases. |
| **[Phase 015](docs/phases/015_market_news_and_intelligence.md)** | Real-Time Market News & Editorial Intelligence | `v0.15.0` | `COMPLETED` | News headlines, publisher wires, timestamps, and referenced symbol tags. |
| **[Phase 016](docs/phases/016_trader_sentiment_and_ideas.md)** | Community Trader Sentiment, Ideas & Micro-Posts | `v0.16.0` | `COMPLETED` | Community trade setups, price targets, bullish/bearish ratio, and minds posts. |

---

## ⚙️ Phase Execution Workflow for Contributors & AI Agents

To ensure strict engineering governance and prevent architectural drift:
1. **Selecting a Phase**: Pick an item with status `PLANNED` or `BACKLOG`. Read its specification in `docs/phases/<NNN>_*.md`.
2. **Branching & Conventions**: Create branch `feat/phase-<NNN>-<slug>`. All commits must use Conventional Commits (`feat:`, `fix:`).
3. **The Gauntlet Verification**:
   - Every phase MUST satisfy its declared Acceptance Criteria (`AC-01`, `AC-02`, ...).
   - All tests must pass: `cargo test --all-targets`.
   - Maintain zero production mocks: `detect-mocks .`.
   - Update architecture docs: `arc42-lint docs/architecture`.
4. **Graduation & Release**:
   - When merged into `main`, GitHub Actions invokes `release-plz` to generate changelogs, bump SemVer, and publish releases automatically.

---

## 🔗 Related Documentation
- [README.md](README.md) - Crate overview, installation, and quickstart examples.
- [AGENTS.md](AGENTS.md) - Autonomous coding agent guidelines and interface contracts.
- [GEMINI.md](GEMINI.md) - Repository architecture rules and verification gauntlet.
- [docs/architecture/](docs/architecture/) - arc42 international standard architecture documentation.
