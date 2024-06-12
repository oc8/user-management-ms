use redis::Commands;
use protos::auth::{ValidateOtpRequest, ValidateOtpResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::errors::ApiError;
use crate::models::user::{UserRepository};
use crate::services::auth_service::generate_tokens;
use crate::validations::{validate_otp_request};

pub async fn validate_otp(
    request: ValidateOtpRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<ValidateOtpResponse, ApiError> {
    validate_otp_request(&request)?;

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