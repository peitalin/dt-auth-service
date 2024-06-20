
////////////////////////////
//////// HashKey
////////////////////////////

#[derive(Debug, Clone)]
pub struct HashKey(ring::hmac::Tag);

impl std::fmt::Display for HashKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sig_as_ref: &[u8] = self.0.as_ref();
        let hex_digest = data_encoding::HEXLOWER.encode(sig_as_ref);
        write!(f, "{}", hex_digest)
    }
}

impl HashKey {
    pub fn new<'a>(query: &'a str) -> Self {
        use ring::{digest, hmac};
        // Read .env file in local crate
        dotenv::dotenv().ok();
        let access_token = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET not set in environment.");
        // Use secret to hash our GQL queries' lookup keys
        let signing_key = hmac::Key::new(hmac::HMAC_SHA256, access_token.as_bytes());
        let signature = hmac::sign(&signing_key, query.as_bytes());
        HashKey(signature)
    }
    pub fn as_str(&self) -> String {
        format!("{}", &self)
    }
}