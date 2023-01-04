use std::time::SystemTime;
use reqwest::Client;
use reqwest::header::{CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE, HeaderMap};
use crate::woo::{Authenticate, Woo, WooAuth};
use async_trait::async_trait;

pub async fn get_v1_no_auth(url: String) -> Result<String, reqwest::Error> {
    let client: Client = Client::new();

    let mut header_map: HeaderMap = HeaderMap::new();

    header_map.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    header_map.insert(CACHE_CONTROL, "no-cache".parse().unwrap());

    let res = client.get(url)
        .headers(header_map)
        .send()
        .await?
        .text()
        .await?;

    Ok::<String, reqwest::Error>(res)
}

//TODO: merge get and post into one with a match

#[async_trait]
impl Authenticate for Woo {
    async fn get_v1_auth(&self, url: String, query: String) -> Result<String, reqwest::Error> {
        let client: Client = Client::new();

        let timestamp: u128  = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut header_map: HeaderMap = HeaderMap::new();

        header_map.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        header_map.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
        header_map.insert("x-api-key", self.key.parse().unwrap());
        header_map.insert("x-api-signature", self.auth_v1(query, timestamp).parse().unwrap());
        header_map.insert("x-api-timestamp", timestamp.to_string().parse().unwrap());

        let res = client.get(url)
            .headers(header_map)
            .send()
            .await?
            .text()
            .await?;

        Ok::<String, reqwest::Error>(res)
    }

    async fn post_v1_auth(&self, url: String, query: String) -> Result<String, reqwest::Error> {
        let client: Client = Client::new();

        let timestamp: u128  = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut header_map: HeaderMap = HeaderMap::new();

        header_map.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        header_map.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
        header_map.insert("x-api-key", self.key.parse().unwrap());
        header_map.insert("x-api-signature", self.auth_v1(query, timestamp).parse().unwrap());
        header_map.insert("x-api-timestamp", timestamp.to_string().parse().unwrap());

        let res = client.post(url)
            .headers(header_map)
            .send()
            .await?
            .text()
            .await?;

        Ok::<String, reqwest::Error>(res)
    }
}