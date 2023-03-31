use crate::woo::{Authenticate, Emit, Woo};
use crate::woo::http::get_v1_no_auth;
use async_trait::async_trait;
use crate::woo::enums::{Side, Timeframe};
use crate::woo::order_struct::{Order, Params, Queryable};
use crate::woo::response::*;

static V1_BASE_URL: &str = "https://api.woo.org/v1";

// TODO enums for various symbols
// Possible to get rid of redundant code here?
#[async_trait]
impl Emit for Woo {
    async fn get_exchange_information(&self, symbol: String) -> ExchangeInformation {
        let query: String = format!("{}/public/info/{}", V1_BASE_URL, symbol);

        match get_v1_no_auth::<ResponseWrapper<ExchangeInformation>>(query).await {
            Ok(body) => body.info,
            Err(e) => panic!("{:?}", e)
        }
    }

    async fn get_funding_rate_history(&self, symbol: String, start_t: Option<u128>, end_t: Option<u128>, page: Option<u128>) -> FundingRateHistory {
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

        match get_v1_no_auth::<FundingRateHistory>(query).await {
            Ok(body) => body,
            Err(e) => panic!("{:?}", e)
        }
    }

    async fn get_token_config(&self) -> Vec<TokenConfig> {

        let url: String = format!("{}/client/token", V1_BASE_URL);

        match self.get_v1_auth::<TokenConfigWrapped>(url, "".to_string()).await {
            Ok(body) => body.rows,
            Err(e) => panic!("{:?}", e)
        }
    }

    async fn get_orderbook_snapshot(&self, symbol: String, max_level: Option<u128>) -> Orderbook {
        let mut url: String = format!("{}/orderbook/{}?", V1_BASE_URL, symbol);

        let mut query: String = "".to_string();

        if max_level.is_some() {
            query.push_str(&format!("max_level={}", max_level.unwrap()));
        }

        url.push_str(&query);

        match self.get_v1_auth::<Orderbook>(url, query).await {
            Ok(body) => body,
            Err(e) => panic!("{:?}", e)
        }
    }

    async fn get_kline(&self, symbol: String, timeframe: Timeframe, limit: Option<u128>) -> Kline {
        let mut url: String = format!("{}/kline?", V1_BASE_URL);

        let mut query: String = format!("symbol={}&type={}", symbol, timeframe.to_string());

        if limit.is_some() {
            query.push_str(&format!("&limit={}", limit.unwrap()));
        }

        url.push_str(&query);

        match self.get_v1_auth::<Kline>(url, query).await {
            Ok(body) => body,
            Err(e) => panic!("{:?}", e)
        }
    }

    // Come back to this (return types)
    // async fn get_holdings(&self) -> String {
    //     let url: String = format!("{}/client/holding", V1_BASE_URL);
    //
    //     self.get_v1_auth(url, "".to_string()).await.unwrap()
    // }

    async fn get_account_information(&self) -> AccountInformation {
        let url: String = format!("{}/client/info", V1_BASE_URL);

        match self.get_v1_auth::<AccountInformation>(url, "".to_string()).await {
            Ok(body) => body,
            Err(e) => panic!("{:?}", e)
        }
    }

    async fn get_token_deposit_address(&self, token: String) -> TokenDepositAddress {
        let mut url: String = format!("{}/asset/deposit?", V1_BASE_URL);

        let query: String = format!("token={}", token);

        url.push_str(&query);

        match self.get_v1_auth::<TokenDepositAddress>(url, query).await {
            Ok(body) => body,
            Err(e) => panic!("{:?}", e)
        }
    }

    async fn send_order<T: Queryable + Send>(&self, order: T) -> String {
        let mut url: String = format!("{}/order?", V1_BASE_URL);

        let query: String = order.to_query_string();

        url.push_str(&query);

        self.post_v1_auth(url, query).await.unwrap()
    }

    async fn cancel_order(&self, order_id: String, symbol: String) -> Cancel {
        let mut url: String = format!("{}/order?", V1_BASE_URL);

        let query: String = format!("order_id={}&symbol={}", order_id, symbol);

        url.push_str(&query);

        match self.get_v1_auth::<Cancel>(url, query).await {
            Ok(body) => body,
            Err(e) => panic!("{:?}", e)
        }
    }
}

