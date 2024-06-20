use actix_web::{
    http::StatusCode,
    error::ResponseError,
    HttpResponse,
};
use failure::Error;
use std::convert::From;
use crate::models::errors::ErrJson;


#[derive(Debug, Fail, Serialize)]
pub enum NotifyActixError {
    #[fail(display = "{}", _0)]
    UserCreated(ErrJson),
    #[fail(display = "{}", _0)]
    WelcomeEmail(ErrJson),
    #[fail(display = "{}", _0)]
    PasswordResetEmail(ErrJson),
}

impl ResponseError for NotifyActixError {
    fn error_response(&self) -> HttpResponse {
       match self {
            NotifyActixError::UserCreated(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            NotifyActixError::WelcomeEmail(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
            NotifyActixError::PasswordResetEmail(ejson) => {
                warn!("{}: {}", ejson.file, ejson.message);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "file": ejson.file, "message": ejson.message }))
            },
       }
    }
}
