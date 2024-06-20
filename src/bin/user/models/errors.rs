use actix_web::{
    http::StatusCode,
    error::ResponseError,
    HttpResponse,
};
use futures::{ future, future::Future };
use failure::Error;
use std::convert::From;

#[derive(Debug, Clone, Serialize, Deserialize, Fail)]
pub struct ErrJson {
    pub file: String,
    pub message: String,
}
impl std::fmt::Display for ErrJson {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
            "{:?}",
            serde_json::to_string(&self).unwrap_or(self.message.clone())
        )
    }
}
impl ErrJson {
    pub fn new(message: &str) -> Self {
        Self {
            file: format!("{}:{}", file!(), line!()),
            message: String::from(message),
        }
    }
}


/// Functions to return a HttpResponse error for invalid requests to Actix Web
pub fn bad_request<E>(error: E) -> impl Future<Output=Result<HttpResponse, actix_web::Error>>
    where E: failure::Fail + serde::Serialize
{
    future::ok(HttpResponse::BadRequest()
        .content_type("application_json")
        .json(error))
}


#[derive(Debug, Fail, Serialize, Deserialize)]
pub enum LoginError {
    #[fail(display = "{}", _0)]
    WrongPassword(ErrJson),
    #[fail(display = "{}", _0)]
    UsernameInvalid(ErrJson),
    #[fail(display = "{}", _0)]
    EmailInvalid(ErrJson),
    #[fail(display = "{}", _0)]
    CredentialsError(ErrJson),
    #[fail(display = "{}", _0)]
    BadRequest(ErrJson),
    #[fail(display = "{}", _0)]
    NoUserError(ErrJson),
    #[fail(display = "{}", _0)]
    DatabaseError(ErrJson),
    #[fail(display = "{}", _0)]
    DecodeError(ErrJson),
    #[fail(display = "{}", _0)]
    Deserialization(ErrJson),
    #[fail(display = "{}", _0)]
    SendgridError(ErrJson),
    #[fail(display = "{}", _0)]
    StripeError(ErrJson),
    #[fail(display = "{}", _0)]
    Timeout(ErrJson),
    #[fail(display = "{}", _0)]
    Unauthorized(ErrJson),
    #[fail(display = "{}", _0)]
    Suspended(ErrJson),
    #[fail(display = "{}", _0)]
    DuplicateUser(ErrJson),
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
       match self {
            LoginError::WrongPassword(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            }
            LoginError::UsernameInvalid(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::EmailInvalid(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::CredentialsError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::BadRequest(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::NoUserError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::DatabaseError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::DecodeError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::Deserialization(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::SendgridError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::StripeError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::Timeout(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::GATEWAY_TIMEOUT)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::Unauthorized(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::Unauthorized()
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::Suspended(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::Unauthorized()
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            LoginError::DuplicateUser(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::BadRequest()
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
       }
    }
}

#[derive(Debug, Fail, Serialize, Deserialize)]
pub enum RpcError {
    #[fail(display = "Error calling dt-payments: {}", _0)]
    Payment(ErrJson),
    #[fail(display = "Error calling dt-user: {}", _0)]
    Customer(ErrJson),
    #[fail(display = "Error calling dt-shopping: {}", _0)]
    UserShoppingDelete(ErrJson),
    #[fail(display = "Error calling dt-notify: {}", _0)]
    Notify(ErrJson),
}

impl ResponseError for RpcError {
    fn error_response(&self) -> HttpResponse {
       match self {
            RpcError::Payment(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            }
            RpcError::Customer(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            }
            RpcError::UserShoppingDelete(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            }
            RpcError::Notify(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
       }
    }
}

#[derive(Debug, Serialize, Deserialize, Fail)]
pub enum AuthError {
    #[fail(display = "{}", _0)]
    NotWorthyEnough(ErrJson),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
       match self {
            AuthError::NotWorthyEnough(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
       }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PaginateError {
    b64DecodeError(ErrJson),
    InvalidCursor(ErrJson),
}

impl ResponseError for PaginateError {
    fn error_response(&self) -> HttpResponse {
       match self {
            PaginateError::b64DecodeError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            PaginateError::InvalidCursor(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            }
       }
    }
}

impl From<base64::DecodeError> for PaginateError {
    fn from(e: base64::DecodeError) -> Self {
        match e {
            base64::DecodeError::InvalidByte(_u, _r) =>
                PaginateError::b64DecodeError(ErrJson::new("err decoding b64 cursor")),
            base64::DecodeError::InvalidLength =>
                PaginateError::b64DecodeError(ErrJson::new("err decoding b64 cursor")),
            base64::DecodeError::InvalidLastSymbol(_u, _r) =>
                PaginateError::b64DecodeError(ErrJson::new("err decoding b64 cursor")),
        }
    }
}

impl From<std::str::Utf8Error> for PaginateError {
    fn from(_e: std::str::Utf8Error) -> Self {
        PaginateError::b64DecodeError(ErrJson::new("err decoding Utf8 b64 cursor"))
    }
}

impl std::fmt::Display for PaginateError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&format!("PaginateError: {:?}", self))
    }
}

impl std::error::Error for PaginateError {
    fn description(&self) -> &str {
        "Error decoding b64 cursor for pagination!"
    }
}


#[derive(Debug, Fail, Serialize, Deserialize)]
pub enum FollowingStoreError {
    #[fail(display = "{}", _0)]
    Read(ErrJson),
    #[fail(display = "{}", _0)]
    Write(ErrJson),
}

impl ResponseError for FollowingStoreError {
    fn error_response(&self) -> HttpResponse {
       match self {
            FollowingStoreError::Read(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            }
            FollowingStoreError::Write(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            }
       }
    }
}
