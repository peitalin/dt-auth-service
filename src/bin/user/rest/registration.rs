
use actix::{Addr, SyncArbiter, Arbiter};
use actix_web::{
    http,
    http::{header, Method},
    web, web::Data, web::Query, web::Json,
    error,
    Error,
    HttpRequest, HttpResponse, HttpMessage,
    Responder,
};
use actix_identity::{Identity};

use redis::RedisResult;

use crate::email::EmailVerifyError;
use crate::AppState;
use crate::db::{
    createUser,
    setEmailVerified,
};
use crate::db::{
    GetPool, GetPoolError,
};
use crate::models::auth::{
    CreateUserForm,
};
use crate::models::{
    User,
    LoginError,
};
use crate::models::errors::{
    bad_request,
    ErrJson,
};
use crate::redis_client::{
    RedisCommand, Setex,
};
use crate::rpc::{
    rpc_create_stripe_customer,
    rpc_notify_user_created
};
use crate::notify_client::{
    NotifyMessage
};


// POST /user/create
pub async fn create_user_handler(
    req: HttpRequest,
    json: Json<CreateUserForm>,
    id: Identity,
) -> Result<HttpResponse, Error> {
    dotenv::dotenv().ok();
    match std::env::var("DISABLE_STRIPE_SENDGRID").ok() {
        Some(s) => {
            if s.to_lowercase() == "yes" || s.to_lowercase() == "true" {
                create_user_handler_dev(req, json, id).await
            } else {
                create_user_handler_prod(req, json, id).await
            }
        },
        None => create_user_handler_prod(req, json, id).await
    }
}


// POST /user/create
// NO SENDGRID
pub async fn create_user_handler_dev(
    req: HttpRequest,
    json: Json<CreateUserForm>,
    id: Identity,
) -> Result<HttpResponse, Error> {

    // Params
    debug!("Incoming request: {:?}", req);
    debug!("Incoming body: {:?}", json);
    let userForm = json.into_inner();

    // 1. Create a user profile in DB
    let conn = AppState::databaseActor(&req)
        .send(GetPool::Postgres)
        .await??;

    // let (
    //     conn,
    //     stripe_customer_response
    // ) = match futures::join!(pool_future, stripe_future) {
    //     (r1, r2) => (r1??, r2?)
    // };

    let user = createUser(
        &conn,
        userForm.email,
        userForm.password,
        userForm.first_name,
        userForm.last_name,
    )?;

    debug!("new user created in db: {:?}", &user);

    let jwt = crate::auth::create_token(
        user.email.clone(),
        user.id.clone(),
        user.user_role.clone(),
    )?;

    // Set JWT as HttpOnly cookie to pass to the client
    id.remember(jwt);

    // // tell notify service to send welcome email
    // let sendgrid_response = AppState::notifyActor(&req)
    //                     .send(NotifyMessage::SendWelcomeEmail(user.id.clone()))
    //                     .await?;

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(json!({
            "user": user,
            // "sendgridResponse": sendgrid_response,
            "sendgridResponse": json!({
                "verified": {
                    "id": "NA",
                    "email": "NA",
                    "username": "NA",
                    "expiresAt": "NA",
                },
                "status": {
                    "message": "NA"
                }
            })
        })
    ))
}

// POST /user/create
// No JWT required
pub async fn create_user_handler_prod(
    req: HttpRequest,
    json: Json<CreateUserForm>,
    id: Identity,
) -> Result<HttpResponse, Error> {

    debug!("Incoming request: {:?}", req);

    // Params
    let userForm = json.into_inner();

    // 1. Get DB thread pool
    let conn = AppState::databaseActor(&req)
        .send(GetPool::Postgres)
        .await??;

    // 2. Set verify_email email in redis cache
    // let verify_email = VerifyEmail::from(&userForm.clone());
    // let _redis_future = AppState::redisActor(&req)
    //                     .do_send(RedisCommand::Setex(
    //                         Setex {
    //                             key: verify_email.email.clone(),
    //                             ttl: 172_800, // 48hrs
    //                             value: verify_email.email.clone(),
    //                         }
    //                     ));

    // // await all 2 futures to return concurrently, then send response
    // let (
    //     conn,
    //     stripe_customer_response,
    // ) = match futures::join!(pool_future, stripe_future) {
    //     (f1, f2) => (f1??, f2?) // unwrap Results
    // };

    let user = createUser(
        &conn,
        userForm.email,
        userForm.password,
        userForm.first_name,
        userForm.last_name,
    )?;
    debug!("new user created in db: {:?}", &user);

    let jwt = crate::auth::create_token(
        user.email.clone(),
        user.id.clone(),
        user.user_role.clone(),
    )?;

    // Set JWT as HttpOnly cookie to pass to the client
    id.remember(jwt);

    // Try tell the notify service about the new user
    // don't wait for it to return with do_send
    // AppState::notifyActor(&req)
    //     .do_send(NotifyMessage::UserCreated(user.id.clone()));

    // // tell notify service to send welcome email
    // let sendgrid_response = AppState::notifyActor(&req)
    //                     .send(NotifyMessage::SendWelcomeEmail(user.id.clone()))
    //                     .await?;

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(json!({
            "user": user,
            // "sendgridResponse": sendgrid_response,
            "sendgridResponse": json!({
                "verified": {
                    "id": "NA",
                    "email": "NA",
                    "username": "NA",
                    "expiresAt": "NA",
                },
                "status": {
                    "message": "NA"
                }
            })
        })
    ))
}

