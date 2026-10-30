# 02 Architecture Constraints

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `a93900d` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 2.1 Technical Constraints

| Constraint ID | Constraint | Description |
|:---|:---|:---|
| `CON-01` | Pure Async Rust | 100% Rust implementation utilizing `tokio` (1.43+) and `tokio-tungstenite` (0.26+). |
| `CON-02` | Permissive Enterprise License | Licensed under the MIT License (`LICENSE`) with zero copyleft dependencies. |
| `CON-03` | Cross-Platform Compatibility | Compatible with Linux (x86_64, aarch64), macOS, and Windows with webpki/native root certificates. |
| `CON-04` | Zero Production Mocks | No mock frameworks (`mockall`, `wiremock`) or silent fake stubs permitted in `src/`. |

## 2.2 Organizational & Operational Constraints

| Constraint ID | Constraint | Description |
|:---|:---|:---|
| `CON-05` | Line & Complexity Limits | Source files strictly `< 350 lines`; individual functions strictly `< 50 lines`. |
| `CON-06` | arc42 Compliance | Modular documentation synchronized via `arc42-lint` with Git provenance tags. |
| `CON-07` | Dual-Tier Testing | L1 static analysis via `detect-mocks` combined with L2 empirical hermetic and live E2E test runs. |
