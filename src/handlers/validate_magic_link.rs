use redis::Commands;
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
        let mut errors = List::<ValidationErrorKind>(vec![]);

        if !self.email.validate_email() {
            errors.0.push(ValidationErrorKind::InvalidEmailFormat("email".to_string()));
        }

        if self.code.len() < 1 {
            errors.0.push(ValidationErrorKind::InvalidMagicCodeFormat("code".to_string()));
        }

        if errors.0.len() > 0 {
            return Err(ValidationError(errors));
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

    if code != request.code {
        return Err(ApiError::InvalidMagicCode)
    }

    let tokens = generate_tokens(&user)?;

    r_conn.del(&format!("magic:{}", user.email))?;


    Ok(ValidateMagicLinkResponse {
        tokens: Some(tokens),
    })
}
