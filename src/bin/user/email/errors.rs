use actix_web::{
    http::StatusCode,
    error::ResponseError,
    HttpResponse,
};
use failure::Error;
use crate::models::ErrJson;

#[derive(Debug, Fail, Serialize, Deserialize)]
pub enum EmailVerifyError {
    #[fail(display = "{}", _0)]
    VerificationError(ErrJson),
    #[fail(display = "{}", _0)]
    RegistrationExpired(ErrJson),
    #[fail(display = "{}", _0)]
    RegistrationNotFound(ErrJson),
    #[fail(display = "{}", _0)]
    ConnectionPoolError(ErrJson),
    #[fail(display = "{}", _0)]
    EmailSendFailure(ErrJson),
    #[fail(display = "{}", _0)]
    DeserializationError(ErrJson),
    #[fail(display = "{}", _0)]
    DbError(ErrJson),
}

impl ResponseError for EmailVerifyError {
    fn error_response(&self) -> HttpResponse {
       match self {
            EmailVerifyError::VerificationError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            EmailVerifyError::RegistrationExpired(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            EmailVerifyError::RegistrationNotFound(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            EmailVerifyError::ConnectionPoolError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            EmailVerifyError::EmailSendFailure(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            EmailVerifyError::DeserializationError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            EmailVerifyError::DbError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
       }
    }
}

impl From<&serde_json::Error> for EmailVerifyError {
    fn from<'a>(e: &'a serde_json::Error) -> Self {
        EmailVerifyError::DeserializationError(errJson!(e))
    }
}

impl From<serde_json::Error> for EmailVerifyError {
    fn from(e: serde_json::Error) -> Self {
        EmailVerifyError::DeserializationError(errJson!(e))
    }
}


#[derive(Debug, Fail, Serialize, Deserialize)]
pub enum PasswordResetError {
    #[fail(display = "{}", _0)]
    VerificationError(ErrJson),
    #[fail(display = "{}", _0)]
    ResetExpired(ErrJson),
    #[fail(display = "{}", _0)]
    EmailNotFound(ErrJson),
    #[fail(display = "{}", _0)]
    ConnectionPoolError(ErrJson),
    #[fail(display = "{}", _0)]
    EmailSendFailure(ErrJson),
    #[fail(display = "{}", _0)]
    DeserializationError(ErrJson),
    #[fail(display = "{}", _0)]
    DbError(ErrJson),
    #[fail(display = "{}", _0)]
    Other(ErrJson),
}

impl ResponseError for PasswordResetError {
    fn error_response(&self) -> HttpResponse {
       match self {
            PasswordResetError::VerificationError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            PasswordResetError::ResetExpired(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            PasswordResetError::EmailNotFound(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            PasswordResetError::ConnectionPoolError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            PasswordResetError::EmailSendFailure(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            PasswordResetError::DeserializationError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            PasswordResetError::DbError(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            PasswordResetError::Other(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
       }
    }
}

impl From<&serde_json::Error> for PasswordResetError {
    fn from<'a>(e: &'a serde_json::Error) -> Self {
        PasswordResetError::DeserializationError(errJson!(e))
    }
}

impl From<serde_json::Error> for PasswordResetError {
    fn from(e: serde_json::Error) -> Self {
        PasswordResetError::DeserializationError(errJson!(e))
    }
}

