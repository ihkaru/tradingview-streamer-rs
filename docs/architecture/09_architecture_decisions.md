# 09 Architecture Decisions

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `79db8b7` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 9.1 ADR-01: Explicit Session Parameter in `resolve_symbol` (Issue #63 Resolution)

### Context
In `mnwato/tradingview-scraper`, the `resolve_symbol` command omitted the `"session"` property, causing TradingView to default to regular hours only. Users requiring pre-market and after-hours bars (Issue #63) received incomplete datasets.

### Decision
We introduce `SessionType::Extended` (`"session": "extended"`) alongside `SessionType::Regular` (`"session": "regular"`). When constructing `resolve_symbol` and `quote_add_symbols`, the session mode is explicitly serialized into the symbol specification JSON string.

### Consequences
- **Positive**: Complete pre-market and after-hours candle data ingested with zero timestamp discrepancies.
- **Risks created**:
  - `RISK-01`: Rate limiting or IP throttling if excessive concurrent extended-hours streams are opened on public endpoints.

---

## 9.2 ADR-02: Rustls with Ring Crypto Stack

### Context
Using OpenSSL requires native C library dependencies (`libssl-dev`), complicating cross-compilation for ARM64 and container deployments.

### Decision
We standardize on pure-Rust `tokio-rustls` with `ring` crypto provider and `webpki-roots`.

### Consequences
- **Positive**: 100% self-contained compilation, memory-safe TLS parsing, and zero external C library dependencies.
- **Risks created**:
  - `RISK-02`: Upstream breaking API changes in Rustls minor versions regarding global crypto provider initialization.
