# 05 Building Block View

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `c16220b` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 5.1 Level 1 - Whitebox Overall System

```mermaid
graph TD
    ClientApp["Downstream Application"] -- "IF-02" --> TvStreamerFacade["TvStreamerFacade"]
    TvStreamerFacade --> TvWebSocketClient["TvWebSocketClient"]
    TvStreamerFacade --> SessionManager["SessionManager"]
    TvStreamerFacade --> ScannerClient["ScannerClient"]
    TvStreamerFacade --> QuoteStreamer["QuoteStreamer"]
    TvStreamerFacade --> TechnicalsEngine["TechnicalsEngine"]
    TvWebSocketClient --> ProtocolEngine["ProtocolEngine"]
    TvWebSocketClient -- "IF-01" --> TradingViewCluster["TradingView WebSocket Cluster"]
    ScannerClient -- "IF-03" --> TradingViewScanner["TradingView Scanner Service"]
    ProtocolEngine --> DomainModels["DomainModels"]
```

### Contained Building Blocks

| Building Block Name | Source Location | Responsibility | Interfaces |
|:---|:---|:---|:---|
| TvStreamerFacade | `src/streamer.rs` & `src/ticker.rs` | Provides ergonomic builder API, one-shot candle batch fetching, 1-line facade, and async streaming. | Implements `IF-02` |
| TvWebSocketClient | `src/client.rs` | Manages Tokio WebSocket connections, HTTP header injection, background loops, and heartbeats. | Consumes `IF-01` |
| ProtocolEngine | `src/protocol/` | Encodes outbound JSON RPC commands, parses `~m~` framing, and extracts candle timescale updates. | Internal |
| SessionManager | `src/session.rs` | Generates 12-char randomized chart (`cs_...`) and quote (`qs_...`) session tokens. | Internal |
| DomainModels | `src/types.rs` | Defines strongly typed data structures (`SessionType`, `Timeframe`, `Symbol`, `OhlcvCandle`). | Internal |
| ScannerClient | `src/scanner/` | Dispatches typed filter queries to TradingView's 19-market screener API with async pagination. | Consumes `IF-03` |
| QuoteStreamer | `src/quote/` | Handles Quote Session (`qs_...`) subscriptions for Level 1 ticker tapes (`lp`, `bid`, `ask`, `rtc`, `chp`). | Consumes `IF-01` |
| TechnicalsEngine | `src/technicals/` | Fetches and maps server-side technical summary ratings (`Recommend.All`) and indicator snapshots. | Consumes `IF-03` |

## 5.2 Level 2 - Core Subsystems

### ProtocolEngine Decomposition
- `src/protocol/framing.rs`: Handles raw `~m~<len>~m~` packaging and heartbeat extraction.
- `src/protocol/messages.rs`: Assembles `resolve_symbol` with `"session": "extended"` (Issue #63).
- `src/protocol/parser.rs`: Extracts `timescale_update`, `du` (study updates), and `qsd` (quote updates) into domain structs.

### ScannerClient Decomposition
- `src/scanner/filter.rs`: Type-safe filter predicate builder (`greater`, `less`, `in_range`, `crosses`).
- `src/scanner/market.rs`: Enumeration of 19 global asset scanning markets (`america`, `crypto`, `forex`, `bonds`, `cfd`, `global`).
- `src/scanner/client.rs`: Async HTTP connection manager targeting `https://scanner.tradingview.com`.
