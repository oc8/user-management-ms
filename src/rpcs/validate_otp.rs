use redis::Commands;
use tonic::{Status};
use protos::auth::{ValidateOtpRequest, ValidateOtpResponse};
use crate::database::PgPooledConnection;
use crate::models::user::{User};
use crate::validations::{validate_otp_request};

pub fn validate_otp(
    request: ValidateOtpRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<ValidateOtpResponse, Status> {
    validate_otp_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

    let user = User::find_by_email(conn, &request.email)
        .ok_or_else(|| Status::not_found("User not found"))?;

    let code: String = r_conn.get(&format!("otp:{}", request.email))
        .map_err(|_| Status::not_found("Invalid OTP"))?;

    if code != request.otp {
        return Err(Status::invalid_argument("Invalid OTP"));
    }

    let tokens = crate::services::auth::generate_tokens(&user)
        .map_err(|_| Status::internal("Failed to generate tokens"))?;

    r_conn.del(&format!("otp:{}", request.email))
        .map_err(|_| Status::internal("Failed to validate OTP"))?;

    Ok(ValidateOtpResponse {
        tokens: Some(tokens),
    })
}