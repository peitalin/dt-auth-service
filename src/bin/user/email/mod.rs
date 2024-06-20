pub mod domain;
pub mod password_reset;
pub mod errors;

use actix::{Addr};
use actix_web::{
    Error,
    web,
    HttpResponse,
    HttpRequest,
};
use uuid::Uuid;

pub use errors::EmailVerifyError;
pub use errors::PasswordResetError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendgridStatus {
    pub message: Option<String>
}
