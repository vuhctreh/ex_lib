use serde::Deserialize;

/// API is not consistent, some responses are wrapped like this,
/// others aren't.
#[derive(Debug, Deserialize)]
pub struct ResponseWrapper<T> {
    pub success: bool,
    pub info: T
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