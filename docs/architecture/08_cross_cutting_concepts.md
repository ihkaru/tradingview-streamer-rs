# 08 Cross-Cutting Concepts

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `96d2a43` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 8.1 Error Handling Strategy
- Standardized typed error taxonomy using [`thiserror::Error`].
- No silent fallbacks: operations fail-fast and return typed `Error` variants (`WebSocket`, `Protocol`, `Timeout`, `InvalidSymbol`, `ScannerError`).
- Downstream callers match explicitly on error cases without relying on string inspection.

## 8.2 Concurrency & Asynchronous Model
- Fully cooperative async multitasking via `tokio`.
- Client I/O runs in a spawned background loop, communicating with caller code via bounded `mpsc::channel(256)`.
- If the consumer stops reading from the channel, backpressure naturally throttles the WebSocket worker.
- HTTP REST scanner calls reuse persistent connection pools to eliminate TCP/TLS handshake latency.

## 8.3 Smart Symbol Resolution & Exchange Inference
- Downstream developers can pass plain ticker strings (`"AAPL"`, `"BTCUSDT"`, `"EURUSD"`).
- The symbol resolver applies deterministic heuristics to infer the primary exchange prefix (`NASDAQ:`, `BINANCE:`, `FX_IDC:`) while allowing explicit overrides (`"NYSE:IBM"`).

## 8.4 Security & Transport Encryption
- TLS encryption via `tokio-rustls` with `ring` crypto provider and embedded Mozilla roots (`webpki-roots`).
- Authentication tokens (`set_auth_token`) default to public `"unauthorized_user_token"`, with optional support for user-supplied session tokens.
- No secrets or credentials are ever written to disk or logs; publishing credentials are injected strictly via Infisical.
