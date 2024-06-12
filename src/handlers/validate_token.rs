use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use protos::auth::{ValidateTokenRequest, ValidateTokenResponse};
use crate::errors::ApiError;
use crate::services::auth_service::Claims;
use crate::validations::{validate_token_request};

pub async fn validate_token(
    request: ValidateTokenRequest,
) -> Result<ValidateTokenResponse, ApiError> {
    validate_token_request(&request)?;

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