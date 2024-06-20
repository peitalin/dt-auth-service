use chrono::{Duration, Local};
use jsonwebtoken::{
    encode,
    decode,
    Header,
    Algorithm,
    Validation,
    EncodingKey,
    DecodingKey
};

use crate::models::auth::{LoginEmail, QueryUserId, LoginForm, UserRole};
use crate::models::{ User, LoginError, ErrJson };


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    // issuer
    iss: String,
    // audience: USER, ANON, DEALER, PLATFORM_ADMIN
    aud: UserRole,
    // subject: User ID
    sub: String,
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    // user email
    email: String,
}
impl Claims {
    fn with_email(
        email: String,
        user_id: String,
        user_role: Option<UserRole>,
    ) -> Self {
        dotenv::dotenv().ok();
        Claims {
            iss: std::env::var("JWT_DOMAIN").unwrap_or(String::from("localhost")),
            sub: user_id,
            aud: user_role.unwrap_or(UserRole::USER),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24*30)).timestamp(),
            email: email,
        }
    }
}

impl From<Claims> for LoginEmail {
    fn from(claims: Claims) -> Self {
        LoginEmail {
            email: claims.email,
        }
    }
}

impl From<Claims> for QueryUserId {
    fn from(claims: Claims) -> Self {
        QueryUserId {
            user_id: claims.sub,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthInfo {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub email: String,
    pub user_role: UserRole,
}

// impl AuthInfo {
//     pub fn update_store_id(mut self, store_id: Option<String>) -> Self {
//         self.store_id = store_id;
//         self
//     }
// }

impl From<User> for AuthInfo {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            email: user.email,
            user_role: user.user_role.unwrap_or(UserRole::USER),
        }
    }
}

impl From<Claims> for AuthInfo {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub,
            email: claims.email,
            user_role: claims.aud,
        }
    }
}

pub fn create_token(
    email: String,
    user_id: String,
    user_role: Option<UserRole>,
) -> Result<String, LoginError> {

    let claims = Claims::with_email(email, user_id, user_role);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_ref()),
    ).map_err(|e| LoginError::DecodeError(errJson!(e)))
}

pub fn decode_token<T>(token: &str) -> Result<T, LoginError>
    where T: From<Claims>
{
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_ref()),
        &Validation::default()
    )
    .map(|data| Ok(data.claims.into()))
    .map_err(|e| LoginError::Unauthorized(errJson!(e)))?
}

pub fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "no jwt secrets!".into())
}

