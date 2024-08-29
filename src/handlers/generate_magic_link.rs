use std::env;
use redis::Commands;
use validator::ValidateEmail;
use protos::auth::{GenerateMagicLinkRequest, GenerateMagicLinkResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::{generate_secret};
use crate::errors::{ApiError, List, ValidationErrorKind};
use crate::errors::ApiError::ValidationError;
use crate::models::user::{UserRegister, UserRepository};
use crate::validations::{ValidateRequest};

impl ValidateRequest for GenerateMagicLinkRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = vec![];

        if !self.email.validate_email() {
            errors.push(ValidationErrorKind::InvalidEmailFormat("email".to_string()));
        }

        if self.pkce_challenge.len() < 1 {
            errors.push(ValidationErrorKind::InvalidPKCEChallengeFormat("pkce_challenge".to_string()));
        }

        if errors.len() > 0 {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)));
        }

        Ok(())
    }
}

pub async fn generate_magic_link(
    request: GenerateMagicLinkRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<GenerateMagicLinkResponse, ApiError> {
    request.validate()?;

    let code = generate_secret();
    let email = request.email;

    match conn.find_user_by_email(&email).await {
        Err(ApiError::UserNotFound) => {
            conn.create_user(&UserRegister {
                email: email.clone(),
            }).await?;
        }
        Err(e) => {
            return Err(e);
        }
        _ => {}
    };

    let otp_ttl = env::var("OTP_TTL")?
        .parse::<u64>()?;

    r_conn.set_ex(
        &format!("magic:{}", email),
        &code,
        otp_ttl,
    )?;

    r_conn.set_ex(
        &format!("otp_pkce:{}", email),
        request.pkce_challenge.clone(),
        otp_ttl,
    )?;

    Ok(GenerateMagicLinkResponse {
        code,
    })
}
