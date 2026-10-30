//! Symbol Fundamentals, Valuation Multiples & Financial Statements (Phase 012).
//!
//! Provides company profiles, valuation statistics (P/E, P/B, P/S, EV/EBITDA),
//! financial statement ratios (ROE, ROA, Debt/Equity), and dividend metrics.

pub mod client;
pub mod types;

pub use client::FundamentalsClient;
pub use types::{
    CompanyProfile, DividendMetrics, FinancialMetrics, FundamentalOverview, ValuationMetrics,
};

use crate::error::Result;

/// Fetches comprehensive fundamental overview for a symbol (1-liner).
pub async fn overview(symbol: impl AsRef<str>) -> Result<FundamentalOverview> {
    FundamentalsClient::new().overview(symbol).await
}

/// Fetches valuation metrics for a symbol (1-liner).
pub async fn valuation(symbol: impl AsRef<str>) -> Result<ValuationMetrics> {
    let o = FundamentalsClient::new().overview(symbol).await?;
    Ok(o.valuation)
}

/// Fetches financial statement metrics and profitability ratios (1-liner).
pub async fn financials(symbol: impl AsRef<str>) -> Result<FinancialMetrics> {
    let o = FundamentalsClient::new().overview(symbol).await?;
    Ok(o.financials)
}

/// Fetches corporate profile information (1-liner).
pub async fn profile(symbol: impl AsRef<str>) -> Result<CompanyProfile> {
    let o = FundamentalsClient::new().overview(symbol).await?;
    Ok(o.profile)
}
