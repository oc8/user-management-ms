use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Status};
use totp_rs::{Algorithm, Secret, TOTP};
use protos::auth::{LoginRequest, LoginResponse};
use crate::models::user::User;
use crate::validations::validate_login_request;
use crate::database::PgPooledConnection;

pub fn login(
    request: LoginRequest,
    conn: &mut PgPooledConnection,
) -> Result<LoginResponse, Status> {
    validate_login_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

    let user = User::find_by_email(conn, &request.email)
        .ok_or_else(|| Status::not_found("User not found"))?;

    let otp_ttl = env::var("OTP_TTL").expect("OTP_TTL must be set").parse().unwrap();

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, otp_ttl, Secret::Encoded(user.otp_secret).to_bytes().unwrap(), None, "".to_string()).unwrap();

    let code = totp.generate(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

    println!("Code: {}", code);

    Ok(LoginResponse {
        code,
    })
}