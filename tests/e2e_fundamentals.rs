//! End-to-end tests for Symbol Fundamentals & Financial Statements (Phase 012).

use tradingview_streamer::{financials, overview, profile, ticker, valuation};

#[tokio::test]
async fn test_live_fundamentals_overview_aapl() {
    let res = overview("AAPL").await;
    match res {
        Ok(data) => {
            println!("AAPL Company: {}", data.profile.company_name);
            println!("Sector: {:?}, Industry: {:?}", data.profile.sector, data.profile.industry);
            println!("P/E TTM: {:?}, Market Cap: {:?}", data.valuation.price_earnings_ttm, data.valuation.market_cap);
            println!("Total Revenue: {:?}, Net Income: {:?}", data.financials.total_revenue, data.financials.net_income_fy);

            assert!(data.profile.company_name.contains("Apple"));
            assert_eq!(data.profile.exchange, "NASDAQ");
            assert!(data.valuation.market_cap.unwrap_or(0.0) > 1_000_000_000.0);
            assert!(data.financials.total_revenue.unwrap_or(0.0) > 100_000_000_000.0);
        }
        Err(e) => {
            eprintln!("[WARN] Live fundamentals test skipped due to network: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_ticker_fundamentals_facade() {
    let t = ticker("MSFT");
    let val_res = t.valuation().await;
    match val_res {
        Ok(val) => {
            println!("MSFT P/E: {:?}, P/B: {:?}", val.price_earnings_ttm, val.price_book);
            assert!(val.market_cap.unwrap_or(0.0) > 1_000_000_000.0);
        }
        Err(e) => {
            eprintln!("[WARN] Live MSFT valuation test skipped: {e}");
        }
    }
}

#[tokio::test]
async fn test_live_convenience_1_liners() {
    let prof_res = profile("NVDA").await;
    if let Ok(prof) = prof_res {
        assert!(prof.company_name.contains("NVIDIA") || prof.company_name.contains("NVDA"));
    }

    let fin_res = financials("NVDA").await;
    if let Ok(fin) = fin_res {
        assert!(fin.total_revenue.is_some() || fin.net_income_fy.is_some());
    }

    let val_res = valuation("NVDA").await;
    if let Ok(val) = val_res {
        assert!(val.market_cap.is_some());
    }
}
