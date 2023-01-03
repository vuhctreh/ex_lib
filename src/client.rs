use std::fmt::{Display, Formatter};
use std::time::SystemTime;
use crate::woo;
use crate::woo::{EmitV1, Woo, WooAuth};

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
    let timestamp: u128  = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let client: Client = Client::new(String::from("key"), String::from("secret"));

    assert_eq!(client.to_string(), "key: key, secret: sec***, type: undefined".to_string());

    let woo: Woo = client.to_woo();

    assert_eq!(woo.to_string(), "key: key, secret: sec***, type: Woo".to_string());

    println!("{}", woo.auth_v1("a".to_string(), timestamp));

    println!("{}", woo.exchange_information("SPOT_BTC_USDT".to_string()).await);
}