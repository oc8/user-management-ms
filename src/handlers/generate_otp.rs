use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use redis::Commands;
use totp_rs::{Algorithm, Secret, TOTP};
use validator::ValidateEmail;
use protos::auth::{GenerateOtpRequest, GenerateOtpResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::models::user::{UserRepository};
use crate::validations::{ValidateRequest};
use crate::errors::{ApiError, List, ValidationErrorKind};

impl ValidateRequest for GenerateOtpRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if self.email.validate_email() {
            Ok(())
        } else {
            Err(ApiError::ValidationError(List::<ValidationErrorKind>(vec![ValidationErrorKind::InvalidEmailFormat("email".to_string())])))
        }
    }
}

pub async fn generate_otp(
    request: GenerateOtpRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<GenerateOtpResponse, ApiError> {
    request.validate()?;

    let user = conn.find_user_by_email(&request.email).await?;

    let otp_ttl = env::var("OTP_TTL")?
        .parse::<u64>()?;

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, Secret::Encoded(user.otp_secret).to_bytes().unwrap(), None, request.email.clone())?;

    let code = totp.generate(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

    r_conn.set_ex(
        &format!("otp:{}", request.email),
        code.clone(),
        otp_ttl,
    )?;

    Ok(GenerateOtpResponse {
        code,
    })
}
