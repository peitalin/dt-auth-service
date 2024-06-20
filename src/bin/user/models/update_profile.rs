
use std::convert::From;
use crate::models::{
    UserId,
    User,
};
use crate::models::auth::UserRole;
use validator::{Validate, ValidationError};
use crate::models::validate_unoffensive_name;




#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserProfile {
    pub id: String,
    pub password_hash: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    #[validate(custom = "validate_unoffensive_name")]
    pub first_name: Option<String>,
    #[validate(custom = "validate_unoffensive_name")]
    pub last_name: Option<String>,
    pub email_verified: Option<bool>,
    pub is_suspended: Option<bool>,
    pub is_deleted: Option<bool>,
    pub user_role: Option<UserRole>,
}

impl UpdateUserProfile {
    pub fn new(user: &User) -> Self {
        UpdateUserProfile {
            id: user.id.clone(),
            password_hash: Some(user.password_hash.clone()),
            email: Some(user.email.clone()),
            password: None,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email_verified: Some(user.email_verified.clone()),
            is_suspended: Some(user.is_suspended.clone()),
            is_deleted: Some(user.is_deleted.clone()),
            user_role: user.user_role.clone(),
        }
    }

    pub fn update_email(&mut self, email: String) {
        self.email = Some(email);
    }

    pub fn update_password(&mut self, password: String) {
        // credential = hash(salt-id + password)
        let new_credential = crate::models::user::generate_credential(&self.id, &password);
        self.password_hash = Some(new_credential);
    }

    pub fn update_first_name(&mut self, first_name: String) {
        self.first_name = Some(first_name);
    }

    pub fn update_last_name(&mut self, last_name: String) {
        self.last_name = Some(last_name);
    }

    pub fn update_email_verified(&mut self, email_verified: bool) {
        self.email_verified = Some(email_verified);
    }

    pub fn update_is_suspended(&mut self, is_suspended: bool) {
        self.is_suspended = Some(is_suspended);
    }

    pub fn update_is_deleted(&mut self, is_deleted: bool) {
        self.is_deleted = Some(is_deleted);
    }

    pub fn update_user_role(&mut self, user_role: UserRole) {
        self.user_role = Some(user_role);
    }
}

impl<'a> From<&'a User> for UpdateUserProfile {
    fn from(user: &User) -> Self {
        UpdateUserProfile {
            id: user.id.clone(),
            password_hash: Some(user.password_hash.clone()),
            email: Some(user.email.clone()),
            password: None,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email_verified: Some(user.email_verified.clone()),
            is_suspended: Some(user.is_suspended.clone()),
            is_deleted: Some(user.is_deleted.clone()),
            user_role: user.user_role.clone(),
        }
    }
}

