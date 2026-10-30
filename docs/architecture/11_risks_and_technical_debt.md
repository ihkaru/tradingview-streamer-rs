# 11 Risks and Technical Debt

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `0a0b0e2` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 11.1 Risks Register

| Risk ID | Title | Impact | Likelihood | Mitigation Strategy |
|:---|:---|:---:|:---:|:---|
| `RISK-01` | Cloudflare WebSocket Rate Limiting / 429 Errors | High | Medium | Enforce staggered delays, circuit breaker, and connection pooling (addressed in [Phase 006](../phases/006_resilient_supervisor_and_proxy_mesh.md)). |
| `RISK-02` | Rustls Crypto Provider Configuration Mismatch | Medium | Low | Explicitly initialize Ring default crypto provider (`install_default`) inside `TvWebSocketClient::connect`. |
| `RISK-03` | TradingView Scanner REST Rate Limiting | High | Medium | Cache screener responses with configurable TTL and integrate HTTP exponential backoff. |
| `RISK-04` | Undocumented Scanner Field Schema Changes | Medium | Low | Implement lenient JSON deserialization with fallback defaults and automated schema drift tests. |

## 11.2 Technical Debt Ledger

| Debt ID | Title | Component | Impact | Remediation Plan |
|:---|:---|:---|:---:|:---|
| `DEBT-01` | Ingestion Allocation via Serde JSON Value | `src/protocol/parser.rs` | Low | Replace generic `serde_json::Value` tree allocations with zero-copy stream deserialization for high-throughput ticks (scheduled in [Phase 005](../phases/005_simd_json_and_polars_arrow_export.md)). |
