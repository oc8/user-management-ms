use std::env;
use protos::auth::{LogoutRequest, LogoutResponse};
use crate::{store_token};
use crate::errors::{ApiError, List, ValidationErrorKind};
use crate::errors::ApiError::ValidationError;
use crate::validations::{ValidateRequest};

impl ValidateRequest for LogoutRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if self.refresh_token.len() > 0 {
            Ok(())
        } else {
            Err(ValidationError(List::<ValidationErrorKind>(vec![ValidationErrorKind::InvalidTokenFormat("refresh_token".to_string())])))
        }
    }
}

pub async fn logout(
    request: LogoutRequest,
    r_conn: &mut redis::Connection,
) -> Result<LogoutResponse, ApiError> {
    request.validate()?;

    let refresh_token_ttl = env::var("REFRESH_TOKEN_TTL")?
        .parse::<usize>()?;

    store_token(r_conn, &request.refresh_token, refresh_token_ttl)?;

    Ok(LogoutResponse {})
}