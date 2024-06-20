
use actix::{Addr, SyncArbiter, Arbiter};
use actix_web::{
    http,
    http::{header, Method},
    web, web::Data, web::Query, web::Json,
    error, Error,
    HttpRequest, HttpResponse, HttpMessage,
    Responder,
};
use actix_identity::{Identity};

use chrono::{Local, Utc, Duration};
use redis::RedisResult;
use std::path::PathBuf;
use std::io::Read;

use crate::endpoints::Endpoint;
use crate::models::auth::{
    ResetPasswordForm,
    RequestResetPasswordForm,
};
use crate::models::{
    User,
    ErrJson,
    UpdateUserProfile,
    errors::LoginError,
};
use crate::email::password_reset::{
    PasswordReset,
    PasswordResetResponse,
};
use crate::email::PasswordResetError;
use crate::db::{
    GetPool,
    GetPoolError,
    loginUser,
    getUser,
};

use crate::redis_client::{
    RedisCommand, Setex,
};
use crate::notify_client::{
    NotifyMessage
};
use crate::email::SendgridStatus;
use crate::AppState;



// REST /forgot/1/sendResetPasswordEmail
// no password required, send email to reset pw
pub async fn send_reset_password_email_handler(
    req: HttpRequest,
    data: Json<RequestResetPasswordForm>
) -> Result<HttpResponse, Error> {

    let request_reset_pw = data.into_inner();
    debug!("body: {:?}", &request_reset_pw);
    let pw_reset = PasswordReset::from(&request_reset_pw);
    debug!("pw_reset args: {:?}", &pw_reset);

    // cache password request
    let _redis_res = AppState::redisActor(&req)
                .send(RedisCommand::Setex(
                    Setex {
                        key: pw_reset.reset_id.clone(),
                        ttl: 3600, // 1hr
                        value: pw_reset.email.clone(),
                    }
                ))
                .await?;


    // then send out a password reset email
    let res = AppState::notifyActor(&req)
                .send(NotifyMessage::SendPasswordResetEmail(
                    pw_reset.email.clone(),
                    pw_reset.reset_id.clone(),
                    pw_reset.expires_at.clone(),
                ))
                .await??;

    // debug!("pw_reset args: {:?}", &pw_reset);

    Ok(HttpResponse::Ok()
    .content_type("application_json")
    .json(
        json!({
            "reset_id": pw_reset.reset_id,
            "email_sent_to": pw_reset.email,
            "status": res
        })
    ))
}


// POST /forgot/2/resetPassword
pub async fn reset_password_handler(
    req: HttpRequest,
    json: web::Json<PasswordReset>,
) -> Result<HttpResponse, Error> {

    debug!("form: \n{:?}\n", &json);
    let pw_reset: PasswordReset = json.into_inner();

    // pw_reset: PasswordReset is a actix message that triggers
    // the DatabaseActor actor to generate a new password_hash
    // based on new password then save to DB.

    let res = AppState::databaseActor(&req)
                .send(pw_reset)
                .await?;

    debug!("pw_reset result: {:?}", &res);

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(res))
}


