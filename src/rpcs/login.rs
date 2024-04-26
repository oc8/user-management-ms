use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use redis::Commands;
use tonic::{Status};
use totp_rs::{Algorithm, Secret, TOTP};
use protos::auth::{LoginRequest, LoginResponse};
use crate::models::user::User;
use crate::validations::validate_login_request;
use crate::database::PgPooledConnection;
use crate::errors;

pub fn login(
    request: LoginRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<LoginResponse, Status> {
    validate_login_request(&request)?;

    let user = User::find_by_email(conn, &request.email)
        .ok_or_else(|| Status::not_found(errors::USER_NOT_FOUND))?;

    let otp_ttl = env::var("OTP_TTL").expect("OTP_TTL must be set").parse().unwrap();

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, Secret::Encoded(user.otp_secret).to_bytes().unwrap(), None, request.email.clone())
        .map_err(|_| {
            // report_error(e);
            Status::internal(errors::INTERNAL)
        })?;

    let code = totp.generate(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

    r_conn.set_ex(
        &format!("otp:{}", request.email),
        code.clone(),
        otp_ttl,
    ).map_err(|_| {
        // report_error(e);
        Status::internal(errors::INTERNAL)
    })?;

    Ok(LoginResponse {
        code,
    })
}