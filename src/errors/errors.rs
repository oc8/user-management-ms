use std::env::VarError;
use std::fmt;
use std::num::ParseIntError;
use jsonwebtoken::errors::{ErrorKind};
use redis::RedisError;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use thiserror::Error;
use totp_rs::TotpUrlError;
use tonic_error::TonicError;
use crate::report_error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ValidationErrorKind {
    #[error("invalid email format, field: {0}")]
    InvalidEmailFormat(String),
    #[error("invalid otp format, field: {0}")]
    InvalidOtpFormat(String),
    #[error("invalid token format, field: {0}")]
    InvalidTokenFormat(String),
    #[error("invalid refresh token format, field: {0}")]
    InvalidRefreshTokenFormat(String),
    #[error("invalid magic code format, field: {0}")]
    InvalidMagicCodeFormat(String),
    #[error("invalid pkce challenge format, field: {0}")]
    InvalidPkceChallengeFormat(String),
    #[error("invalid pkce verifier format, field: {0}")]
    InvalidPkceVerifierFormat(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List<T>(pub Vec<T>);

impl<T> fmt::Display for List<T>
where
    T: fmt::Display + serde::Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&item.to_string());
        }
        write!(f, "{}", s)
    }
}

#[derive(Error, Debug, Serialize, Deserialize, TonicError)]
#[non_exhaustive]
pub enum ApiError {
    #[error("internal server error")]
    InternalServerError,
    #[error("the request was invalid {0}")]
    InvalidRequest(String),
    #[error("redis connection failure")]
    RedisConnectionFailure,
    #[error("cache error")]
    CacheError,
    #[error("database connection failure")]
    DatabaseConnectionFailure,
    #[error("database error {0}")]
    DatabaseError(String),
    #[error("already exists {0}")]
    AlreadyExists(String),
    #[error("not found {0}")]
    NotFound(String),
    #[error("invalid token")]
    InvalidToken,
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error("invalid magic code")]
    InvalidMagicCode,
    #[error("invalid otp")]
    InvalidOtp,
    #[error("user not found")]
    UserNotFound,
    #[error("invalid pkce")]
    InvalidPkce,
    #[error("validation error {0}")]
    ValidationError(List<ValidationErrorKind>),
}

impl ApiError {
    pub fn code(&self) -> tonic::Code {
        match self {
            ApiError::InvalidRequest(_) => tonic::Code::InvalidArgument,
            ApiError::RedisConnectionFailure => tonic::Code::Unavailable,
            ApiError::CacheError => tonic::Code::Unavailable,
            ApiError::DatabaseConnectionFailure => tonic::Code::Unavailable,
            ApiError::DatabaseError(_) => tonic::Code::Internal,
            ApiError::AlreadyExists(_) => tonic::Code::AlreadyExists,
            ApiError::NotFound(_) => tonic::Code::NotFound,
            ApiError::InvalidOtp => tonic::Code::InvalidArgument,
            ApiError::InvalidToken => tonic::Code::InvalidArgument,
            ApiError::InvalidRefreshToken => tonic::Code::InvalidArgument,
            ApiError::InvalidMagicCode => tonic::Code::InvalidArgument,
            ApiError::UserNotFound => tonic::Code::NotFound,
            ApiError::ValidationError(_) => tonic::Code::InvalidArgument,
            _ => tonic::Code::Internal,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            ApiError::ValidationError(_) => true,
            _ => false,
        }
    }

    pub fn errors(&self) -> serde_json::Value {
        match self {
            ApiError::ValidationError(errors) => {
                let mut v = Vec::new();
                for e in &errors.0 {
                    let type_name = to_variant_name(e).unwrap();
                    let data = e.to_string();
                    let (message, field) = data.split_once(", field: ").unwrap_or(("", ""));
                    v.push(serde_json::json!({
                        "message": message,
                        "field": field,
                        "type": type_name,
                    }));
                }

                serde_json::json!(v)
            }
            _ => serde_json::json!([])
        }
    }
}

impl From<VarError> for ApiError {
    fn from(error: VarError) -> Self {
        report_error(&error);
        ApiError::InternalServerError
    }
}

impl From <jsonwebtoken::errors::Error> for ApiError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        match error.kind() {
            ErrorKind::InvalidToken => ApiError::InvalidToken,
            _ => {
                report_error(&error);
                ApiError::InternalServerError
            }
        }
    }
}

impl From <TotpUrlError> for ApiError {
    fn from(error: TotpUrlError) -> Self {
        report_error(&error);
        ApiError::InternalServerError
    }
}

impl From<ParseIntError> for ApiError {
    fn from(error: ParseIntError) -> Self {
        report_error(&error);
        ApiError::InternalServerError
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => ApiError::UserNotFound,
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    ApiError::AlreadyExists(e.message().to_string())
                } else {
                    report_error(&e);
                    ApiError::DatabaseError(e.message().to_string())
                }
            }
            _ => {
                report_error(&error);
                ApiError::InternalServerError
            }
        }
    }
}

impl From<RedisError> for ApiError {
    fn from(error: RedisError) -> Self {
        report_error(&error);
        ApiError::InternalServerError
    }
}

impl From<lettre::transport::smtp::Error> for ApiError {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        report_error(&error);
        ApiError::InternalServerError
    }
}
