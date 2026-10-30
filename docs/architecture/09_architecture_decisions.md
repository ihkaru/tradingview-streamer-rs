# 09 Architecture Decisions

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `4f38783` (branch: `main`)
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

## 9.4 ADR-04: yfinance-Grade Ergonomic Facade with Smart Exchange Inference

### Context
TradingView requires strict `EXCHANGE:SYMBOL` syntax (e.g. `NASDAQ:AAPL`), causing confusion and friction for developers accustomed to Python's `yfinance` where passing `"AAPL"` works instantly.

### Decision
Implement a smart exchange inference layer that deterministically maps plain symbols to their primary exchanges (`"AAPL"` -> `NASDAQ:AAPL`, `"BTCUSDT"` -> `BINANCE:BTCUSDT`), backed by 1-line top-level helpers (`tradingview_streamer::download()`).

### Consequences
- **Positive**: Zero-boilerplate developer experience matching the ergonomics of `yfinance`.
- **Risks created**:
  - `RISK-04`: Ambiguous tickers traded across multiple venues require explicit prefixes (`"NYSE:IBM"`).
