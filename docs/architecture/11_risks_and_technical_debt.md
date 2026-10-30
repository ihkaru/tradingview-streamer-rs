# 11 Risks and Technical Debt

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `4f38783` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 11.1 Risks Register

| Risk ID | Title | Impact | Likelihood | Mitigation Strategy |
|:---|:---|:---:|:---:|:---|
| `RISK-01` | Cloudflare WebSocket Rate Limiting / 429 Errors | High | Medium | Fully mitigated in Phase 006 (`src/supervisor/`) via full-jitter exponential backoff, circuit breaker state machine, and proxy mesh rotation. |
| `RISK-02` | Rustls Crypto Provider Configuration Mismatch | Medium | Low | Explicitly initialize Ring default crypto provider (`install_default`) inside `TvWebSocketClient::connect`. |
| `RISK-03` | TradingView Scanner REST Rate Limiting | High | Medium | Cache screener responses with configurable TTL and integrate HTTP exponential backoff. |
| `RISK-04` | Undocumented Scanner Field Schema Changes | Medium | Low | Implement lenient JSON deserialization with fallback defaults and automated schema drift tests. |

## 11.2 Technical Debt Ledger

| Debt ID | Title | Component | Impact | Remediation Plan |
|:---|:---|:---|:---:|:---|
| `DEBT-01` | Ingestion Allocation via Serde JSON Value | `src/protocol/parser.rs` | Low | Addressed in Phase 005 (`src/export/`) via cache-friendly Structure-of-Arrays `ColumnarCandles` with zero-copy vector transposition and fast CSV export. |
