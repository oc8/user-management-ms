use redis::Commands;
use tonic::{Status};
use protos::auth::{ValidateOtpRequest, ValidateOtpResponse};
use crate::database::PgPooledConnection;
use crate::errors;
use crate::models::user::{User};
use crate::validations::{validate_otp_request};

pub fn validate_otp(
    request: ValidateOtpRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<ValidateOtpResponse, Status> {
    validate_otp_request(&request)?;

    let user = User::find_by_email(conn, &request.email)
        .ok_or_else(|| Status::not_found(errors::USER_NOT_FOUND))?;

    let code: String = r_conn.get(&format!("otp:{}", request.email))
        .map_err(|e| {
            if e.kind() == redis::ErrorKind::TypeError {
                return Status::invalid_argument(errors::INVALID_OTP);
            }
            log::error!("Failed to get OTP: {:?}", e);
            Status::internal(errors::INTERNAL)
        })?;

    if code != request.otp {
        return Err(Status::invalid_argument(errors::INVALID_OTP));
    }

    let tokens = crate::services::auth::generate_tokens(&user)
        .map_err(|e| {
            log::error!("Failed to generate tokens: {:?}", e);
            Status::internal(errors::INTERNAL)
        })?;

    r_conn.del(&format!("otp:{}", request.email))
        .map_err(|e| {
            log::error!("Failed to delete OTP: {:?}", e);
            Status::internal(errors::INTERNAL)
        })?;

    Ok(ValidateOtpResponse {
        tokens: Some(tokens),
    })
}