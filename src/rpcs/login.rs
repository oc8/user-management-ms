use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use redis::Commands;
use tonic::{Status};
use totp_rs::{Algorithm, Secret, TOTP};
use protos::auth::{LoginRequest, LoginResponse};
use crate::models::user::User;
use crate::validations::validate_login_request;
use crate::database::PgPooledConnection;

pub fn login(
    request: LoginRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<LoginResponse, Status> {
    validate_login_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

    let user = User::find_by_email(conn, &request.email)
        .ok_or_else(|| Status::not_found("User not found"))?;

    let otp_ttl = env::var("OTP_TTL").expect("OTP_TTL must be set").parse().unwrap();

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, Secret::Encoded(user.otp_secret).to_bytes().unwrap(), None, request.email.clone()).unwrap();

    let code = totp.generate(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

    let _ : () = r_conn.set_ex(
        &format!("otp:{}", request.email),
        code.clone(),
        otp_ttl,
    ).map_err(|_| Status::internal("Failed to generate OTP"))?;

    Ok(LoginResponse {
        code,
    })
}