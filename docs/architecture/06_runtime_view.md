# 06 Runtime View

> [!INFO] **Architecture Provenance & Freshness**
> - **Verified Commit**: `d7b3d39` (branch: `main`)
> - **Verified Timestamp**: `2026-10-30T18:45:58+07:00`
> - **Tracked Paths**: `src/`, `tests/`, `Cargo.toml`
> - **Drift Status**: `verified`

## 6.1 Scenario 1 - Extended Trading Hours (ETH) Stream Ingestion

```mermaid
sequenceDiagram
    autonumber
    actor App as Consumer App
    participant Facade as TvStreamerFacade
    participant Client as TvWebSocketClient
    participant TV as TradingView Server

    App->>Facade: fetch_candles(NASDAQ:AAPL, Extended)
    Facade->>Client: start_streaming(Symbol, Timeframe, Extended)
    Client->>TV: WSS Handshake (Origin: https://www.tradingview.com)
    TV-->>Client: Handshake 101 Switching Protocols
    Client->>TV: set_auth_token("unauthorized_user_token")
    Client->>TV: chart_create_session("cs_...")
    Client->>TV: resolve_symbol("cs_...", "sds_sym_1", "={...session:extended...}")
    Client->>TV: create_series("cs_...", "sds_1", "s1", "sds_sym_1", "1", 100)
    TV-->>Client: timescale_update (OHLCV candles including pre-market)
    Client-->>Facade: StreamEvent::Candles(Vec<OhlcvCandle>)
    Facade-->>App: Ok(Vec<OhlcvCandle>)
```

## 6.2 Scenario 2 - Automated Heartbeat Keep-Alive Loop

```mermaid
sequenceDiagram
    autonumber
    participant TV as TradingView Server
    participant Client as TvWebSocketClient

    TV->>Client: ~m~5~m~~h~42
    Note over Client: Detects InboundFrame::Heartbeat(42)
    Client->>TV: ~m~5~m~~h~42
    Note over TV: Session remains active
```
