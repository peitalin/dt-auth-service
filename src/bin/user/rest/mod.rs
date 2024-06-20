pub mod login;
pub mod forgot_password;
pub mod profile;
pub mod registration;
pub mod health;

pub use login::*;
pub use forgot_password::*;
pub use profile::*;
pub use registration::*;
pub use health::*;

///////////////////////////////////////

use crate::AppState;
use crate::models::errors::RpcError;
use crate::endpoints::Endpoint;

use actix_web::{HttpResponse, HttpRequest, Error};

pub fn test_handler(
    req: HttpRequest
) -> HttpResponse {

    debug!("req: {:?}", req);
    debug!("headers: {:?}", req.headers());
    debug!("uri: {:?}", req.uri());

    HttpResponse::Ok()
        .content_type("application_json")
        .json(json!({
            "status": "OK",
            "message": "Test response for dt-user service",
            "endpoint": Endpoint::Base("/test").as_url(),
        }))
}

pub fn handle_404(req: actix_web::HttpRequest) -> HttpResponse {
    HttpResponse::NotFound()
        .json(
            json!({
                "status": 404,
                "reason": "Endpoint not found.",
                "path": format!("{:?}", req.path()),
                "query_string": format!("{:?}", req.query_string()),
                "headers": format!("{:?}", req.headers()),
                "connection_info": format!("{:?}", req.connection_info()),
            })
        )
}

