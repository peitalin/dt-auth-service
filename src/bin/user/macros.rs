
use crate::models::ErrJson;
use crate::models::LoginError;

#[macro_export]
macro_rules! errJson {
    ( $e:expr ) => {
        ErrJson {
            file: format!("{}:{}", file!(), line!()),
            message: $e.to_string(),
        }
    };
}

#[macro_export]
macro_rules! noJwtError {
    () => {
        LoginError::CredentialsError(
            ErrJson {
                file: format!("{}:{}", file!(), line!()),
                message: String::from("No JWT found, please login."),
            }
        )
    };
}


/// "expand" fields for Stripe
#[macro_export]
macro_rules! expand_as_ref {
    ( $expand:expr ) => {
        $expand.iter()
            .map(String::as_ref)
            .collect::<Vec<&str>>()
    };
}