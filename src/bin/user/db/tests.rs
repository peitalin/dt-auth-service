use crate::models::User;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
// from ./src/db
use dt::db;
use dt::db::establish_connection_pg;
use crate::models::LoginError;
use crate::models::{
    UserId,
    UpdateUserProfile,
    ErrJson,
};
use crate::db::queries::users_raw::*;


#[test]
fn gets_user_profile() {
    let conn = db::establish_connection_pg();
    let _res = login(&conn, String::from("sirius@hogwarts.com"), String::from("password"));
}

#[test]
fn creates_and_deletes_user_profile() {
    let conn = db::establish_connection_pg();
    conn.test_transaction::<_, LoginError, _>(|| {
        let created_user = insert_user_profile(
            &conn,
            &User::new(
                String::from("test@testemail.com"),
                String::from("password"),
                None,
                None,
            )
        ).expect("created test user");
        delete_user_profile(
            &conn,
            &created_user.id,
            String::from("password"),
        )
    });
}

#[test]
fn creates_updates_gets_and_deletes_user_profile() {
    let conn = db::establish_connection_pg();
    conn.test_transaction::<_, LoginError, _>(|| {

        let created_user = insert_user_profile(
            &conn,
            &User::new(
                String::from("jack@black.com"),
                String::from("tenacious"),
                None,
                None,
            )
        ).expect("Created test user");

        // Update jack black's profile
        let mut updated_profile = UpdateUserProfile::new(&created_user);
        updated_profile.update_username(String::from("jablinski"));
        updated_profile.update_email(String::from("jack.black@tenacious.com"));
        let _update_result = update_user_profile(
            &conn,
            updated_profile,
        )?;

        // Get Jack's profile again
        let jack = login(&conn,
            String::from("jack.black@tenacious.com"),
            String::from("tenacious")
        )?;

        // Delete Jack's profile first before assertion tests
        let delete_result = delete_user_profile(
            &conn,
            &jack.id,
            String::from("tenacious"),
        )?;
        // See if Jack's information is updated
        assert_eq!(jack.username, Some(String::from("jablinski")));
        assert_eq!(jack.email, String::from("jack.black@tenacious.com"));
        Ok(delete_result)
    });
}

#[test]
fn updates_password_credentials() {

    let conn = db::establish_connection_pg();
    conn.test_transaction::<String, LoginError, _>(|| {

        let created_user = insert_user_profile(
            &conn,
            &User::new(
                String::from("kyle@gass.com"),
                String::from("tribute"),
                None,
                None,
            )
        ).expect("Created test user");
        // Update jack black's profile
        let mut updated_profile = UpdateUserProfile::new(&created_user);
        updated_profile.update_password(String::from("new_password"));

        let _update_result = update_user_profile(
            &conn,
            updated_profile,
        )?;

        // Get Jack's profile again
        let jack = login(&conn,
            String::from("kyle@gass.com"),
            String::from("new_password")
        )?;

        // Delete Jack's profile first before assertion tests
        delete_user_profile(
            &conn,
            &jack.id,
            String::from("new_password"),
        )
    });
}

#[test]
fn stores_user_profile() {

    // Leave these writes in database for tests
    let conn = establish_connection_pg();
    let email = String::from("sirius@hogwarts.com");
    let passwd = String::from("password");
    let first_name = Some(String::from("Sirius"));
    let last_name = Some(String::from("Black"));
    let user = User::new(
        email.clone(),
        passwd.clone(),
        first_name,
        last_name,
    );
    let _res: Result<User, LoginError> = match &user.store_user_profile(&conn) {
        Ok(user) => Ok(user.clone()),
        Err(_) => {
            // sirius already exists, can't create duplicate.
            crate::db::queries::users_raw::get_user_profile_by_email(
                &conn,
                "sirius@hogwarts.com",
            )
        }
    };
}

#[test]
#[should_panic]
fn stores_user_profile_panic() {

    // Leave these writes in database for tests
    let conn = establish_connection_pg();
    let email = String::from("moony@hogwarts.com");
    let passwd = String::from("password");
    let first_name = Some(String::from("Remus"));
    let last_name = Some(String::from("Lupin"));
    let mut user = User::new(
        email.clone(),
        passwd.clone(),
        first_name,
        last_name,
    );

    let _store_cred = &user.store_user_profile(&conn);
    let verif_cred_err = &user.verify_credentials(&conn,
        String::from("123wrongpassword")
    );
    match verif_cred_err {
        Ok(s) => debug!("Verify Credentials: {:?}\n", s),
        Err(_e) => panic!("Wrong Password!"),
    };
}


#[test]
fn deletes_user_profile() {
    let conn = establish_connection_pg();
    conn.test_transaction::<String, LoginError, _>(|| {
        let email = String::from("james.potter@hogwarts.com");
        let passwd = String::from("password");
        let first_name = Some(String::from("James"));
        let last_name = Some(String::from("Potter"));
        let user = User::new(
            email.clone(),
            passwd.clone(),
            first_name,
            last_name,
        );

        let james_profile = match user.store_user_profile(&conn) {
            Ok(user) => user,
            Err(_e) => {
                crate::db::queries::users_raw::get_user_profile_by_email(
                    &conn,
                    "james.potter@hogwarts.com",
                ).expect("Couldn't create test james.potter@hogwarts.com profile")
            }
        };
        crate::db::queries::users_raw::delete_user_profile(
            &conn, &james_profile.id, String::from("password")
        )
    });
}


#[test]
fn update_email_verified() {
    let conn = establish_connection_pg();
    conn.test_transaction::<(), LoginError, _>(|| {

        use dt::db::schema::users;

        let email = String::from("james.potter@hogwarts.com");
        let passwd = String::from("password");
        let first_name = Some(String::from("James"));
        let last_name = Some(String::from("Potter"));
        let user = User::new(
            email.clone(),
            passwd.clone(),
            first_name,
            last_name,
        );

        let _james_profile = match user.store_user_profile(&conn) {
            Ok(user) => user,
            Err(_e) => {
                crate::db::queries::users_raw::get_user_profile_by_email(
                    &conn,
                    "james.potter@hogwarts.com",
                ).expect("Couldn't create test james.potter@hogwarts.com profile")
            }
        };

        let res: QueryResult<User> = diesel::update(
                users::table.filter(users::email.eq("james.potter@hogwarts.com"))
            )
            .set(users::email_verified.eq(true))
            .get_result::<User>(&conn);

            // .map_err(|e| EmailVerifyError::DbError(e.to_string()))?;

        match res {
            Ok(user) => assert_eq!(user.email_verified, true),
            Err(_) => panic!("Couldn't update james.potter@hogwarts.com email_verified status"),
        };
        Ok(())
    });
}


