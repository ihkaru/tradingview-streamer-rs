# 09 Architecture Decisions

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `c6038cb` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T22:55:29.431514+07:00`
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

---

## 9.3 ADR-03: Dual-Protocol Transport Architecture (WebSocket + REST)

### Context
Upstream Python scrapers mix WebSocket scraping and HTTP requests inconsistently. Real-time OHLCV and Quote updates require long-lived bidirectional streaming, while bulk market screening, mover rankings, and corporate profiles are point-in-time queries best served by HTTP.

### Decision
Adopt a Dual-Protocol transport design:
- WebSocket client (`data.tradingview.com`) for stateful Chart Sessions (`cs_`) and Quote Sessions (`qs_`).
- Async REST client (`scanner.tradingview.com`) for market screening, movers, technical ratings, and fundamentals.

### Consequences
- **Positive**: Lowest latency for streaming price feeds without socket bloat for one-off screener queries.
- **Risks created**:
  - `RISK-03`: Rate limiting on unauthenticated Scanner HTTP endpoints during high-frequency scanning loops.

---

## 9.4 ADR-04: Explicit Venue Specification & Dynamic Scanner Disambiguation (Anti-Hardcode)

### Context
TradingView requires `EXCHANGE:SYMBOL` syntax (e.g. `NASDAQ:AAPL`, `NYSE:IBM`). Initial heuristic attempts silently defaulted un-prefixed tickers to `"NASDAQ"` or `"BINANCE"`. In institutional quantitative finance, this hardcoded heuristic is hazardous: `IBM` trades on `NYSE` (defaulting to `NASDAQ:IBM` returns empty news and failed technicals), `BBCA` trades on `IDX`, and `BTCUSD` trades across multiple spot venues (Coinbase, Kraken, Bitstamp) with varying spreads. Silent hardcoding contaminates datasets and violates consumer choice.

### Decision
In accordance with `CON-08`, eliminate all silent hardcoded exchange assumptions:
1. Canonical parsing strictly requires explicit `EXCHANGE:TICKER` syntax via `Symbol::parse_strict` (or `FromStr`), failing fast with `Error::MissingExchange` when omitted.
2. Enable explicit consumer-configured default exchanges via `Ticker::new_with_default_exchange("AAPL", "NASDAQ")` and `Symbol::parse_with_default("AAPL", "NASDAQ")`.
3. Provide dynamic, evidence-based venue disambiguation via `DiscoveryClient::resolve_primary_venue` and `Ticker::resolve_dynamic` backed by TradingView's live Global Scanner API (`scanner.tradingview.com/global/scan`), ranking candidates strictly by verifiable 24h trading volume.
4. Support multi-venue inspection via `Ticker::venues()` so users can explicitly review and select their desired execution venue.

### Consequences
- **Positive**: 100% data integrity, prevention of silent venue contamination, full consumer autonomy over execution venues, and authentic market discovery.
- **Risks created**:
  - `RISK-04`: Dynamic resolution requires an initial HTTP lookup when an exchange prefix is omitted without a user default. Consumers seeking zero network overhead should use explicit prefixes (`"NYSE:IBM"`).
