use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use tonic::{Status};
use protos::auth::{ValidateTokenRequest, ValidateTokenResponse};
use crate::validations::{validate_token_request};

pub fn validate_token(
    request: ValidateTokenRequest,
) -> Result<ValidateTokenResponse, Status> {
    validate_token_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

    let secret = env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
    decode::<crate::services::auth::Claims>(&request.access_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).map_err(|_| Status::invalid_argument("Invalid token"))?;

    Ok(ValidateTokenResponse {
        valid: true,
    })
}