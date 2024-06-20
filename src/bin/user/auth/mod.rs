
pub mod actor;
pub mod jwt;

pub use actor::*;
pub use jwt::*;

pub fn create_jwt_secret() -> (String, String) {
    let secret = std::env::var("JWT_ID_KEY")
        .unwrap_or_else(|_| "0x47".repeat(8));
    // 32 char length
    let domain = std::env::var("JWT_DOMAIN")
        .unwrap_or_else(|_| "localhost".to_string());
    (secret, domain)
}
