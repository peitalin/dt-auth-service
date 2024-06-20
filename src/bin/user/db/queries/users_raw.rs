use crate::models::User;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
// from ./src/db
use dt::db;
use crate::models::{ LoginError, ErrJson, FollowingStoreError };
use crate::models::{
    UserId,
    UpdateUserProfile,
    UserPublic,
};
use crate::models::{
    ConnectionQuery,
    PageBasedConnectionQuery,
};

///////////////////////////////////
///  Raw queries direct to Database
///////////////////////////////////

pub fn get_user_profile_by_email(
    conn: &PgConnection,
    email: &str,
) -> Result<User, LoginError> {

    // login without credential verification, use JWT instead
    use db::schema::users;

    users::table
        .filter(users::email.eq(email))
        .get_result::<User>(conn)
        .map_err(|e| LoginError::NoUserError(errJson!(e)))
}

pub fn get_user_profile_by_id(
    conn: &PgConnection,
    id: &str,
) -> Result<User, LoginError> {

    // login without credential verification, use JWT instead
    use db::schema::users;

    users::table
        .filter(users::id.eq(id))
        .get_result::<User>(conn)
        .map_err(|e| LoginError::NoUserError(errJson!(e)))
}



pub fn get_user_profiles_by_ids(
    conn: &PgConnection,
    user_ids: &Vec<String>,
) -> Result<Vec<User>, LoginError> {

    use db::schema::users;

    users::table
        .filter(users::id.eq_any(user_ids))
        .load::<User>(conn)
        .map_err(|e| LoginError::NoUserError(errJson!(e)))
}


pub fn login(
    conn: &PgConnection,
    email: String,
    attempted_password: String,
) -> Result<User, LoginError> {

    use db::schema::users;

    let user = users::table
        .filter(users::email.eq(&email))
        .get_result::<User>(conn);

    match user {
        Err(e) => Err(LoginError::NoUserError(errJson!(e))),
        // Verify password hashed with salt yields correct credential
        Ok(mut u) => match u.verify_credentials(conn, attempted_password) {
            Err(e) => Err(LoginError::CredentialsError(errJson!(e))),
            Ok(auth_user) => Ok(auth_user),
        },
    }
}

pub fn check_password(
    conn: &PgConnection,
    user_id: String,
    attempted_password: String,
) -> Result<User, LoginError> {

    use db::schema::users;

    let user = users::table
        .filter(users::id.eq(user_id))
        .get_result::<User>(conn);

    match user {
        Err(e) => Err(LoginError::NoUserError(errJson!(e))),
        // Verify password hashed with salt yields correct credential
        Ok(mut u) => match u.verify_credentials(conn, attempted_password) {
            Err(e) => Err(LoginError::CredentialsError(errJson!(e))),
            Ok(auth_user) => Ok(auth_user),
        },
    }
}


pub fn delete_user_profile(
    conn: &PgConnection,
    user_id: &str,
    password: String,
) -> Result<String, LoginError> {

    // use db::schema::users::dsl::*;
    // Import `users` (table) from `users` module as `users`
    // Also imports `id` (and other columns from `users` table)

    use db::schema::users;
    let user_db = users::table
        .filter(users::id.eq(user_id))
        .get_result::<User>(conn);

    match user_db {
        Err(e) => Err(LoginError::NoUserError(errJson!(e))),
        // Verify password hashed with salt yields correct credential.
        Ok(mut user) => match user.verify_credentials(conn, password) {
            Err(e) => Err(LoginError::CredentialsError(errJson!(e))),
            Ok(auth_user) => {
                // clear the user profile from DB (soft delete)
                let deleted_user = diesel::update(users::table.filter(users::id.eq(&auth_user.id)))
                  .set((
                        users::email.eq(format!("deleted_{}", auth_user.id)),
                        users::first_name.eq(None as Option<String>),
                        users::last_name.eq(None as Option<String>),
                        users::user_role.eq(auth_user.user_role),
                        users::is_deleted.eq(true),
                  ))
                  .get_result::<User>(conn);

                match deleted_user {
                    Err(e) => Err(LoginError::NoUserError(errJson!(e))),
                    Ok(_return_code) => Ok(format!("Deleted user: {}", auth_user.email)),
                }
            },
        },
    }
}

pub fn insert_user_profile(conn: &PgConnection, user: &User) -> Result<User, Error> {
    use db::schema::users;
    // Import `users` (table) from `users` module as `users`
    // Also imports `id` (and other columns from `users` table)
    diesel::insert_into(users::table)
        .values(user)
        .get_result::<User>(conn)
}

pub fn update_user_profile(
    conn: &PgConnection,
    new: UpdateUserProfile,
) -> Result<User, LoginError> {
    // import table `users`
    use db::schema::users;

    let new_user = diesel::update(users::table.filter(users::id.eq(&new.id)))
        .set((
            users::email.eq(&new.email.unwrap()),
            // users::password_hash.eq(&new.password_hash.unwrap()),
            users::first_name.eq(&new.first_name),
            users::last_name.eq(&new.last_name),
            // users::email_verified.eq(&new.email_verified.unwrap()),
            // users::is_suspended.eq(&new.is_suspended),
            users::user_role.eq(&new.user_role),
        ))
        .get_result::<User>(conn);

    match new_user {
        Err(e) => Err(LoginError::NoUserError(errJson!(e))),
        Ok(user) => Ok(user),
    }
}



pub fn set_suspended(
    conn: &PgConnection,
    user_id: String,
    suspend: bool,
) -> Result<User, LoginError> {
    // import table `users`
    use db::schema::users;

    let new_user = diesel::update(users::table.filter(users::id.eq(&user_id)))
        .set(users::is_suspended.eq(&suspend))
        .get_result::<User>(conn);

    match new_user {
        Err(e) => Err(LoginError::NoUserError(errJson!(e))),
        Ok(user) => Ok(user),
    }
}

pub fn set_new_password(
    conn: &PgConnection,
    user_id: &str,
    new_password_hash: &str,
) -> Result<User, LoginError> {
    // import table `users`
    use db::schema::users;

    let new_user = diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(users::password_hash.eq(new_password_hash))
        .get_result::<User>(conn);

    match new_user {
        Err(e) => Err(LoginError::NoUserError(errJson!(e))),
        Ok(user) => Ok(user),
    }
}


pub fn set_email_verified(
    conn: &PgConnection,
    email: String,
    email_verified: bool,
) -> Result<User, LoginError> {
    // import table `users`
    use db::schema::users;

    let new_user = diesel::update(users::table.filter(users::email.eq(&email)))
        .set(users::email_verified.eq(&email_verified))
        .get_result::<User>(conn);

    match new_user {
        Err(e) => Err(LoginError::NoUserError(errJson!(e))),
        Ok(user) => Ok(user),
    }
}



