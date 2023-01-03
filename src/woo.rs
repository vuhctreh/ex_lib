mod auth;
mod http;
mod v1;

use std::fmt::{Display, Formatter};
use async_trait::async_trait;

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

pub trait WooAuth {
    fn auth_v1(&self, query: String, timestamp: u128) -> String;
    #[allow(unused)]
    fn auth_v3(&self, query: String) -> String;
}

#[async_trait]
pub trait Emit {
    async fn exchange_information(&self, symbol: String) -> String;
    async fn funding_rate_history(&self, symbol: String, start_t: Option<u128>, end_t: Option<u128>, page: Option<u128>) -> String;
    async fn token_config(&self) -> String;
}

#[async_trait]
pub trait Authenticate {
    async fn get_v1_auth(&self, url: String, query: String) -> Result<String, reqwest::Error>;
}