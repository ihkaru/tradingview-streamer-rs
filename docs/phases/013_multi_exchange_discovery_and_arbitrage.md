# Phase 013: Multi-Exchange Instrument Resolution & Global Arbitrage Discovery

> **Status**: `COMPLETED`  
> **Milestone**: `v0.13.0`  
> **Target Release**: Q3 2027  
> **Primary Focus**: Global cross-exchange symbol mapping, liquidity discovery, and cross-venue price disparity detection.

---

## 1. Objective & Problem Statement
In global crypto, forex, and cross-listed equities, a single asset (e.g. `BTC` or `BABA`) trades across dozens of disparate venues with varying liquidity, spreads, and local exchange pricing.
Upstream Python `tradingview-scraper` (`symbol_markets.py` and `markets.py`) provides scanners to discover all exchanges offering a given instrument.

Phase 013 implements an automated cross-exchange discovery engine in Rust, allowing quant algorithms to scan all venues trading a symbol, compare bid/ask spreads, and identify cross-venue arbitrage opportunities in real time.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Implemented Capabilities |
|:---|:---|:---|
| Market Venue Models | `src/discovery/types.rs` | Struct `MarketVenue` (`exchange`, `symbol`, `price`, `volume`, `currency`, `security_type`). |
| Arbitrage Model | `src/discovery/types.rs` | Struct `ArbitrageSpread` (`highest_venue`, `lowest_venue`, `spread_abs`, `spread_percent`). |
| Discovery Client | `src/discovery/client.rs` | REST client scanning global venues and computing arbitrage divergence. |
| Discovery Facade | `src/discovery/mod.rs` & `src/ticker.rs` | 1-liners: `tradingview_streamer::discovery::venues("BTCUSD")`, `Ticker::new("AAPL").venues().await?`. |
| Discovery Test Suite | `tests/e2e_discovery.rs` | Live verification of venue discovery and cross-exchange arbitrage spreads. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: Retrieve all traded venues for a given base asset or stock cross-listing.
- [x] **AC-02**: Provide sorted venue rankings by 24h liquidity/volume and tightest spread.
- [x] **AC-03**: Calculate cross-exchange arbitrage spreads with threshold alerts.
- [x] **AC-04**: Hermetic test validating cross-exchange market response decoding.

---

## 4. Verification Command
```bash
cargo test --test e2e_discovery
```
