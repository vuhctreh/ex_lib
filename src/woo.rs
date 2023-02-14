mod auth;
mod http;
mod v1;
mod response;
pub mod enums;
pub mod order_struct;

use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use crate::woo::enums::{Side, Timeframe};
use crate::woo::order_struct::{Order, OrderTypes};
use crate::woo::response::{AccountInformation, ExchangeInformation, FundingRateHistory, Kline, Orderbook, TokenConfig, TokenDepositAddress};

/// Struct for interacting with the Woo Exchange
pub struct Woo {
    pub key: String,
    pub secret: String,
}

// Implementation of Woo methods
impl Woo {
    pub fn new(key: String, secret: String) -> Woo {
        Woo {
            key,
            secret,
        }
    }
}

impl Display for Woo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "key: {}, secret: {}***, type: Woo", self.key, self.secret.get(0..3).unwrap())
    }
}

trait WooAuth {
    fn auth_v1(&self, query: String, timestamp: u128) -> String;
    #[allow(unused)]
    fn auth_v3(&self, query: String) -> String;
}

#[async_trait]
pub trait Emit {
    async fn get_exchange_information(&self, symbol: String) -> ExchangeInformation;
    async fn get_funding_rate_history(&self, symbol: String, start_t: Option<u128>, end_t: Option<u128>, page: Option<u128>) -> FundingRateHistory;
    async fn get_token_config(&self) -> Vec<TokenConfig>;
    async fn get_orderbook_snapshot(&self, symbol: String, max_level: Option<u128>) -> Orderbook;
    async fn get_kline(&self, symbol: String, timeframe: Timeframe, limit: Option<u128>) -> Kline;
    // Come back to this
    // async fn get_holdings(&self) -> String;
    async fn get_account_information(&self) -> AccountInformation;
    async fn get_token_deposit_address(&self, token: String) -> TokenDepositAddress;
    //TODO: make order type enums, parameterise
    async fn send_order<T: OrderTypes + Send>(&self, order: Order<T>) -> String;
}

#[async_trait]
trait Authenticate {
    async fn get_v1_auth<T: DeserializeOwned>(&self, url: String, query: String) -> Result<T, reqwest::Error>;
    async fn post_v1_auth(&self, url: String, query: String) -> Result<String, reqwest::Error>;
}