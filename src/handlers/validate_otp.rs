use redis::Commands;
use validator::ValidateEmail;
use protos::auth::{ValidateOtpRequest, ValidateOtpResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::errors::{ApiError, List, ValidationErrorKind};
use crate::errors::ApiError::ValidationError;
use crate::models::user::{UserRepository};
use crate::services::auth_service::generate_tokens;
use crate::validations::{ValidateRequest};

impl ValidateRequest for ValidateOtpRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = List::<ValidationErrorKind>(vec![]);

        if !self.email.validate_email() {
            errors.0.push(ValidationErrorKind::InvalidEmailFormat("email".to_string()));
        }

        if self.otp.len() != 6 {
            errors.0.push(ValidationErrorKind::InvalidOTPFormat("code".to_string()));
        }

        if errors.0.len() > 0 {
            return Err(ValidationError(errors));
        }

        Ok(())
    }
}

pub async fn validate_otp(
    request: ValidateOtpRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<ValidateOtpResponse, ApiError> {
    request.validate()?;

    let user = conn.find_user_by_email(&request.email).await?;

    let code: String = r_conn.get(&format!("otp:{}", request.email))
        .map_err(|e| {
            if e.kind() == redis::ErrorKind::TypeError {
                return ApiError::InvalidOTP
            }
            ApiError::InternalServerError
        })?;

    if code != request.otp {
        return Err(ApiError::InvalidOTP)
    }

    let tokens = generate_tokens(&user)?;

    r_conn.del(&format!("otp:{}", request.email))?;

    Ok(ValidateOtpResponse {
        tokens: Some(tokens),
    })
}