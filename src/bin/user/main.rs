#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![recursion_limit = "128"]

///////////////////////
//// External Imports
///////////////////////

// Actix Actors
extern crate actix;
extern crate actix_rt;
extern crate actix_web;
extern crate actix_cors;
extern crate actix_identity;

extern crate dotenv;
#[macro_use] extern crate diesel;
#[macro_use] extern crate failure;
extern crate jsonwebtoken;

#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate redis;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;

extern crate validator;
#[macro_use] extern crate validator_derive;

#[macro_use]
extern crate lazy_static;

use actix::{Addr, Actor, SyncArbiter, Arbiter};
use actix_web::{
    http,
    http::{header, Method},
    web, web::Data,
    App, Error,
    HttpServer, HttpRequest, HttpResponse,
};
use actix_web::middleware::{Logger};
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::cookie::SameSite;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;
use std::cmp;

///////////////////////
//// CRATE MODULES
///////////////////////

// Root crate import. Exposes modules in ./src/lib.rs
extern crate dt;
use dt::db::{
    create_postgres_pool,
};
use dt::utils::{
    // load_ssl_keys,
    init_logging,
};

#[macro_use] mod macros;
mod auth;
mod db;
mod email;
mod endpoints;
mod models;
mod notify_client;
mod redis_client;
mod rest;
mod rpc;
mod bug_reporting;

use db::{
    DatabaseActor,
};
use redis_client::{
    start_redis_server,
    start_redis_client,
    RedisActor,
};
use notify_client::{
    NotifyActor
};
use rest::{
    handle_404,
    login_handler,
    logout_handler,
    // read user profile
    get_profile_handler,
    get_users_by_ids,
    // user profile changes
    change_password_handler,
    delete_profile_handler,
    update_profile_handler,
    // Auth ID cookie
    get_id_from_set_cookie,
    // public user queries
    get_user_handler,
    get_user_by_email_handler,
    create_user_handler,
    // password reset by email
    send_reset_password_email_handler,
    reset_password_handler,
    // User suspension
    suspend_user_handler,
    unsuspend_user_handler,
    check_password_handler,
};

//// Constants
const PORT: i32 = 8082;
const IP: &str = "0.0.0.0";
const DEFAULT_MAX_DB_CONNECTIONS: u32 = 4;
const NUM_ACTOR_THREADS: usize = 2;

/////////////////////////////////////////////////
//// The entry point for the Login Service
/////////////////////////////////////////////////

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    init_logging("user", "debug");
    start_redis_server().ok();

    //// DB Pool Constants
    let max_db_connections = match std::env::var("MAX_DB_CONNECTIONS") {
        Ok(s) => match s.parse::<u32>() {
            Ok(v) => v,
            Err(_e) => DEFAULT_MAX_DB_CONNECTIONS
        },
        Err(_e) => DEFAULT_MAX_DB_CONNECTIONS
    };
    let max_connections_per_pool = cmp::max(1, max_db_connections / NUM_ACTOR_THREADS as u32);
    // `DatabaseActor` manages and executes queries on the schema
    let database_actor: Addr<DatabaseActor> = SyncArbiter::start(
        NUM_ACTOR_THREADS,
        move || DatabaseActor::new(
            create_postgres_pool("DATABASE_URL", max_connections_per_pool),
        )
    );
    // `SyncArbiter` creates actors in separate threads,
    // for when actors require blocking calls
    // debug!("create_jwt_secret...");
    let (secret, domain) = auth::create_jwt_secret();
    // debug!("create_jwt_secret...{} {}", &secret, &domain);

    // Start the http server
    HttpServer::new(move || {
        // Start the actors, set AppState
        App::new()
        .app_data(AppState {
            database_actor: database_actor.clone(),
            http_client: AppState::create_client(),
            redis_actor: RedisActor::new().start(),
            notify_actor: NotifyActor::new().start(),
        })
        // Enable middlewares
        .wrap(Logger::default())
        // Accept credentialed (cookie) requests
        .wrap(Cors::new().supports_credentials().finish())
        // Enable JWT cookies
        .wrap(IdentityService::new(
            CookieIdentityPolicy::new(secret.as_bytes())
            .name("degen-auth")
            .path("/")
            .domain(domain.as_str())
            .same_site(SameSite::Lax)
            .secure(false) // only true if https
        ))
        /////////// Auth Routes ///////////////////////////
        // require everything under '/auth' to require auth
        ///////////////////////////////////////////////////
        .service(web::scope("/auth")
            // Manage user profile
            .service(web::resource("/profile/get")
                .route(web::get().to(get_profile_handler)))
            .service(web::resource("/profile/update")
                .route(web::post().to(update_profile_handler)))
            .service(web::resource("/profile/changePassword")
                .route(web::post().to(change_password_handler)))
            .service(web::resource("/profile/delete")
                .route(web::post().to(delete_profile_handler)))
            // Auth ID decryption, for gateway
            .service(web::resource("/id")
                .route(web::get().to(get_id_from_set_cookie)))
            // Admin only
            .service(web::resource("/profile/suspendUser")
                .route(web::get().to(suspend_user_handler)))
            .service(web::resource("/profile/unsuspendUser")
                .route(web::get().to(unsuspend_user_handler)))
        )
        /////////////////////////////////////
        ////////// Non Auth Routes //////////
        /////////////////////////////////////
        .service(web::resource("/user/get")
            .route(web::get().to(get_user_handler))
        )
        .service(web::resource("/user/get/by/email")
            .route(web::get().to(get_user_by_email_handler))
        )
        .service(web::resource("/user/create")
            .route(web::post().to(create_user_handler))
        )
        .service(web::resource("/users/read/many")
            .route(web::post().to(get_users_by_ids))
        )
        .service(web::resource("/login")
            .route(web::post().to(login_handler))
        )
        .service(web::resource("/logout")
            .route(web::delete().to(logout_handler))
        )
        .service(web::resource("/check/password")
            .route(web::post().to(check_password_handler))
        )
        //// Password Reset
        .service(web::scope("/forgot")
            // 1. Request password reset email
            .service(web::resource("/1/sendResetPasswordEmail")
                .route(web::post().to(send_reset_password_email_handler)))
            // 2. Password reset form posts to this endpoint
            .service(web::resource("/2/resetPassword")
                .route(web::post().to(reset_password_handler)))
        )
        // Test routes
        .service(web::resource("/test")
            .route(web::get().to(crate::rest::test_handler)))
        .service(web::resource("/test/rpc")
            .route(web::get().to(crate::rpc::rpc_test_handler)))
        .service(web::resource("/_health")
            .route(web::get().to(rest::health::retrieve_health_status)))
        // 404 Routes
        .default_service(web::route().to(crate::rest::handle_404))
    })
    .bind(format!("{}:{}", IP, PORT))
    // .bind_ssl(&ip, _ssl_builder)
    .expect(&format!("Cannot bind to {}:{}, address in use or invalid", IP, PORT))
    .run()
    .await
}

//// Actors
struct AppState {
    database_actor: Addr<DatabaseActor>,
    pub http_client: actix_web::client::Client,
    pub redis_actor: Addr<RedisActor>,
    pub notify_actor: Addr<NotifyActor>,
}

impl AppState {

    pub fn from<'a>(req: &'a HttpRequest) -> &'a AppState {
        req.app_data::<AppState>().unwrap()
    }

    pub fn databaseActor(req: &HttpRequest) -> &Addr<DatabaseActor> {
        &req.app_data::<AppState>().expect("AppState error")
            .database_actor
    }

    pub fn redisActor(req: &HttpRequest) -> &Addr<RedisActor> {
        &req.app_data::<AppState>().expect("AppState error")
            .redis_actor
    }

    pub fn notifyActor(req: &HttpRequest) -> &Addr<NotifyActor> {
        &req.app_data::<AppState>().expect("AppState error")
            .notify_actor
    }

    pub fn httpClient(req: &HttpRequest) -> &actix_web::client::Client {
        &req.app_data::<AppState>().expect("AppState error")
            .http_client
    }

    pub fn create_client() -> actix_web::client::Client {
        actix_web::client::ClientBuilder::new()
            .header("Content-Type", "application/json")
            .header("Accept-Encoding", "*")
            .finish()
    }
}
