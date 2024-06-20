
use actix_web::{
    web, web::Data, web::Query, web::Json,
    HttpRequest, HttpResponse,
    Responder,
    Error,
};
use actix_identity::{Identity};
use crate::auth::{
    AuthInfo,
};

use crate::db::{
    loginUser,
    getUser,
    checkPasswordForUserId,
};
use crate::db::{
    GetPool, GetPoolError
};
use crate::redis_client::{
    RedisCommand, Setex,
};
use crate::auth::{
    CheckJwt, CheckJwtError,
    decode_token,
};
use crate::models::auth::{
    LoginEmail,
    LoginForm,
    UserRole,
};
use crate::models::{
    User,
    UserPublic,
    LoginError,
    ErrJson,
    bad_request,
};
use crate::AppState;

/// 1. Login with JWT.
/// Requires password and email to login, then creates a JsonWebToken
/// for the client to use in their request headers so they don't need
/// to re-login every time. This JWT expires in 30days.
/// JWT authentication is only for users to read profile info,
/// and non-critical updates.
/// 2. Delete Profile and password change require password login to re-authenticate.

////////////////////////////
//// REST API Login Handlers
////////////////////////////

// GET /login
pub async fn login_handler(
    req: HttpRequest,
    id: Identity,
    data: Json<LoginForm>
) -> Result<HttpResponse, Error> {

    let login = data.into_inner();

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    // requires password
    let user: User = loginUser(
        &conn,
        login.email,
        login.password
    ).map_err(Error::from)?;

    if user.is_suspended {
        let _ = destroy_and_blacklist_jwt(req, id);
        return Err(
            LoginError::Suspended(ErrJson::new("User is suspended"))
        ).map_err(Error::from)
    }

    if user.is_deleted {
        let _ = destroy_and_blacklist_jwt(req, id);
        return Err(
            LoginError::Suspended(ErrJson::new("User is deleted"))
        ).map_err(Error::from)
    }

    let jwt = crate::auth::create_token(
        user.email.clone(),
        user.id.clone(),
        user.user_role.clone(),
    ).map_err(Error::from)?;

    debug!("login created jwt: {:?}", &jwt);
    // Set JWT as HttpOnly cookie to pass to the client
    id.remember(jwt.clone());

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(json!({
            "user": user,
            "jwt": jwt,
        })
    ))
}

// DELETE /logout
pub fn logout_handler(
    req: HttpRequest,
    id: Identity
) -> HttpResponse {

    let _ = destroy_and_blacklist_jwt(req, id);

    HttpResponse::Ok().json(json!({
        "status": "logged out successfully.",
    }))
}

pub fn destroy_and_blacklist_jwt(
    req: HttpRequest,
    id: Identity
) -> () {
    if let Some(jwt) = id.identity() {
        // Revoke JWT in redis
        AppState::from(&req).redis_actor
        .do_send(RedisCommand::Setex(
            Setex {
                key: jwt,
                ttl: 86_400 * 30, // JWT lifetimes are 1 day, do for 30 days
                value: String::from("REVOKED")
            }
        ));
    }
    // remove JWT as HttpOnly session cookie
    id.forget();
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordCheckBody {
    pub password: String,
}
// POST /check/password
pub async fn check_password_handler(
    req: HttpRequest,
    id: Identity,
    json: Json<PasswordCheckBody>
) -> Result<HttpResponse, Error> {

    let json = json.into_inner();

    let jwt = id.identity()
        .ok_or(Error::from(noJwtError!()))?;

    let authInfo: AuthInfo = decode_token(&jwt)
        .map_err(Error::from)?;

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    // requires password
    let _user: User = checkPasswordForUserId(&conn,
        authInfo.user_id,
        json.password,
    ).map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(json!({
            "password_matches": true,
        })
    ))
}