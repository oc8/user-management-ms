use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Status};
use totp_rs::{Algorithm, Secret, TOTP};
use protos::auth::{ValidateOtpRequest, ValidateOtpResponse};
use crate::database::PgPooledConnection;
use crate::models::user::{User};
use crate::validations::{validate_otp_request};

pub fn validate_otp(
    request: ValidateOtpRequest,
    conn: &mut PgPooledConnection,
) -> Result<ValidateOtpResponse, Status> {
    validate_otp_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

    let user = User::find_by_email(conn, &request.email)
        .ok_or_else(|| Status::not_found("User not found"))?;

    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let otp_ttl = env::var("OTP_TTL").expect("OTP_TTL must be set").parse().unwrap();
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, otp_ttl, Secret::Encoded(user.otp_secret.clone()).to_bytes().unwrap(), None, "".to_string()).unwrap();

    if !totp.check(&request.otp, ts) {
        return Err(Status::invalid_argument("Invalid otp code"));
    }

    let tokens = crate::services::auth::generate_tokens(&user)
        .map_err(|_| Status::internal("Failed to generate tokens"))?;

    Ok(ValidateOtpResponse {
        tokens: Some(tokens),
    })
}