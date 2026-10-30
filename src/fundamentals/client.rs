//! HTTP client implementation for fetching fundamental corporate metrics.

use std::collections::HashMap;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

use crate::error::{Error, Result};
use crate::types::Symbol;
use super::types::{
    CompanyProfile, DividendMetrics, FinancialMetrics, FundamentalOverview, ValuationMetrics,
};

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

const FUNDAMENTAL_FIELDS: &[&str] = &[
    "name",
    "description",
    "type",
    "subtype",
    "exchange",
    "country",
    "sector",
    "industry",
    "market_cap_basic",
    "price_earnings_ttm",
    "price_book_fq",
    "price_sales_ttm",
    "price_free_cash_flow_ttm",
    "earnings_per_share_basic_ttm",
    "book_value_per_share_fq",
    "total_revenue",
    "net_income_fy",
    "return_on_equity_fq",
    "return_on_assets_fq",
    "debt_to_equity_fq",
    "current_ratio_fq",
    "quick_ratio_fq",
    "EBITDA",
    "dividends_yield",
    "dividend_payout_ratio_ttm",
];

/// HTTP client for querying corporate fundamentals and valuation statistics.
#[derive(Debug, Clone)]
pub struct FundamentalsClient {
    client: reqwest::Client,
}

impl Default for FundamentalsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FundamentalsClient {
    /// Creates a new fundamentals client.
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { client }
    }

    /// Fetches a consolidated fundamental overview for the given symbol.
    pub async fn overview(&self, symbol: impl AsRef<str>) -> Result<FundamentalOverview> {
        let resolved = Symbol::resolve(symbol.as_ref());
        let fields_param = FUNDAMENTAL_FIELDS.join(",");
        let url = format!(
            "https://scanner.tradingview.com/symbol?symbol={}&fields={fields_param}&no_404=true",
            resolved.to_exchange_symbol()
        );

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(Error::Protocol(format!(
                "Fundamentals endpoint returned HTTP status {}",
                response.status()
            )));
        }

        let map: HashMap<String, serde_json::Value> = response.json().await?;
        if map.is_empty() {
            return Err(Error::DataNotFound(format!(
                "No fundamental data found for {}",
                resolved
            )));
        }

        self.map_to_overview(resolved.ticker, map)
    }

    fn map_to_overview(
        &self,
        symbol: String,
        map: HashMap<String, serde_json::Value>,
    ) -> Result<FundamentalOverview> {
        let profile = CompanyProfile {
            company_name: get_str(&map, "description")
                .unwrap_or_else(|| get_str(&map, "name").unwrap_or_else(|| symbol.clone())),
            description: get_str(&map, "description"),
            security_type: get_str(&map, "type"),
            subtype: get_str(&map, "subtype"),
            exchange: get_str(&map, "exchange").unwrap_or_default(),
            country: get_str(&map, "country"),
            sector: get_str(&map, "sector"),
            industry: get_str(&map, "industry"),
        };

        let valuation = ValuationMetrics {
            market_cap: get_f64(&map, "market_cap_basic"),
            price_earnings_ttm: get_f64(&map, "price_earnings_ttm"),
            price_book: get_f64(&map, "price_book_fq"),
            price_sales_ttm: get_f64(&map, "price_sales_ttm"),
            price_free_cash_flow_ttm: get_f64(&map, "price_free_cash_flow_ttm"),
            earnings_per_share_basic_ttm: get_f64(&map, "earnings_per_share_basic_ttm"),
            book_value_per_share: get_f64(&map, "book_value_per_share_fq"),
        };

        let financials = FinancialMetrics {
            total_revenue: get_f64(&map, "total_revenue"),
            net_income_fy: get_f64(&map, "net_income_fy"),
            return_on_equity_fq: get_f64(&map, "return_on_equity_fq"),
            return_on_assets_fq: get_f64(&map, "return_on_assets_fq"),
            debt_to_equity: get_f64(&map, "debt_to_equity_fq"),
            current_ratio: get_f64(&map, "current_ratio_fq"),
            quick_ratio: get_f64(&map, "quick_ratio_fq"),
            ebitda: get_f64(&map, "EBITDA"),
        };

        let dividends = DividendMetrics {
            dividend_yield: get_f64(&map, "dividends_yield"),
            dividend_payout_ratio_ttm: get_f64(&map, "dividend_payout_ratio_ttm"),
        };

        Ok(FundamentalOverview {
            symbol,
            profile,
            valuation,
            financials,
            dividends,
        })
    }
}

fn get_str(map: &HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
    map.get(key).and_then(|v| v.as_str()).map(str::to_string)
}

fn get_f64(map: &HashMap<String, serde_json::Value>, key: &str) -> Option<f64> {
    map.get(key).and_then(|v| v.as_f64())
}
