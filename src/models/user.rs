use std::convert::TryFrom;
use uuid::Uuid;
use crate::errors::{ApiError};

use async_trait::async_trait;
use crate::generate_secret;

/// Defines the full user details structure.
///
/// This should never be returned in an API response, as it contains the users secret.
#[derive(Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub otp_secret: String,
}

/// Defines a user structure that can be inserted into the database.
#[derive(Debug, PartialEq)]
pub struct UserInsert {
    pub email: String,
    pub otp_secret: String,
}

/// Defines the data required to create a new user.
#[derive(Debug, PartialEq)]
pub struct UserRegister {
    pub email: String,
}

impl TryFrom<&UserRegister> for UserInsert {
    type Error = ApiError;

    fn try_from(account_register: &UserRegister) -> Result<Self, Self::Error> {
        let UserRegister {
            email,
            ..
        } = account_register;

        Ok(Self {
            email: email.to_string(),
            otp_secret: generate_secret(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum AuthType {
    MagicLink,
    OTP,
}

#[derive(Debug, PartialEq)]
pub struct UserAuthenticate {
    pub email: String,
    pub auth_type: AuthType,
}

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    /// Attempts to create a new user.
    ///
    /// # Parameters
    /// A struct containing the user details to be registered.
    ///
    /// # Return Values
    ///
    /// ## Success
    /// A struct containing the newly created users account details.
    ///
    /// ## Errors
    /// An error could occur if the user has already been registered, or a failure occurred with the
    /// database.
    async fn create_user(
        &mut self,
        account_register: &UserRegister,
    ) -> Result<User, ApiError>;

    /// Attempts to find a user by their email address.
    ///
    /// # Parameters
    /// The email address of the user to be found.
    ///
    /// # Return Values
    ///
    /// ## Success
    /// A struct containing the users account details.
    ///
    /// ## Errors
    /// If the attempted authentication details were incorrect, or a failure occurred with the
    /// database.
    async fn find_user_by_email(
        &mut self,
        email: &str,
    ) -> Result<User, ApiError>;

    /// Attempts to find a user by their id.
    ///
    /// # Parameters
    /// The id of the user to be found.
    ///
    /// ## Success
    /// A struct containing the users account details.
    ///
    /// ## Errors
    /// If the attempted authentication details were incorrect, or a failure occurred with the
    /// database.
    async fn find_user_by_id(
        &mut self,
        id: Uuid,
    ) -> Result<User, ApiError>;
}