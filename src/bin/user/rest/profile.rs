
use actix_web::{
    web,
    web::Data,
    web::Query,
    web::Json,
    Error,
    HttpRequest,
    HttpResponse,
};
use actix_identity::{Identity};

use crate::db::{
    loginUser,
    getUser,
    updateUser,
    deleteUser,
    setEmailVerified,
    setSuspended,
    setNewPassword,
    getUsersByIds,
};
use crate::db::{
    GetPool, GetPoolError,
};
use crate::redis_client::{
    RedisCommand, Setex,
};
use crate::auth::{
    AuthInfo,
    // CheckJwt Actor Message
    CheckJwt, CheckJwtError,
    decode_token,
};
use crate::models::auth::{
    LoginEmail,
    LoginForm,
    QueryUserId,
    QueryUserEmail,
    DeleteUserForm,
    UserRole,
};
use crate::models::{
    User,
    UserPublic,
    UpdateUserProfile,
    LoginError,
    ErrJson,
    AuthError,
};
use crate::AppState;
use crate::rpc;
use crate::rest::destroy_and_blacklist_jwt;




// GET /user/get?user_id=user_id
pub async fn get_user_handler(
    req: HttpRequest,
    query: Query<QueryUserId>,
) -> Result<HttpResponse, Error> {

    // Retrieves public user profiles by id
    let user_id = query.user_id.clone();

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    let user = getUser(&conn, None, Some(&user_id))
        .map_err(Error::from);

    // filter public fields with UserPublic
    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(user?))

}


// GET /user/get/by/email?user_email=email@domain
pub async fn get_user_by_email_handler(
    req: HttpRequest,
    query: Query<QueryUserEmail>,
) -> Result<HttpResponse, Error> {

    // Retrieves user profiles by email
    let user_email = query.user_email.clone();

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    let user = getUser(&conn, Some(&user_email), None)
        .map_err(Error::from);

    // filter public fields with UserPublic
    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(user?))

}


// GET /auth/profile/get
// JWT required for this route
pub async fn get_profile_handler(
    req: HttpRequest,
    id: Identity,
) -> Result<HttpResponse, Error> {

    debug!("Incoming request: {:?}", req);
    // Notes: async fetches DBPool and redis JWT check at the same time.
    // If JWT is not in blacklist, proceed with getUser DB call.
    let jwt = id.identity()
        .ok_or(Error::from(noJwtError!()))?;

    let authInfo: AuthInfo = decode_token(&jwt)
        .map_err(Error::from)?;

    // Retreive DB pool thread asynchronously
    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;


    // Check if JWT exists in blacklist, return early if so.
    let res = AppState::databaseActor(&req)
                .send(CheckJwt(jwt))
                .await?;

    match res {
        Err(e) => Err(Error::from(e)),
        Ok(_) => {

            let user = match getUser(&conn, None, Some(&authInfo.user_id)) {
                Err(e) => return Err(Error::from(e)),
                Ok(u) => u,
            };

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

            Ok(HttpResponse::Ok()
            .content_type("application_json")
            .json(
                // Return HttpResponse with user data
                getUser(&conn, None, Some(&authInfo.user_id))?
            ))
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersByIdsBody {
    pub user_ids: Vec<String>,
}

// POST /users/read/many
pub async fn get_users_by_ids(
    req: HttpRequest,
    json: Json<UsersByIdsBody>,
    id: Identity,
) -> Result<HttpResponse, Error> {

    // Retrieves public user profiles by storeIds
    let body = json.into_inner();

    let auth_info = match id.identity() {
        None => None,
        Some(jwt) => match decode_token::<AuthInfo>(&jwt) {
            Err(e) => return Err(Error::from(e)),
            Ok(auth_info) => match auth_info.user_role {
                UserRole::PLATFORM_ADMIN => Some(auth_info),
                _ => None
            }
        },
    };

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    match auth_info {
        Some(a) => match a.user_role {

            UserRole::PLATFORM_ADMIN => {
                let users: Vec<User> = getUsersByIds(&conn, body.user_ids)
                    .map_err(Error::from)?;

                Ok(HttpResponse::Ok()
                .content_type("application_json")
                .json(json!({
                    "userRole": a.user_role,
                    "userType": "UserPrivate",
                    "users": users,
                })))
            },
            _ => {
                let users: Vec<UserPublic> = getUsersByIds(&conn, body.user_ids)
                    .map_err(Error::from)?
                    .into_iter()
                    .map(UserPublic::from)
                    .collect::<Vec<UserPublic>>();

                Ok(HttpResponse::Ok()
                .content_type("application_json")
                .json(json!({
                    "userRole": a.user_role,
                    "userType": "UserPublic",
                    "users": users,
                })))
            }
        },
        None => {
            let users: Vec<UserPublic> = getUsersByIds(&conn, body.user_ids)
                .map_err(Error::from)?
                .into_iter()
                .map(UserPublic::from)
                .collect::<Vec<UserPublic>>();

            Ok(HttpResponse::Ok()
            .content_type("application_json")
            .json(json!({
                "userRole": "",
                "userType": "UserPublic",
                "users": users,
            })))
        }
    }
}



// GET /auth/id
// JWT required for this route
pub async fn get_id_from_set_cookie(
    req: HttpRequest,
    id: Identity,
) -> Result<HttpResponse, Error> {
    // debug!("Incoming request: {:?}", req);
    let jwt = match id.identity() {
        None => return Err(Error::from(noJwtError!())),
        Some(jwt) => jwt,
    };
    let auth_info: AuthInfo = decode_token(&jwt)
        .map_err(Error::from)?;
    // Check if JWT exists in blacklist, return early with error if so.
    let _check_jwt = AppState::databaseActor(&req)
                .send(CheckJwt(jwt))
                .await?;

    // let conn = AppState::databaseActor(&req)
    //             .send(GetPool::Postgres)
    //             .await??;

    // // lookup user cartId and storeId, deprecate storeId and cartId in cookie
    // let user: User = getUser(&conn, None, Some(&auth_info.user_id))
    //     .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(
            // AuthInfo::from(user)
            auth_info
        )) // authInfo serializes as camelCase
}


// POST /auth/profile/update
pub async fn update_profile_handler(
    req: HttpRequest,
    data: Json<UpdateUserProfile>,
    id: Identity
) -> Result<HttpResponse, Error> {

    // debug!("Incoming request: {:?}", req);
    let profile = data.into_inner();
    info!("profile: {:?}", profile);

    let authInfo: AuthInfo = match id.identity() {
        None => return Err(Error::from(noJwtError!())),
        Some(jwt) => match decode_token(&jwt) {
            Err(e) => return Err(Error::from(e)),
            Ok(auth_info) => auth_info,
        },
    };
    info!("authInfo: {:?}", authInfo);

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    let updated_user: User = updateUser(
        &conn,
        authInfo.user_id,
        profile.email,
        profile.first_name,
        profile.last_name,
    ).map_err(Error::from)?;
    debug!("updated_user: {:?}", updated_user);

    let jwt = crate::auth::create_token(
        updated_user.email.clone(),
        updated_user.id.clone(),
        updated_user.user_role.clone(),
    ).map_err(Error::from)?;

    // debug!("updated jwt: {:?}", &jwt);
    // Set JWT as HttpOnly cookie to pass to the client
    id.remember(jwt);

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(updated_user))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePassword {
    current_password: String,
    new_password: String,
}

// POST /auth/profile/changePassword
pub async fn change_password_handler(
    req: HttpRequest,
    data: Json<ChangePassword>,
    id: Identity
) -> Result<HttpResponse, Error> {

    // debug!("Incoming request: {:?}", req);
    let password_reset = data.into_inner();
    // debug!("password_reset request: {:?}", password_reset);

    let authInfo: AuthInfo = match id.identity() {
        None => return Err(Error::from(noJwtError!())),
        Some(jwt) => match decode_token(&jwt) {
            Err(e) => return Err(Error::from(e)),
            Ok(auth_info) => auth_info,
        },
    };
    info!("authInfo: {:?}", authInfo);

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    let updated_user: User = setNewPassword(
        &conn,
        &authInfo.user_id,
        &password_reset.current_password,
        &password_reset.new_password,
    ).map_err(Error::from)?;
    // debug!("updated user password: {:?}", updated_user);

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(updated_user))
}


// POST /auth/profile/delete
pub async fn delete_profile_handler(
    req: HttpRequest,
    json: Json<DeleteUserForm>,
    id: Identity
) -> Result<HttpResponse, Error> {

    // debug!("Incoming request: {:?}", req);

    let password = json.into_inner().password;
    let authInfo: AuthInfo = match id.identity() {
        None => return Err(Error::from(noJwtError!())),
        Some(jwt) => match decode_token(&jwt) {
            Err(e) => return Err(Error::from(e)),
            Ok(auth_info) => auth_info,
        },
    };
    info!("authInfo: {:?}", authInfo);

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;


    let res = deleteUser(
        &conn,
        &authInfo.user_id,
        password,
    )?;

    // // Ask shopping service to delete things owned by this user
    // let user_delete_res = rpc::rpc_delete_user_shopping(
    //     AppState::httpClient(&req),
    //     &authInfo.user_id
    // ).await?;

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(res))
}


// POST /auth/profile/suspendUser?user_id=
pub async fn suspend_user_handler(
    req: HttpRequest,
    query: Query<QueryUserId>,
    id: Identity
) -> Result<HttpResponse, Error> {

    // debug!("Incoming request: {:?}", req);
    let user_id = query.user_id.clone();

    let authInfo: AuthInfo = match id.identity() {
        None => return Err(Error::from(noJwtError!())),
        Some(jwt) => match decode_token(&jwt) {
            Err(e) => return Err(Error::from(e)),
            Ok(auth_info) => auth_info,
        },
    };
    info!("authInfo: {:?}", authInfo);

    if authInfo.user_role != UserRole::PLATFORM_ADMIN {
        return Err(Error::from(
                LoginError::CredentialsError(
                    errJson!("Not an admin, can't suspend a user"))))
    }

    // send message to DB actor to retrieve pool.
    // then unwrap pool Future, and then Result (twice) to obtain connection.
    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    let user = setSuspended(&conn, user_id, true)
        .map_err(Error::from)?;

    debug!("user suspended: {:?}", user.email);

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(user))
}

// POST /auth/profile/unsuspendUser?user_id=
pub async fn unsuspend_user_handler(
    req: HttpRequest,
    query: Query<QueryUserId>,
    id: Identity
) -> Result<HttpResponse, Error> {

    // debug!("Incoming request: {:?}", req);
    let user_id = query.user_id.clone();

    let authInfo: AuthInfo = match id.identity() {
        None => return Err(Error::from(noJwtError!())),
        Some(jwt) => match decode_token(&jwt) {
            Err(e) => return Err(Error::from(e)),
            Ok(auth_info) => auth_info,
        },
    };
    info!("authInfo: {:?}", authInfo);

    if authInfo.user_role != UserRole::PLATFORM_ADMIN {
        return Err(Error::from(LoginError::CredentialsError(
                    errJson!("Not an admin, can't unsuspend a user"))))
    }

    let conn = AppState::databaseActor(&req)
                .send(GetPool::Postgres)
                .await??;

    let user = setSuspended(&conn, user_id, false)
        .map_err(Error::from)?;

    debug!("user suspended: {:?}", user.email);

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(user))
}

