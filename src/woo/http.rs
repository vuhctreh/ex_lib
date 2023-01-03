use reqwest::Client;
use reqwest::header::{CACHE_CONTROL, CONTENT_TYPE, HeaderMap};

pub async fn get_v1_no_auth (url: String) -> Result<String, reqwest::Error> {
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

#[tokio::test]
async fn test_http() {
    let var = get_v1_no_auth( "https://api.woo.org/v1/public/info/SPOT_BTC_USDT".to_string()).await;

    println!("{}", var.unwrap())
}