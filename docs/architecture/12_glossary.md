# 12 Glossary

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `c16220b` (branch: `main`)
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
| **Level 1 Market Data** | Best bid, best ask, last traded price (`lp`), daily change (`chp`), and cumulative volume. |
| **Scanner API** | TradingView's high-throughput REST query engine (`scanner.tradingview.com/{market}/scan`) filtering 19 global asset classes. |
| **Technical Rating (`Recommend.All`)** | Server-side consensus score (-1.0 to 1.0) aggregating 26 technical indicators into Strong Buy, Buy, Neutral, Sell, Strong Sell. |
| **Pre-Market Movers** | High-momentum instruments displaying significant volume and percentage price change during pre-market trading (04:00 - 09:30 EST). |
| **Timescale Update** | Asynchronous WebSocket push event delivering newly completed or updated candle bars. |
| **Heartbeat (`~h~`)** | Keep-alive ping frame emitted by TradingView server to confirm client responsiveness. |
