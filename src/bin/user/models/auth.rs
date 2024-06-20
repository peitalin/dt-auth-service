///////////////////////////////////////////////
////////////// Request Extractors /////////////
///////////////////////////////////////////////

/// Extract and deserialize incoming requests into form structs,
/// while checking for JWT authorization in headers
/// 1. The 'FromRequest' trait is responsible for authorization

use actix_web::{
    FromRequest,
    Error,
    HttpRequest,
    dev::Payload,
};
use actix_identity::{Identity};
use crate::auth::{decode_token};
use crate::models::{User, LoginError, ErrJson};

/////////////////////////////////
///////////// Login /////////////
/////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginEmail {
    pub email: String,
}
impl From<LoginForm> for LoginEmail {
    fn from(login_form: LoginForm) -> Self {
        LoginEmail { email: login_form.email }
    }
}
impl From<User> for LoginEmail {
    fn from(user: User) -> Self {
        LoginEmail { email: user.email }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserId {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserEmail {
    pub user_email: String,
}

///////////// Create User /////////////
// Deserializes incoming requests into structs

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserForm {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserForm {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordForm {
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestResetPasswordForm {
    pub email: String,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(AsExpression, FromSqlRow)]
#[sql_type = "Text"] // Declare type as Text for PostgreSQL
pub enum UserRole {
  /// Not logged in
  ANON,
  /// Logged in as somebody
  USER,
  /// Logged in as a dealer
  DEALER,
  /// A platform owner superuser (ie us)
  PLATFORM_ADMIN,
  /// The system / not a human
  SYSTEM,
}
// update dt-payments if this changes

impl UserRole {
    fn as_str(&self) -> &str {
        match *self {
            UserRole::ANON => "ANON",
            UserRole::USER => "USER",
            UserRole::DEALER => "DEALER",
            UserRole::PLATFORM_ADMIN => "PLATFORM_ADMIN",
            UserRole::SYSTEM => "SYSTEM",
        }
    }
    fn as_string(&self) -> String {
        String::from(format!("{:?}", &self))
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "ANON" => UserRole::ANON,
            "USER" => UserRole::USER,
            "DEALER" => UserRole::DEALER,
            "PLATFORM_ADMIN" => UserRole::PLATFORM_ADMIN,
            "SYSTEM" => UserRole::SYSTEM,
            _ => UserRole::ANON,
        }
    }
}

impl std::default::Default for UserRole {
    fn default() -> Self {
        UserRole::USER
    }
}

use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::{Double, Float, Jsonb, Text};

// Diesel
impl ToSql<Text, Pg> for UserRole {
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, Pg>) -> ::diesel::serialize::Result {
        let stance = self.as_string();
        ToSql::<Text, Pg>::to_sql(&stance, out)
    }
}
impl FromSql<Text, Pg> for UserRole {
    fn from_sql(maybe_bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let stance = <String as FromSql<Text, Pg>>::from_sql(maybe_bytes)
            .expect("Error parsing UserRole: <String as FromSql<Text, Pg>>");
        Ok(UserRole::from(stance))
    }
}
