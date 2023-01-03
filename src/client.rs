use std::fmt::{Display, Formatter};
use crate::woo::{Emit, Woo};

// TODO think of some methods here
pub struct Client {
    key: String,
    secret: String,
}

impl Client {
    pub fn new(key: String, secret: String) -> Client {
        Client {
            key,
            secret
        }
    }

    pub fn to_woo(self) -> Woo {
        Woo::new(self.key.to_owned(), self.secret.to_owned())
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "key: {}, secret: {}***, type: undefined", self.key, self.secret.get(0..3).unwrap())
    }
}

#[tokio::test]
async fn test_client() {

    let client: Client = Client::new(String::from("key"), String::from("secret"));

    let woo: Woo = client.to_woo();

    //println!("{}", woo.exchange_information("SPOT_BTC_USDT".to_string()).await);

    //println!("{}", woo.funding_rate_history("PERP_BTC_USDT".to_string(), None, None, None).await)

    println!("{}", woo.orderbook_snapshot("SPOT_BTC_USDT".to_string(), None).await)
}