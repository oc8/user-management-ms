use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use redis::{Commands, RedisResult};
use tonic::{Status};
use totp_rs::{Algorithm, Secret, TOTP};
use protos::auth::{ValidateOtpRequest, ValidateOtpResponse};
use crate::database::PgPooledConnection;
use crate::models::user::{User};
use crate::validations::{validate_otp_request};

fn store_otp_token(client: &redis::Client, code: &str, expiration_seconds: usize) -> RedisResult<()> {
    let mut con = client.get_connection()?;

    con.set_ex(code, expiration_seconds, expiration_seconds as u64)?;
    Ok(())
}

// fn is_otp_valid(client: &redis::Client, code: &str) -> RedisResult<bool> {
//     let mut con = client.get_connection()?;
//
//     let exists: bool = con.exists(code)?;
//     Ok(!exists)
// }

pub fn validate_otp(
    request: ValidateOtpRequest,
    r_client: &redis::Client,
    conn: &mut PgPooledConnection,
) -> Result<ValidateOtpResponse, Status> {
    validate_otp_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

    // let valid_otp = is_otp_valid(r_client, &request.otp)
    //     .map_err(|_| Status::internal("Failed to validate otp"))?;

    // println!("Valid otp: {}", valid_otp);
    //
    // if !valid_otp {
    //     return Err(Status::invalid_argument("Invalid otp code"));
    // }

    let user = User::find_by_email(conn, &request.email)
        .ok_or_else(|| Status::not_found("User not found"))?;

    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let otp_ttl = env::var("OTP_TTL").expect("OTP_TTL must be set").parse().unwrap();
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, otp_ttl, Secret::Encoded(user.otp_secret).to_bytes().unwrap(), None, "".to_string()).unwrap();

    let valid = totp.check(&request.otp, ts);

    if valid {
        let tokens = crate::services::auth::generate_tokens(user.id)
            .map_err(|_| Status::internal("Failed to generate tokens"))?;

        let otp_ttl = env::var("OTP_TTL").expect("OTP_TTL must be set").parse::<usize>().unwrap();

        store_otp_token(r_client, &request.otp, otp_ttl)
            .map_err(|_| Status::internal("Failed to store otp token"))?;

        Ok(ValidateOtpResponse {
            tokens: Some(tokens),
        })
    } else {
        Err(Status::invalid_argument("Invalid otp code"))
    }
}