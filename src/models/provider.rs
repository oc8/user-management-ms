use std::convert::TryFrom;
use uuid::Uuid;
use num_derive::{FromPrimitive, ToPrimitive};
use crate::errors::{ApiError};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use crate::generate_secret;

/// Defines the supported event status.
#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive, sqlx::Type, Copy, Clone, Serialize)]
#[sqlx(type_name = "provider", rename_all = "lowercase")]
pub enum ProviderType {
    Local,
    PasswordLess,
}

impl ProviderType {
    pub fn to_string(&self) -> String {
        match self {
            ProviderType::Local => "local".to_string(),
            ProviderType::PasswordLess => "passwordless".to_string(),
        }
    }
}

/// Defines the full provider details structure.
///
/// This should never be returned in an API response, as it contains the users secret.
#[derive(Debug, PartialEq)]
pub struct Provider {
    pub id: Uuid,
    pub provider: ProviderType,
    pub created_at: NaiveDateTime,
}

/// Defines a user structure that can be inserted into the database.
#[derive(Debug, PartialEq)]
pub struct ProviderInsert {
    pub email: String,
    pub otp_secret: String,
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
    async fn create_provider(
        &mut self,
        provider: &ProviderInsert,
    ) -> Result<Provider, ApiError>;

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
    async fn find_provider_by_name(
        &mut self,
        name: &str,
    ) -> Result<Provider, ApiError>;

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
    async fn find_provider_by_id(
        &mut self,
        id: Uuid,
    ) -> Result<Provider, ApiError>;
}