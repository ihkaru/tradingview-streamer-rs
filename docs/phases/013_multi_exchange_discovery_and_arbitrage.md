# Phase 013: Multi-Exchange Instrument Resolution & Global Arbitrage Discovery

> **Status**: `BACKLOG`  
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

| Planned Module | Target File | Planned Capabilities |
|:---|:---|:---|
| Market Discovery Models | `src/markets/discovery.rs` | Struct `ExchangeMarket` (`exchange`, `symbol`, `price`, `volume_24h`, `currency`, `spread`). |
| Cross-Exchange Scanner | `src/markets/scanner.rs` | Scanner querying global venue data for a base ticker. |
| Arbitrage Analysis Helper | `src/markets/arbitrage.rs` | Evaluator computing min/max venue price spread and percentage delta. |
| Ticker Integration | `src/ticker.rs` | `Ticker::new("BTC").markets().await?` returning all traded venues. |
| Markets Test Suite | `tests/markets_e2e.rs` | Hermetic and live verification of multi-exchange discovery. |

---

## 3. Acceptance Criteria

- [ ] **AC-01**: Retrieve all traded venues for a given base asset or stock cross-listing.
- [ ] **AC-02**: Provide sorted venue rankings by 24h liquidity/volume and tightest spread.
- [ ] **AC-03**: Calculate cross-exchange arbitrage spreads with threshold alerts.
- [ ] **AC-04**: Hermetic test validating cross-exchange market response decoding.

---

## 4. Verification Command
```bash
cargo test --test markets_e2e
```
