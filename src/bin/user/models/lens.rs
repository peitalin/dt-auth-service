use actix_web::{
    HttpResponse,
    http::StatusCode,
    error::ResponseError,
};

////////////////////////////////////////////////
/// Lens Trait: peek in Serde JSON queries and variables
////////////////////////////////////////////////

pub trait Lens {
    fn get_key(&self, key: &str) -> Result<serde_json::Value, LensError>;
    fn get_query(&self) -> Result<String, LensError>;
    fn get_variables(&self) -> Result<serde_json::Value, LensError>;
}

#[derive(Debug, Fail)]
pub enum LensError {
    #[fail(display = "Error reading Lens. Invalid variable: {}", _0)]
    KeyError(String),
    #[fail(display = "{{\"status\": \"{}\"}}", _0)]
    EmailKeyError(String),
}

impl ResponseError for LensError {
    fn error_response(&self) -> HttpResponse {
        match self {
            LensError::KeyError(_s) => {
                HttpResponse::new(StatusCode::BAD_REQUEST)
            }
            LensError::EmailKeyError(_s) => {
                HttpResponse::new(StatusCode::BAD_REQUEST)
            }
        }
    }
}
