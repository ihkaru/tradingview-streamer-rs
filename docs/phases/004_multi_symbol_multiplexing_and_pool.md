# Phase 004: Multi-Symbol Multiplexing & Connection Pooling

> **Status**: `BACKLOG`  
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
| Multi-Symbol Manager | `src/pool/multiplexer.rs` | Dispatches multiple symbols over a single chart/quote session using unique series IDs (`sds_1`, `sds_2`, etc.). |
| Connection Pool | `src/pool/manager.rs` | Spawns and manages worker sockets when symbol thresholds are exceeded. |
| Dynamic Subscription | `src/pool/dynamic.rs` | Allows adding or removing symbols at runtime without interrupting active streams. |
| Screener Facade | `src/screener.rs` | Batch screener API: `screener.subscribe_many(&["NASDAQ:AAPL", "NASDAQ:MSFT", "BINANCE:BTCUSDT"])`. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: A single WebSocket connection reliably streams real-time updates for at least 50 concurrent symbols.
- [ ] **AC-02**: Adding or unsubscribing a symbol at runtime does not cause frame drops on other active symbol series.
- [ ] **AC-03**: Connection pool automatically balances subscriptions evenly across connections.
- [ ] **AC-04**: Hermetic test verifies multi-symbol demultiplexing and routing to individual consumer channels.

---

## 4. Verification Command
```bash
cargo test --test e2e_multiplexing_pool
```
