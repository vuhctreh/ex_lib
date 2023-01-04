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