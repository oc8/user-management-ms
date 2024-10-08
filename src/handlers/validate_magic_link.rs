use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use redis::Commands;
use sha2::Digest;
use validator::ValidateEmail;
use protos::auth::{ValidateMagicLinkRequest, ValidateMagicLinkResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::errors::{ApiError, List, ValidationErrorKind};
use crate::errors::ApiError::ValidationError;
use crate::models::user::{UserRepository};
use crate::services::auth_service::generate_tokens;
use crate::validations::{ValidateRequest};

impl ValidateRequest for ValidateMagicLinkRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = vec![];

        if !self.email.validate_email() {
            errors.push(ValidationErrorKind::InvalidEmailFormat("email".to_string()));
        }

        if self.code.len() < 1 {
            errors.push(ValidationErrorKind::InvalidMagicCodeFormat("code".to_string()));
        }

        if self.pkce_verifier.len() < 43 || self.pkce_verifier.len() > 128 {
            errors.push(ValidationErrorKind::InvalidPkceVerifierFormat("pkce_verifier".to_string()));
        }

        if errors.len() > 0 {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)));
        }

        Ok(())
    }
}

pub async fn validate_magic_link(
    request: ValidateMagicLinkRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<ValidateMagicLinkResponse, ApiError> {
    request.validate()?;

    let user = conn.find_user_by_email(&request.email).await?;

    let code: String = r_conn.get(
        &format!("magic:{}", user.email),
    ).map_err(|e| {
        if e.kind() == redis::ErrorKind::TypeError {
            return ApiError::InvalidMagicCode
        }
        ApiError::InternalServerError
    })?;

    let stored_pkce_challenge: String = r_conn.get(&format!("otp_pkce:{}", request.email)).map_err(|e| {
        ApiError::InternalServerError
    })?;

    if code != request.code {
        return Err(ApiError::InvalidMagicCode)
    }

    let mut hasher = sha2::Sha256::new();
    hasher.update(request.pkce_verifier.as_bytes());
    let result = hasher.finalize();

    let expected_pkce_challenge = URL_SAFE_NO_PAD.encode(&result);
    if stored_pkce_challenge != expected_pkce_challenge {
        return Err(ApiError::InvalidPkce);
    }

    let tokens = generate_tokens(&user)?;

    r_conn.del(&format!("magic:{}", user.email))?;
    r_conn.del(&format!("otp_pkce:{}", request.email))?;


    Ok(ValidateMagicLinkResponse {
        tokens: Some(tokens),
    })
}
