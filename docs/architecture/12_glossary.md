# 12 Glossary

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `d03d3ea` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 12.1 Domain & Architectural Terms

| Term | Definition |
|:---|:---|
| **ETH (Extended Trading Hours)** | Trading activity that occurs outside normal exchange operating hours, including pre-market (04:00 - 09:30 EST) and post-market (16:00 - 20:00 EST). |
| **RTH (Regular Trading Hours)** | Standard exchange hours (09:30 - 16:00 EST for US stock markets). |
| **OHLCV** | Open, High, Low, Close price and Volume bar aggregated across a specific timeframe interval. |
| **Framing (`~m~`)** | TradingView's proprietary message wrapping format (`~m~<length>~m~<payload>`) used over raw WebSockets. |
| **Chart Session (`cs_...`)** | Stateful server-side session managing symbol resolution, series calculations, and indicators. |
| **Quote Session (`qs_...`)** | Stateful server-side session providing real-time ticker quotes, bid/ask spreads, and trade metadata. |
| **Timescale Update** | Asynchronous WebSocket push event delivering newly completed or updated candle bars. |
| **Heartbeat (`~h~`)** | Keep-alive ping frame emitted by TradingView server to confirm client responsiveness. |
