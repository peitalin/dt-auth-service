use actix_web::{
  HttpRequest, HttpResponse,
  web, web::Query, web::Json,
  Error,
};
use crate::AppState;
use crate::db::{ GetPool };

// GET /_health
pub fn retrieve_health_status(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
      .content_type("application/json")
      .json(json!({
          "isHealthy": true
      }))
}
