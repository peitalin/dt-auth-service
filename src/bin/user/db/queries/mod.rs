#![allow(dead_code)]
pub mod users;
pub mod users_raw;
///  Contains raw/direct queries to Database


use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::db;
use crate::models::{
    UserId,
    LoginError,
    User,
    UpdateUserProfile,
    ErrJson,
    UserPublic,
};

pub use users::*;
