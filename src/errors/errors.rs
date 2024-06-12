use thiserror::Error;
use totp_rs::TotpUrlError;
use crate::report_error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ApiError {
    #[error("internal server error")]
    InternalServerError,
    #[error("the request was invalid {0}")]
    InvalidRequest(String),
    #[error("redis connection failure")]
    RedisConnectionFailure,
    #[error("cache error")]
    CacheError(#[from] redis::RedisError),
    #[error("database connection failure")]
    DatabaseConnectionFailure,
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("a user with the email {0} already exists")]
    UserAlreadyExists(String),
    #[error("invalid email format")]
    InvalidEmailFormat,
    #[error("invalid OTP")]
    InvalidOTP,
    #[error("invalid OTP format")]
    InvalidOTPFormat,
    #[error("invalid token")]
    InvalidToken,
    #[error("invalid token format")]
    InvalidTokenFormat,
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error("invalid magic code format")]
    InvalidMagicCodeFormat,
    #[error("invalid magic code")]
    InvalidMagicCode,
    #[error("user not found")]
    UserNotFound,
    #[error("invalid auth type")]
    InvalidAuthType,
    #[error("totp error")]
    TOTPError(#[from] TotpUrlError),
    #[error("jwt error")]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("env error")]
    EnvError(#[from] std::env::VarError),
    #[error("parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Unknown(#[source] Box<dyn std::error::Error + Sync + Send>),
}

impl From<ApiError> for tonic::Status {
    fn from(api_error: ApiError) -> tonic::Status {
        report_error(&api_error);
        match api_error {
            ApiError::InvalidRequest(_) => {
                tonic::Status::invalid_argument(format!("{:?}", api_error))
            }
            ApiError::RedisConnectionFailure => tonic::Status::internal(format!("{:?}", api_error)),
            ApiError::DatabaseConnectionFailure => tonic::Status::internal(format!("{:?}", api_error)),
            ApiError::DatabaseError(_) => tonic::Status::internal(format!("{:?}", api_error)),
            ApiError::UserAlreadyExists(_) => tonic::Status::already_exists(format!("{:?}", api_error)),
            ApiError::InvalidEmailFormat => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::InvalidOTP => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::InvalidOTPFormat => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::InvalidToken => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::InvalidTokenFormat => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::InvalidRefreshToken => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::InvalidMagicCodeFormat => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::InvalidMagicCode => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::UserNotFound => tonic::Status::not_found(format!("{:?}", api_error)),
            ApiError::InvalidAuthType => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::TOTPError(_) => tonic::Status::invalid_argument(format!("{:?}", api_error)),
            ApiError::JWTError(_) => tonic::Status::internal(format!("{:?}", api_error)),
            ApiError::EnvError(_) => tonic::Status::internal(format!("{:?}", api_error)),
            ApiError::ParseError(_) => tonic::Status::internal(format!("{:?}", api_error)),
            _ => tonic::Status::internal(format!("{:?}", api_error)),
        }
    }
}