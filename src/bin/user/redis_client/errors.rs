use actix_web::{
    http::StatusCode,
    error::ResponseError,
    HttpResponse,
};
use failure::Error;
use std::convert::From;


#[derive(Debug, Fail, Serialize)]
pub enum RedisActixError {
    #[fail(display = "Redis GET Error: {}", _0)]
    Get(String),
    #[fail(display = "Redis SET Error: {}", _0)]
    Set(String),
    #[fail(display = "Redis SETEX Error: {}", _0)]
    Setex(String),
    #[fail(display = "Redis DEL Error: {}", _0)]
    Del(String),
    #[fail(display = "{{\"redis_connection_error\": \"{}\"}}", _0)]
    Connection(String),
    #[fail(display = "{{\"redis_error\": \"{}\"}}", _0)]
    Other(String),
}

impl From<redis::RedisError> for RedisActixError {
    fn from(e: redis::RedisError) -> Self {
        RedisActixError::Other(e.to_string())
    }
}

impl ResponseError for RedisActixError {
    fn error_response(&self) -> HttpResponse {
       match self {
            RedisActixError::Connection(_s) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            },
            RedisActixError::Other(_s) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            },
            RedisActixError::Get(_s) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            },
            RedisActixError::Set(_s) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            },
            RedisActixError::Setex(_s) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            },
            RedisActixError::Del(_s) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            },
       }
    }
}