//! A Rust library aiming to serve as an SDK for programmatic
//! trading of Cryptocurrencies on various exchanges (in-dev).
//!
//! Currently supported Exchanges are:
//!
//! - `Woo Exchange` <https://woo.org/>

use crate::client::Client;
use crate::woo::Emit;

pub mod client;
pub mod woo;

//TODO: rearrange http and auth modules so they are not visible from woo.
#[tokio::test]
async fn test() {
    let client = Client::new("".to_string(), "".to_string());

    let woo = client.to_woo();

    println!("{:?}", woo.get_funding_rate_history("PERP_BTC_USDT".to_string(), None, None, None).await);
}
