# Phase 012: Symbol Fundamentals, Valuation Multiples & Financial Statements

> **Status**: `COMPLETED`  
> **Milestone**: `v0.12.0`  
> **Target Release**: Q3 2027  
> **Primary Focus**: Comprehensive company profiles, valuation metrics, and quarterly/annual financial statement ratios.

---

## 1. Objective & Problem Statement
Fundamental analysis requires granular company information, capital structure data, and valuation multiples.
Upstream Python `tradingview-scraper` (`overview.py` and `fundamental_graphs.py`) extracts basic profile data, valuation ratios, and graph metrics across stocks and crypto.

Phase 012 builds a robust Rust fundamentals module providing structured access to corporate profiles (market cap, shares float, employees, sector/industry), valuation multiples (P/E, P/B, P/S, EV/EBITDA, dividend yield), and fundamental statement metrics over quarterly and annual periods.

---

## 2. Target Deliverables & Architectural Modules

| Planned Module | Target File | Implemented Capabilities |
|:---|:---|:---|
| Profile Models | `src/fundamentals/types.rs` | Strongly typed representations for `CompanyProfile`, `Sector`, `Industry`, `CapitalStructure`. |
| Valuation Multiples | `src/fundamentals/types.rs` | Struct containing `ValuationMetrics` (P/E TTM, P/B, P/S, EV/EBITDA, Debt/Equity, ROE, Current Ratio). |
| Financial Statement Metrics | `src/fundamentals/types.rs` | Extraction of fundamental statement metrics (Revenue, Net Income, ROE, ROA, Current Ratio). |
| Fundamentals Client | `src/fundamentals/client.rs` | REST client querying `scanner.tradingview.com/symbol` with smart exchange resolution. |
| Fundamentals Facade | `src/ticker.rs` & `src/lib.rs` | 1-line methods: `tradingview_streamer::overview("AAPL")`, `Ticker::new("AAPL").overview().await?`. |
| Fundamentals Test Suite | `tests/e2e_fundamentals.rs` | End-to-end verification of fundamental and profile data retrieval. |

---

## 3. Acceptance Criteria

- [x] **AC-01**: Retrieve comprehensive profile information (company name, description, country, sector, industry, exchange).
- [x] **AC-02**: Parse valuation metrics and financial ratios with proper float handling and `Option<f64>` for missing values (e.g. non-dividend paying stocks).
- [x] **AC-03**: Integrate with `Ticker` API to mirror Python's `yfinance.Ticker.info` ergonomics.
- [x] **AC-04**: Hermetic test validating JSON deserialization of corporate fundamentals.

---

## 4. Verification Command
```bash
cargo test --test e2e_fundamentals
```
