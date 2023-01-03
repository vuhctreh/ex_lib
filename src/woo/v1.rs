use std::time::SystemTime;
use crate::woo::{EmitV1, Woo, WooAuth};
use crate::woo::http::get_v1_no_auth;
use async_trait::async_trait;

static V1_BASE_URL: &str = "https://api.woo.org/v1";

#[async_trait]
impl EmitV1 for Woo {

    //TODO make return type a serde struct at some point
    async fn exchange_information(&self, symbol: String) -> String {
        let query: String = format!("{}/public/info/{}", V1_BASE_URL, symbol);

        match get_v1_no_auth(query).await {
            Ok(body) => body,
            Err(e) => panic!("{:?}", e)
        }
    }

    async fn funding_rate_history(&self, symbol: String, start_t: Option<u128>, end_t: Option<u128>, page: Option<u128>) -> String {


        let timestamp: u128  = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        todo!()
    }
}

