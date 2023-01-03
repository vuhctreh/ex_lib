use std::time::SystemTime;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::woo::{Woo, WooAuth};

impl WooAuth for Woo {
     fn auth_v1(&self, query: String, timestamp: u128) -> String {
        type HmacSha256 = Hmac<Sha256>;

        let mut sha = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("Unable to instantiate Sha256 HMAC");

        let sig_string: String = query +
            "|" + &timestamp.to_string();

        sha.update(sig_string.as_bytes());

        hex::encode(sha.finalize().into_bytes())
    }

    #[allow(unused)]
    fn auth_v3(&self, query: String) -> String {
        todo!()
    }
}