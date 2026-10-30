# AGENTS.md - Coding Agent Guidelines & Release Governance

> [!NOTE]
> Standard repository specification for autonomous coding agents (AAIF / Linux Foundation aligned).
> For deep architectural details, see [GEMINI.md](GEMINI.md) and [docs/architecture/](docs/architecture/).

## 1. Project Purpose & Interface Contract
`tradingview-streamer` is a pure async Rust crate for streaming real-time and historical OHLCV market data from TradingView WebSockets.
- Resolves upstream **[mnwato/tradingview-scraper#63](https://github.com/mnwato/tradingview-scraper/issues/63)** by providing native Extended Trading Hours (ETH) support via `SessionType::Extended`.
- Public API contract:
  - 1-line facade: `tradingview_streamer::download()`, `tradingview_streamer::download_extended()`
  - High-level ticker: `tradingview_streamer::Ticker` (`history()`, `history_extended()`, `stream()`, `stream_extended()`)
  - Low-level builder: `TvStreamer::builder()`, `SessionType`, `Timeframe`, `OhlcvCandle`.

## 2. Integration into Other Projects

### A. Via crates.io (Standard Release)
```toml
[dependencies]
tradingview-streamer = "0.1"
tokio = { version = "1.43", features = ["full"] }
```
Or via CLI:
```bash
cargo add tradingview-streamer
```

### B. Via Git Repository (Bleeding-Edge)
```toml
[dependencies]
tradingview-streamer = { git = "https://github.com/ihkaru/tradingview-streamer-rs", branch = "main" }
tokio = { version = "1.43", features = ["full"] }
```

### C. 1-Line Minimal Getting Started (yfinance-Grade Ergonomics)
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let candles = tradingview_streamer::download("AAPL").await?;
    println!("Fetched {} candles for AAPL (Latest: ${:.2})", candles.len(), candles.last().unwrap().close);
    Ok(())
}
```

## 3. Build & Test Commands (The Gauntlet)
Agents modifying this repository MUST run these verification commands before marking tasks complete:

```bash
# 1. Compile & Typecheck
cargo check --all-targets

# 2. Parallel Test Suite (Unit + Hermetic Protocol E2E + Multi-Ticker Matrix)
cargo test --all-targets

# 3. Linter & Formatting
cargo clippy --all-targets -- -D warnings
cargo fmt --check

# 4. Line Limit Audit (< 350L per file)
bash scripts/check-line-limits.sh

# 5. Zero-Mock Audit
detect-mocks .

# 6. Architecture Drift & Provenance Audit
arc42-lint docs/architecture

# 7. One-Shot Full Pre-Flight Gate
bash scripts/release-check.sh
```

## 4. Invariants & Guardrails
- **File & Function Line Limits**: Every source file in `src/` MUST be `< 350 lines`. Individual functions MUST be `< 50 lines`.
- **Zero-Mock Policy**: Never use `mockall`, dummy stubs, or fake returns in production paths (`src/`). Integration tests in `tests/` must use either genuine protocol frames against in-process mock TCP listeners or real live servers.
- **Fail-Fast Typed Errors**: Unimplemented paths must return explicit typed errors (`Result<T, TvStreamerError>`), never silent empty fallbacks or dummy successes.
- **Protocol Framing Invariant**: Frames must be formatted as `~m~<len>~m~<json_or_payload>`. Heartbeats matching `~m~<len>~m~~h~<seq>` must be echoed immediately without decompression headers.

## 5. Roadmap & Modular Phase Architecture
All ongoing feature tasks are organized into numbered phase documents:
- Master Dashboard: [ROADMAP.md](ROADMAP.md)
- Phase Specifications: `docs/phases/<NNN>_<slug>.md` (e.g. [001](docs/phases/001_core_streamer_and_extended_hours.md), [002](docs/phases/002_technical_indicators_and_studies.md), [003](docs/phases/003_deep_historical_backfill_and_paging.md))
- When implementing a new feature or phase, an agent MUST:
  1. Review the corresponding `docs/phases/<NNN>_*.md` file.
  2. Implement code adhering to `< 350 lines` per file and zero-mock constraints.
  3. Verify all Acceptance Criteria (`AC-xx`) defined in the phase file.
  4. Run the full Gauntlet and audit architecture with `arc42-lint docs/architecture`.

## 6. Release Management & SemVer Governance

### 6.1 SemVer 2.0.0 & RFC 1105 Taxonomy
The repository strictly enforces Semantic Versioning 2.0.0 and Rust API Evolution RFC 1105 invariants:

| Version Change | Trigger | SemVer Rule | Example |
| :--- | :--- | :--- | :--- |
| **PATCH** (`0.1.X`) | Bug fixes, internal refactoring, doc improvements | 100% backward-compatible, zero public API signature alterations | `fix(session): correct heartbeat seq parser` |
| **MINOR** (`0.X.0`) | New additive public APIs, new indicator studies, new timeframe variants | Backward-compatible additions to public API contracts | `feat(ticker): add live stream_extended method` |
| **MAJOR** (`X.0.0` or breaking in `0.X`) | Breaking public API changes, removed types, changed function signatures | Incompatible API changes requiring consumer code updates | `feat(api)!: migrate to structured session options` |

### 6.2 Conventional Commits Mapping
Automated tooling analyzes commit headers to determine version increment:
- `fix(...)`: Triggers automatic **PATCH** bump.
- `feat(...)`: Triggers automatic **MINOR** bump.
- `feat(...)!`, `fix(...)!`, or commit body with `BREAKING CHANGE:`: Triggers automatic **MAJOR** bump.
- `docs(...)`, `chore(...)`, `test(...)`, `style(...)`: No release bump triggered.

### 6.3 Centralized Secret Ingestion via Infisical
Credentials for crates.io publishing (`CARGO_REGISTRY_TOKEN` / `CRATES_IO_TOKEN`) MUST NEVER be stored in git or plain text files.
All tokens are managed centrally via Infisical:
- **Project ID**: `f13379e0-9661-4f8e-81ef-0e81d1502da1`
- **Environment**: `dev`
- **Secret Keys**: `CARGO_REGISTRY_TOKEN`, `CRATES_IO_TOKEN`

Agents and developers execute commands with secret injection:
```bash
infisical run --projectId f13379e0-9661-4f8e-81ef-0e81d1502da1 --env dev -- <command>
```

### 6.4 Automated Release Engine (`release-plz` + GitHub Actions)
The repository uses [`release-plz`](https://release-plz.dev/) configured in `release-plz.toml` and `ci/workflows/release.yml`:
> [!NOTE]
> Workflow templates are maintained under `ci/workflows/`. To activate automated GitHub Actions on your repository, ensure your GitHub token has the `workflow` scope (`gh auth refresh -s workflow`) and move them to `.github/workflows/`.
1. **Pull Request Automation**: On every commit to `main`, `release-plz` analyzes new conventional commits, runs `cargo-semver-checks`, updates `CHANGELOG.md`, and increments the version in `Cargo.toml`.
2. **Release PR**: A dedicated release PR is created or updated automatically by the bot.
3. **Automated Publishing**: Once the maintainer merges the release PR, `release-plz` automatically:
   - Creates the Git tag (e.g. `v0.1.0`).
   - Generates the GitHub Release with markdown changelog.
   - Publishes the new package version to `crates.io` using the injected `CARGO_REGISTRY_TOKEN`.

### 6.5 Pre-Flight Release Verification Checklist (The 6 Gates)
Before any release or merge of a release PR, execute `bash scripts/release-check.sh` to verify:
1. `cargo test --all-targets` $\rightarrow$ 100% tests pass hermetically and in parallel.
2. `cargo check --all-targets` $\rightarrow$ Clean compilation across all crate targets.
3. `bash scripts/check-line-limits.sh` $\rightarrow$ All source files `< 350L`, functions `< 50L`.
4. `detect-mocks .` $\rightarrow$ Zero test doubles, mock frameworks, or silent fallback stubs in `src/`.
5. `arc42-lint docs/architecture` $\rightarrow$ Fresh architecture documentation with matching Git provenance SHA.
6. `cargo publish --dry-run` $\rightarrow$ Verifies packaging includes only necessary files (`src/`, `Cargo.toml`, `README.md`, `LICENSE`), total compressed size `< 30 KiB`.

### 6.6 Manual CLI Publishing Flow (Maintainer Fast-Path)
In situations where an emergency or manual publish is required directly from the CLI:

```bash
# Step 1: Pre-flight dry-run packaging check
cargo publish --dry-run

# Step 2: Publish to crates.io using Infisical-injected credentials
infisical run --projectId f13379e0-9661-4f8e-81ef-0e81d1502da1 --env dev -- \
  sh -c 'cargo publish --token "$CARGO_REGISTRY_TOKEN"'
```

### 6.7 Post-Release Verification
After publishing:
1. Confirm crate availability: `cargo search tradingview-streamer`
2. Verify documentation generation on [docs.rs/tradingview-streamer](https://docs.rs/tradingview-streamer).
