use serde::Deserialize;

// Can solve this with generics?
#[derive(Debug, Deserialize)]
pub struct ResponseWrapper {
    pub success: bool,
    pub info: ExchangeInformation
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInformation {
    pub symbol: String,
    pub quote_min: i64,
    pub quote_max: i64,
    pub quote_tick: f64,
    pub base_min: f64,
    pub base_max: i64,
    pub base_tick: f64,
    pub min_notional: f64,
    pub price_range: f64,
    pub created_time: String,
    pub updated_time: String,
}

// #[derive(Debug, Deserialize)]
// pub struct FundingRateHistory