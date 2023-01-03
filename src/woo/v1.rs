use std::time::SystemTime;
use crate::woo::{Authenticate, Emit, Woo};
use crate::woo::http::get_v1_no_auth;
use async_trait::async_trait;

static V1_BASE_URL: &str = "https://api.woo.org/v1";

#[async_trait]
impl Emit for Woo {

    //TODO make return type a serde struct at some point
    async fn exchange_information(&self, symbol: String) -> String {
        let query: String = format!("{}/public/info/{}", V1_BASE_URL, symbol);

        match get_v1_no_auth(query).await {
            Ok(body) => body,
            Err(e) => panic!("{:?}", e)
        }
    }

    // TODO error handling for get_v1_no_auth
    async fn funding_rate_history(&self, symbol: String, start_t: Option<u128>, end_t: Option<u128>, page: Option<u128>) -> String {
        let mut query: String = format!("{}/public/funding_rate_history?symbol={}", V1_BASE_URL, symbol);

        if start_t.is_some() {
            query.push_str(&format!("&start_t={}", start_t.unwrap()))
        }

        if end_t.is_some() {
            query.push_str(&format!("&end_t={}", end_t.unwrap()));
        }

        if page.is_some() {
            query.push_str(&format!("&page={}", page.unwrap()));
        }

        get_v1_no_auth(query).await.unwrap()
    }

    async fn token_config(&self) -> String {

        let url: String = format!("{}/client/token", V1_BASE_URL);

        self.get_v1_auth(url, "".to_string()).await.unwrap()
    }
}

