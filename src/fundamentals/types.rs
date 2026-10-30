//! Models for symbol fundamentals, company profile, and financial metrics.

use serde::{Deserialize, Serialize};

/// Comprehensive fundamental snapshot for a financial instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FundamentalOverview {
    /// Ticker symbol (e.g. `"AAPL"`).
    pub symbol: String,
    /// Company profile information.
    pub profile: CompanyProfile,
    /// Valuation multiples and price ratios.
    pub valuation: ValuationMetrics,
    /// Key financial statement ratios and margins.
    pub financials: FinancialMetrics,
    /// Dividend yields and distribution metrics.
    pub dividends: DividendMetrics,
}

/// Company identification and corporate metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CompanyProfile {
    /// Full corporate name (e.g. `"Apple Inc."`).
    pub company_name: String,
    /// Detailed corporate description.
    pub description: Option<String>,
    /// Security type (e.g. `"stock"`, `"crypto"`, `"fund"`).
    pub security_type: Option<String>,
    /// Security sub-type (e.g. `"common"`, `"etf"`).
    pub subtype: Option<String>,
    /// Primary listing exchange (e.g. `"NASDAQ"`).
    pub exchange: String,
    /// Country of incorporation/headquarters.
    pub country: Option<String>,
    /// Broad market sector (e.g. `"Electronic Technology"`).
    pub sector: Option<String>,
    /// Specific industry classification (e.g. `"Telecommunications Equipment"`).
    pub industry: Option<String>,
}

/// Market valuation multiples and per-share figures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValuationMetrics {
    /// Basic market capitalization.
    pub market_cap: Option<f64>,
    /// Trailing twelve months Price-to-Earnings ratio (P/E TTM).
    pub price_earnings_ttm: Option<f64>,
    /// Most recent quarter Price-to-Book ratio (P/B FQ).
    pub price_book: Option<f64>,
    /// Price-to-Sales ratio TTM.
    pub price_sales_ttm: Option<f64>,
    /// Price to Free Cash Flow TTM.
    pub price_free_cash_flow_ttm: Option<f64>,
    /// Basic Earnings Per Share TTM.
    pub earnings_per_share_basic_ttm: Option<f64>,
    /// Book value per share for the most recent fiscal quarter.
    pub book_value_per_share: Option<f64>,
}

/// Key profitability, capital structure, and liquidity metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialMetrics {
    /// Total trailing twelve months revenue.
    pub total_revenue: Option<f64>,
    /// Net income for the fiscal year.
    pub net_income_fy: Option<f64>,
    /// Return on Equity for the most recent quarter (ROE FQ %).
    pub return_on_equity_fq: Option<f64>,
    /// Return on Assets for the most recent quarter (ROA FQ %).
    pub return_on_assets_fq: Option<f64>,
    /// Total Debt-to-Equity ratio.
    pub debt_to_equity: Option<f64>,
    /// Current liquidity ratio (Current Assets / Current Liabilities).
    pub current_ratio: Option<f64>,
    /// Quick liquidity ratio (Acid-test ratio).
    pub quick_ratio: Option<f64>,
    /// Earnings Before Interest, Taxes, Depreciation, and Amortization.
    pub ebitda: Option<f64>,
}

/// Corporate dividend payout and yield figures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DividendMetrics {
    /// Annual dividend yield (e.g. 0.31 for 0.31%).
    pub dividend_yield: Option<f64>,
    /// Trailing twelve months dividend payout ratio percentage.
    pub dividend_payout_ratio_ttm: Option<f64>,
}
