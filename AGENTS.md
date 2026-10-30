# AGENTS.md - Coding Agent Guidelines & Integration Guide

> [!NOTE]
> Standard repository specification for autonomous coding agents (AAIF / Linux Foundation aligned).
> For deep architectural details, see [GEMINI.md](GEMINI.md) and [docs/architecture/](docs/architecture/).

## 1. Project Purpose & Interface Contract
`tradingview-streamer` is a pure async Rust crate for streaming real-time and historical OHLCV market data from TradingView WebSockets.
- Resolves upstream **[mnwato/tradingview-scraper#63](https://github.com/mnwato/tradingview-scraper/issues/63)** by providing native Extended Trading Hours (ETH) support via `SessionType::Extended`.
- Public API contract: [`TvStreamer`], [`TvStreamerBuilder`], [`SessionType`], [`Timeframe`], [`OhlcvCandle`].

## 2. Integration into Other Projects

### A. Via crates.io (Standard)
```toml
[dependencies]
tradingview-streamer = "0.1"
tokio = { version = "1.43", features = ["full"] }
```

### B. Via Git Repository
```toml
[dependencies]
tradingview-streamer = { git = "https://github.com/ihkaru/tradingview-streamer-rs", branch = "main" }
tokio = { version = "1.43", features = ["full"] }
```

### C. 3-Line Developer Usage Pattern
```rust
use tradingview_streamer::{TvStreamer, SessionType, Timeframe};

let streamer = TvStreamer::builder()
    .symbol_str("NASDAQ:AAPL")?
    .session(SessionType::Extended) // Includes pre & post market bars
    .build()?;

let candles = streamer.fetch_candles().await?;
```

## 3. Build & Test Commands (The Gauntlet)
Agents modifying this repository MUST run these verification commands before marking tasks complete:

```bash
# 1. Compile & Check
cargo check --all-targets

# 2. Parallel Test Suite (Unit + Hermetic E2E + Multi-Ticker Matrix)
cargo test --all-targets

# 3. Linter & Formatting
cargo clippy --all-targets -- -D warnings
cargo fmt --check

# 4. Zero-Mock Audit
detect-mocks .

# 5. Architecture Drift & Provenance Audit
arc42-lint docs/architecture
```

## 4. Invariants & Guardrails
- **File & Function Line Limits**: Every source file in `src/` MUST be `< 350 lines`. Individual functions MUST be `< 50 lines`.
- **Zero-Mock Policy**: Never use `mockall`, dummy stubs, or fake returns in production paths (`src/`). Integration tests in `tests/` must use either genuine protocol frames against in-process mock TCP listeners or real live servers.
- **Conventional Commits & SemVer**:
  - `feat(...)`: Triggers minor version bump (non-breaking additive API).
  - `fix(...)`: Triggers patch version bump (bug fix).
  - `feat(...)!` or `BREAKING CHANGE:`: Triggers major version bump.
  - Automated by `release-plz` and checked by `cargo-semver-checks`.

## 5. Roadmap & Modular Phase Architecture
All ongoing feature tasks are organized into numbered phase documents:
- Master Dashboard: [ROADMAP.md](ROADMAP.md)
- Phase Specifications: `docs/phases/<NNN>_<slug>.md` (e.g. [001](docs/phases/001_core_streamer_and_extended_hours.md), [002](docs/phases/002_technical_indicators_and_studies.md), [003](docs/phases/003_deep_historical_backfill_and_paging.md))
- When implementing a new feature or phase, an agent MUST:
  1. Review the corresponding `docs/phases/<NNN>_*.md` file.
  2. Implement code adhering to `< 350 lines` per file and zero-mock constraints.
  3. Verify all Acceptance Criteria (`AC-xx`) defined in the phase file.
  4. Run the full Gauntlet and audit architecture with `arc42-lint docs/architecture`.
