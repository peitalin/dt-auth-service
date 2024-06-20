
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::models::{
    UserId,
    LoginError,
    User,
    UpdateUserProfile,
    ErrJson,
    UserPublic,
};

use super::users_raw::{
    login,
    check_password,
    get_user_profile_by_email,
    get_user_profile_by_id,
    get_user_profiles_by_ids,
    delete_user_profile,
    insert_user_profile,
    update_user_profile,
    set_new_password,
    set_suspended,
    set_email_verified,
};

//////////////////////////////////////////
///////// Main DB Login Queries //////////
//////////////////////////////////////////

pub fn loginUser(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    email: String,
    password: String,
) -> Result<User, LoginError> {
    login(conn, email, password)
}

pub fn checkPasswordForUserId(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    userId: String,
    password: String,
) -> Result<User, LoginError> {
    check_password(conn, userId, password)
}


pub fn getUser(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    email: Option<&str>,
    id: Option<&str>,
) -> Result<User, LoginError> {

    match (email, id) {
        (Some(email), None) => get_user_profile_by_email(conn, email),
        (None, Some(id)) => get_user_profile_by_id(conn, id),
        (Some(_), Some(_)) => Err(
            LoginError::BadRequest(errJson!("Can't use both email and id"))
        ),
        (None, None) => Err(LoginError::BadRequest(errJson!("No arguments supplied"))),
    }
}


pub fn getUsersByIds(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    user_ids: Vec<String>,
) -> Result<Vec<User>, LoginError> {

    get_user_profiles_by_ids(conn, &user_ids)
}

pub fn deleteUser(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    user_id: &str,
    password: String,
) -> Result<String, LoginError> {
    delete_user_profile(conn, user_id, password)
}

pub fn createUser(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    email: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
) -> Result<User, LoginError> {

    let user = User::new(
        email,
        password,
        first_name,
        last_name,
    );
    // Validate email is acceptable
    // .validate() is from the `#[derive(Validate)]` trait.
    match user.validate() {
        Err(e) => Err(LoginError::EmailInvalid(errJson!(e))),
        Ok(_) => user.store_user_profile(conn),
    }
}

pub fn updateUser(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    user_id: UserId,
    new_email: Option<String>,
    new_first_name: Option<String>,
    new_last_name: Option<String>,
) -> Result<User, LoginError> {

    debug!("retrieving user: {}", user_id);
    let user = get_user_profile_by_id(&conn, &user_id)?;
    let mut update_profile = UpdateUserProfile::from(&user);

    if let Some(e) = new_email {
        update_profile.update_email(e);
    }
    if let Some(f) = new_first_name {
        update_profile.update_first_name(f);
    }
    if let Some(l) = new_last_name {
        update_profile.update_last_name(l);
    }

    match update_profile.validate() {
        Err(e) => Err(LoginError::EmailInvalid(errJson!(e))),
        Ok(_) => update_user_profile(&conn, update_profile)
    }
    // Update UserProfile in database
}

pub fn setNewPassword(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    user_id: &str,
    current_password: &str,
    new_password: &str,
) -> Result<User, LoginError> {

    let mut user = get_user_profile_by_id(&conn, user_id)?;
    // credential = hash(salt-id + password)
    match user.verify_credentials(&conn, current_password.to_string()) {
        Err(e) => Err(e),
        Ok(user) => {
            let new_password_hash = user.generate_new_password_hash(new_password);
            set_new_password(conn, &user.id, &new_password_hash)
        }
    }
}


pub fn setEmailVerified(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    email: String,
    new_email_verified: bool,
) -> Result<User, LoginError> {
    set_email_verified(conn, email, new_email_verified)
}

pub fn setSuspended(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    user_id: UserId,
    new_is_suspended: bool,
) -> Result<User, LoginError> {
    set_suspended(conn, user_id, new_is_suspended)
}

