//! A Rust library aiming to serve as an SDK for programmatic
//! trading of Cryptocurrencies on various exchanges (in-dev).
//!
//! Currently supported Exchanges are:
//!
//! - `Woo Exchange` <https://woo.org/>

pub mod client;
pub mod woo;


#[test]
fn test() {
    println!("Hello, world!");
}
