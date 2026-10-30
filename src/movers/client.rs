//! Query dispatcher and record mapper for Market Movers.

use crate::error::Result;
use crate::scanner::{
    Filter, FilterCondition, ScannerClient, ScannerMarket, SortConfig,
};
use super::types::{MoverCategory, MoverRecord};

/// Default columns retrieved for market movers.
const MOVER_COLUMNS: &[&str] = &[
    "name",
    "close",
    "change",
    "change_abs",
    "volume",
    "market_cap_basic",
    "premarket_close",
    "premarket_change",
    "premarket_volume",
    "postmarket_close",
    "postmarket_change",
    "postmarket_volume",
];

/// Client for fetching market movers rankings.
#[derive(Debug, Clone, Default)]
pub struct MoversClient {
    scanner: ScannerClient,
}

impl MoversClient {
    /// Creates a new market movers client.
    pub fn new() -> Self {
        Self {
            scanner: ScannerClient::new(),
        }
    }

    /// Fetches market movers for the given category and market.
    pub async fn fetch(
        &self,
        market: ScannerMarket,
        category: MoverCategory,
        limit: usize,
    ) -> Result<Vec<MoverRecord>> {
        let filters = self.build_filters(category);
        let sort = self.build_sort(category);
        let columns: Vec<String> = MOVER_COLUMNS.iter().map(|&s| s.to_string()).collect();

        let response = self
            .scanner
            .scan(market, &filters, &columns, Some(&sort), (0, limit))
            .await?;

        let records = response
            .items
            .into_iter()
            .map(|item| MoverRecord {
                symbol: item.symbol.clone(),
                name: item.name().unwrap_or(&item.symbol).to_string(),
                price: item.close().unwrap_or(0.0),
                change_percent: item.change().unwrap_or(0.0),
                change_abs: item.change_abs(),
                volume: item.volume().unwrap_or(0.0),
                market_cap: item.market_cap(),
                premarket_change: item.get_f64("premarket_change"),
                premarket_volume: item.get_f64("premarket_volume"),
                postmarket_change: item.get_f64("postmarket_change"),
                postmarket_volume: item.get_f64("postmarket_volume"),
            })
            .collect();

        Ok(records)
    }

    fn build_filters(&self, category: MoverCategory) -> Vec<FilterCondition> {
        match category {
            MoverCategory::Gainers => vec![Filter::new("change").greater(0.0)],
            MoverCategory::Losers => vec![Filter::new("change").less(0.0)],
            MoverCategory::MostActive => vec![Filter::new("volume").greater(0.0)],
            MoverCategory::PennyStocks => vec![
                Filter::new("close").less(5.0),
                Filter::new("volume").greater(10_000.0),
            ],
            MoverCategory::PreMarketGainers => {
                vec![Filter::new("premarket_change").greater(0.0)]
            }
            MoverCategory::PreMarketLosers => vec![Filter::new("premarket_change").less(0.0)],
            MoverCategory::AfterHoursGainers => {
                vec![Filter::new("postmarket_change").greater(0.0)]
            }
            MoverCategory::AfterHoursLosers => {
                vec![Filter::new("postmarket_change").less(0.0)]
            }
        }
    }

    fn build_sort(&self, category: MoverCategory) -> SortConfig {
        match category {
            MoverCategory::Gainers => SortConfig::desc("change"),
            MoverCategory::Losers => SortConfig::asc("change"),
            MoverCategory::MostActive | MoverCategory::PennyStocks => {
                SortConfig::desc("volume")
            }
            MoverCategory::PreMarketGainers => SortConfig::desc("premarket_change"),
            MoverCategory::PreMarketLosers => SortConfig::asc("premarket_change"),
            MoverCategory::AfterHoursGainers => SortConfig::desc("postmarket_change"),
            MoverCategory::AfterHoursLosers => SortConfig::asc("postmarket_change"),
        }
    }
}
