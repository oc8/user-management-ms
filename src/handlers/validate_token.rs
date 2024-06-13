use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use protos::auth::{ValidateTokenRequest, ValidateTokenResponse};
use crate::errors::{ApiError, List, ValidationErrorKind};
use crate::errors::ApiError::ValidationError;
use crate::services::auth_service::Claims;
use crate::validations::{ValidateRequest};

impl ValidateRequest for ValidateTokenRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if self.access_token.len() > 0 {
            Ok(())
        } else {
            Err(ValidationError(List::<ValidationErrorKind>(vec![ValidationErrorKind::InvalidTokenFormat("access_token".to_string())])))
        }
    }
}

pub async fn validate_token(
    request: ValidateTokenRequest,
) -> Result<ValidateTokenResponse, ApiError> {
    request.validate()?;

    let secret = env::var("ACCESS_TOKEN_SECRET")?;
    decode::<Claims>(
        &request.access_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )?;

    Ok(ValidateTokenResponse {
        valid: true,
    })
}