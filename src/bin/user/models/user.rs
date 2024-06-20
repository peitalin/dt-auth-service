use actix_web::{
    http::StatusCode,
    HttpResponse,
};
use failure::Error;
use ring::rand::SystemRandom;
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;
// validation
use validator::{Validate, ValidationError};
use crate::models::validate_unoffensive_name;

// Internal Imports
use dt::utils::dates::from_datetimestr_to_naivedatetime;
use dt::utils::dates::from_datetimestr_to_option_naivedatetime;

/////////// Needed for diesel table schemas
use diesel::prelude::*;
use dt::db::schema::users;
use dt::db::{establish_connection_pg};
use dt::db;
//////////////////////

use crate::db::queries::users_raw::{insert_user_profile, get_user_profile_by_id};

use crate::models::{ LoginError, ErrJson };
use crate::models::auth::UserRole;
use crate::models::generate_user_id::generate_nano_user_id;

// Constants for hashing passwords
static DIGEST_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const PBKDF2_ITERATIONS: u32 = 20_000;

// Type aliases for IDs
pub type UserId = String;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Identifiable, Insertable, Queryable, Validate)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct User {
    pub id: UserId,
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_unoffensive_name")]
    pub first_name: Option<String>,
    #[validate(custom = "validate_unoffensive_name")]
    pub last_name: Option<String>,
    pub password_hash: String,
    pub email_verified: bool,
    #[serde(deserialize_with = "from_datetimestr_to_option_naivedatetime")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(deserialize_with = "from_datetimestr_to_option_naivedatetime")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub is_suspended: bool,
    pub is_deleted: bool,
    pub user_role: Option<UserRole>,
}

impl User {
    pub fn new(
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Self {
        // Uuid as random salt
        // let user_id = format!("user_{}", uuid::Uuid::new_v4());
        let user_id = format!("u{}", generate_nano_user_id());
        let salt = user_id.clone();
        let password_hash = generate_credential(&salt, &password);

        User {
            id: salt,
            email: email,
            first_name: first_name,
            last_name: last_name,
            password_hash: password_hash,
            email_verified: false,
            created_at: None, // PG does this automatically
            updated_at: None, // PG does this automatically
            is_suspended: false,
            is_deleted: false,
            user_role: Some(UserRole::USER),
        }
    }

    pub fn generate_new_password_hash(&self, password: &str) -> String {
        // salt: user_id
        generate_credential(&self.id, password)
    }

    pub fn store_user_profile(
        &self,
        conn: &diesel::PgConnection,
    ) -> Result<Self, LoginError> {
        match insert_user_profile(conn, &self) {
            Ok(user) => Ok(user),
            Err(e) => match e {
                // Match a specific DB error
                // https://docs.diesel.rs/diesel/result/enum.DatabaseErrorKind.html
                diesel::result::Error::DatabaseError(d, _) => match d {
                    // Uniqu violation error returns a STATUS code 400
                    diesel::result::DatabaseErrorKind::UniqueViolation =>
                        Err(LoginError::DuplicateUser(errJson!(e))),
                    // Otherwise return a general database error with STATUS 500
                    _ => Err(LoginError::DatabaseError(errJson!(e))),
                }
                _ => Err(LoginError::DatabaseError(errJson!(e)))
            }
        }
    }

    pub fn verify_credentials(
        &mut self,
        conn: &diesel::PgConnection,
        attempted_password: String
    ) -> Result<User, LoginError> {
        // Check attempted_password matches decoded password_hash
        match base64::decode(&self.password_hash) {
            Err(_) => Err(LoginError::DecodeError(errJson!("Password could not be decoded!"))),
            Ok(mut decoded_password_hash) => {
                // See if salt + attempted_password recreates current password_hash
                let salt = &self.id;
                let authentication = pbkdf2::verify(
                    DIGEST_ALG,
                    NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
                    salt.as_bytes(),
                    attempted_password.as_bytes(),
                    &mut decoded_password_hash,
                ).map_err(|_| LoginError::WrongPassword(errJson!("Wrong password!")));

                // If credentials match, return user profile
                match authentication {
                    Err(e) => Err(e),
                    Ok(_) => get_user_profile_by_id(conn, &self.id),
                }
            }
        }
    }
}

pub fn generate_credential(salt: &str, password: &str) -> String {
    // Store hash(password + salt) into credential
    let mut credential: [u8; CREDENTIAL_LEN] = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        DIGEST_ALG,          // digest_alg: &'static Algorithm,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(), // NonZeroU32
        salt.as_bytes(),     // salt: &[u8]
        password.as_bytes(), // secret: &[u8]
        &mut credential,     // out: &mut [u8]
    );
    // Encode bytearray credential as base64 string
    base64::encode(&credential)
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPublic {
    pub id: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(deserialize_with = "from_datetimestr_to_option_naivedatetime")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        Self {
            id: Some(u.id),
            email: Some(u.email),
            first_name: u.first_name,
            last_name: u.last_name,
            created_at: u.created_at,
        }
    }
}


/// Used for deserializing PayoutMethods from payment service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutMethod {
    pub id: String,
    /// store_id or affiliate_id are both payee_id
    pub payee_id: String,
    #[serde(deserialize_with = "from_datetimestr_to_naivedatetime")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(deserialize_with = "from_datetimestr_to_option_naivedatetime")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub payout_type: Option<String>, // Paypal, Bank, Card
    pub payout_email: Option<String>, // paypal_email
    pub payout_processor: Option<String>, // Paypal, Adyen
    pub payout_processor_id: Option<String>, // some other payment ID
}
