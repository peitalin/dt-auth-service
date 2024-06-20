
/// STEPS
/// 1. Send VerifyEmail: send after "createUser" has saved a user to DB
/// 2. User checks email, clicks VerifyEmail link, which sends to our endpoint.
/// 3. Handle VerifyEmail:
///     if the request contains the correct GUID and before expiry, mark the user
///     account in the DB as "email_verified"

use actix::Addr;
use actix_web::{
    Error,
    web,
    HttpResponse, HttpRequest
};
use chrono::{Local, Utc, Duration};
use diesel::PgConnection;
use diesel::prelude::*;
use std::convert::From;
use uuid::Uuid;

////// Internal Modules //////

use dt::utils::from_datetimestr_to_naivedatetime;
use dt::utils::{
    init_logging,
};

use crate::db::{
    DatabaseActor,
};
use crate::endpoints::Endpoint;
use crate::email::{
    PasswordResetError,
    SendgridStatus,
};
use crate::models::auth::RequestResetPasswordForm;
use crate::models::{User, ErrJson};
use crate::{AppState};
use crate::email::domain::DomainVars;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetResponse {
    reset_id: String,
    email_sent_to: String,
    status: SendgridStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordReset {
    pub reset_id: String,
    pub email: String,
    pub new_password: Option<String>,
    #[serde(deserialize_with = "from_datetimestr_to_naivedatetime")]
    pub expires_at: chrono::NaiveDateTime
}
// Note on adding new fields: must also add to <password-reset-form.html>
// for Askama. Otherwise parse error when reset_password_handler() tries
// to deserialize PasswordReset with no new field.

impl PasswordReset {
    pub fn updated_password(&mut self, new_password: String) {
        self.new_password = Some(new_password);
    }
}

impl From<&RequestResetPasswordForm> for PasswordReset {
    fn from<'a>(reset: &'a RequestResetPasswordForm) -> Self {
        PasswordReset {
            reset_id: Uuid::new_v4().to_string(),
            email: reset.email.clone(),
            new_password: None,
            expires_at: Local::now().naive_utc() + Duration::hours(48),
        }
    }
}

//////////////////////////////////////////////////
///// 2. Password Reset response to DatabaseActor
//////////////////////////////////////////////////

// impl PasswordReset as a Message for DatabaseActor Actor
impl actix::Message for PasswordReset {
    type Result = Result<PasswordReset, PasswordResetError>;
}

// Handle PasswordReset Message
impl actix::Handler<PasswordReset> for DatabaseActor {
    type Result = Result<PasswordReset, PasswordResetError>;

    fn handle(&mut self, msg: PasswordReset, _ctx: &mut Self::Context) -> Self::Result {

        use dt::db::schema::users;

        if Utc::now().timestamp() > msg.expires_at.timestamp() {
            debug!("password reset expired! {:?}", &msg.expires_at);
            return Err(
                PasswordResetError::ResetExpired(errJson!("Password Reset Expired"))
            )
        }

        // Check that the msg.email matches our registration email
        let email = self.get_from_cache(&msg.reset_id)
            .map_err(|_e| {
                debug!("reset_id invalid! {:?}", &msg.reset_id);
                debug!("for email! {:?}", &msg.email);
                PasswordResetError::VerificationError(errJson!(&format!(
                    "Email: {:?} is not the one which requested password reset!",
                    &msg.email
                )))
            })?;

        debug!("resetting password for user with email: {:?}", email);

        let conn = self.pool.get()
            .map_err(|e| PasswordResetError::ConnectionPoolError(errJson!(e)))?;

        // Get old user profile, need the user id to generate new credential
        let user: User = crate::db::getUser(&conn, Some(&email), None)
            .map_err(|e| PasswordResetError::DbError(errJson!(e)))?;

        debug!("success reading user profile: {:?}", user);

        // create new credential = hash(user_id(salt) + password)
        let new_password_hash = user.generate_new_password_hash(
            &msg.new_password.clone().expect("new_password to exist"),
        );

        let updated_user = diesel::update(
                users::table.filter(users::email.eq(&email))
            )
            .set(users::password_hash.eq(new_password_hash))
            .get_result::<User>(&conn)
            .map_err(|e| PasswordResetError::DbError(errJson!(e)))?;

        debug!("Successfully updated user password in DB: {:?}", updated_user);

        // try delete and invalidate reset_id from redis cache
        let redis_conn = self.get_redis_client()
            .map_err(|e| PasswordResetError::Other(errJson!(e)));

        match redis_conn {
            Err(_e) => {},
            Ok(mut rconn) => {
                let _redis_res: redis::RedisResult<String> = redis::cmd("DEL")
                    .arg(&msg.reset_id)
                    .query(&mut rconn);
            }
        };

        Ok(msg)
    }
}
