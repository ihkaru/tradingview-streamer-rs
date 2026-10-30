# Phase 004: Multi-Symbol Multiplexing & Connection Pooling

> **Status**: `COMPLETED`  
> **Milestone**: `v0.4.0`  
> **Target Release**: Q2 2027  
> **Primary Focus**: High-concurrency portfolio monitoring across 100+ symbols over a single or pooled WebSocket.

---

## 1. Objective & Problem Statement
Creating an individual WebSocket connection per symbol does not scale for portfolio scanners or market screamers monitoring dozens of assets simultaneously. It wastes socket descriptors and quickly triggers Cloudflare anti-bot / IP connection limiters.

Phase 004 implements symbol multiplexing within shared quote/chart sessions, alongside an adaptive connection pool that dynamically balances subscriptions across a managed set of WebSocket connections.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Multi-Symbol Manager | `src/pool/multiplexer.rs` | Dispatches multiple symbols over a single session and demultiplexes quote streams. |
| Connection Pool | `src/pool/manager.rs` | Spawns and manages worker sockets when symbol thresholds are exceeded. |
| Dedicated Worker | `src/pool/worker.rs` | WebSocket worker handling up to N symbols concurrently with dynamic commands. |
| Portfolio API | `src/pool/mod.rs` | Batch portfolio API: `tradingview_streamer::portfolio_quotes(&["NASDAQ:AAPL", "BINANCE:BTCUSDT"])`. |
| Pool Test Suite | `tests/e2e_multiplexing_pool.rs` | Hermetic and live verification of multi-symbol routing and dynamic subscription. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: A single WebSocket connection reliably streams real-time updates for multiple concurrent symbols.
- [x] **AC-02**: Adding or unsubscribing a symbol at runtime does not cause frame drops on other active symbol series.
- [x] **AC-03**: Connection pool automatically balances subscriptions evenly across connections.
- [x] **AC-04**: Hermetic test verifies multi-symbol demultiplexing and routing to individual consumer channels.

---

## 4. Verification Command
```bash
cargo test --test e2e_multiplexing_pool
```
