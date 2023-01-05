use serde::Deserialize;

/// API is not consistent, some responses are wrapped like this,
/// others aren't.
#[derive(Debug, Deserialize)]
pub struct ResponseWrapper<T> {
    pub success: bool,
    pub info: T
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub total: i64,
    pub records_per_page: i64,
    pub current_page: i64,
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

#[derive(Debug, Deserialize)]
pub struct FundingRateHistory {
    pub success: bool,
    pub meta: Metadata,
    pub rows: Vec<FundingRate>,
    pub timestamp: u64
}

#[derive(Debug, Deserialize)]
pub struct FundingRate {
    pub symbol: String,
    pub funding_rate: f64,
    pub funding_rate_timestamp: i64,
    pub next_funding_time: i64,
}

/// API is not consistent, some responses are wrapped like this,
/// others aren't.
#[derive(Debug, Deserialize)]
pub struct TokenConfigWrapped {
    pub success: bool,
    pub rows: Vec<TokenConfig>
}

#[derive(Debug, Deserialize)]
pub struct TokenConfig {
    pub token: String,
    pub collateral_ratio: f64,
    pub margin_factor: f64,
    pub futures_margin_factor: f64,
    pub collateral: bool,
    pub can_collateral: bool,
    pub can_short: bool,
    pub stable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Orderbook {
    pub success: bool,
    pub asks: Vec<Ask>,
    pub bids: Vec<Bid>,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]pub struct Ask {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Deserialize)]
pub struct Bid {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Deserialize)]pub struct Kline {
    pub success: bool,
    pub rows: Vec<KlineData>,
}

#[derive(Debug, Deserialize)]
pub struct KlineData {
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub volume: f64,
    pub amount: f64,
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
}