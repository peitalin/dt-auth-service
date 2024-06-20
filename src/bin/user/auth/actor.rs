//// External Imports
use actix::{Actor, Handler, SyncContext, Message};
use actix_web::{
    http::StatusCode,
    error::ResponseError,
    HttpResponse,
};
use failure::Error;
use redis::{RedisResult, Commands}; // for .get() .set() methods
use std::sync::Arc;

//// Internal Imports
use crate::db::DatabaseActor;

/////////// Message Handlers for DatabaseActor Actor
////////// Check JWT Blacklist Message Handler ////////////
/// These are message types to send to DatabaseActor
/// to execute Redis Commands from other actors

use actix_identity::{Identity};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckJwt(pub String);

impl Message for CheckJwt {
    type Result = Result<String, CheckJwtError>;
}

impl Handler<CheckJwt> for DatabaseActor {
    type Result = Result<String, CheckJwtError>;

    fn handle(
        &mut self,
        msg: CheckJwt,
        _ctx: &mut SyncContext<Self>
    ) -> Result<String, CheckJwtError> {

        // Check if JWT exists in redis, if so
        // Return a Jwt revoked error
        let mut conn = self.redis_sync_client.clone().get_connection()?;
        let res: RedisResult<String> = redis::cmd("GET").arg(msg.0.clone()).query(&mut conn);
        match res {
            // Not in blacklist => Ok
            Err(_) => Ok(msg.0),
            // In blacklist => Err
            Ok(_) => Err(CheckJwtError::Revoked),
        }
    }
}

#[derive(Debug, Fail)]
pub enum CheckJwtError {
    #[fail(display = "{{\"status\":\"JWT revoked, login again\"}}")]
    Revoked,
    #[fail(display = "{{\"status\":\"JWT missing, please login\"}}")]
    Missing,
}

impl From<redis::RedisError> for CheckJwtError {
    fn from(_e: redis::RedisError) -> Self {
        CheckJwtError::Revoked
    }
}

impl ResponseError for CheckJwtError {
    fn error_response(&self) -> HttpResponse {
       match self {
            CheckJwtError::Revoked => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                .content_type("application_json")
                .json(json!({
                    "status": "REVOKED",
                    "message": "JWT revoked from logout, login again."
                }))
            },
            CheckJwtError::Missing => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                .content_type("application_json")
                .json(json!({
                    "status": "MISSING",
                    "message": "JWT missing from cookies, login again."
                }))
            },
       }
    }
}