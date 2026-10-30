# Phase 006: Resilient Supervisor, Circuit Breaker & Proxy Rotation

> **Status**: `BACKLOG`  
> **Milestone**: `v0.6.0`  
> **Target Release**: Q4 2027  
> **Primary Focus**: Addressing `RISK-01` via autonomous reconnection, circuit breaking, and proxy mesh integration.

---

## 1. Objective & Problem Statement
Under heavy scraping workloads or unstable network connections, WebSocket sockets can drop abruptly, or Cloudflare may temporarily return HTTP 429 Too Many Requests (`RISK-01`). Applications need automated fault recovery that reconnects, re-authorizes sessions, and re-subscribes active symbols transparently.

Phase 006 introduces a supervision actor that manages connection lifecycles, exponential backoff with full jitter, circuit breaking, and optional SOCKS5/HTTP rotating proxy support.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Supervisor Actor | `src/supervisor/actor.rs` | Supervises WebSocket connection state machine (`Connected`, `Reconnecting`, `Cooldown`). |
| Backoff & Jitter | `src/supervisor/backoff.rs` | Full-jitter exponential backoff algorithm avoiding thundering herd problems. |
| Circuit Breaker | `src/supervisor/circuit.rs` | Halts requests when consecutive 429/403 responses exceed safety thresholds. |
| Proxy Rotation | `src/supervisor/proxy.rs` | Rotates outgoing connections through user-provided SOCKS5/HTTP proxy pools. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Unexpected socket closures automatically trigger reconnection within configurable backoff limits.
- [ ] **AC-02**: Active symbol subscriptions and chart sessions are automatically re-registered upon reconnect.
- [ ] **AC-03**: Consecutive 429 rate limits trip the circuit breaker into Cooldown state without crashing.
- [ ] **AC-04**: Proxy rotation seamlessly switches upstream egress IPs across reconnection cycles.

---

## 4. Verification Command
```bash
cargo test --test e2e_supervisor_recovery
```
