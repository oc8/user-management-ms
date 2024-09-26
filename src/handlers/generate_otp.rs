use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::{header, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use redis::Commands;
use totp_rs::{Algorithm, Secret, TOTP};
use validator::ValidateEmail;
use protos::auth::{GenerateOtpRequest, GenerateOtpResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::models::user::{UserRepository};
use crate::validations::{ValidateRequest};
use crate::errors::{ApiError, List, ValidationErrorKind};
use crate::errors::ApiError::ValidationError;
use crate::get_config;
use crate::models::mails::generate_otp_email;

impl ValidateRequest for GenerateOtpRequest {
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

pub async fn generate_otp(
    request: GenerateOtpRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<GenerateOtpResponse, ApiError> {
    request.validate()?;

    let cfg = get_config!();

    let user = conn.find_user_by_email(&request.email).await?;

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, Secret::Encoded(user.otp_secret).to_bytes().unwrap(), None, request.email.clone())?;

    let code = totp.generate(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

    r_conn.set_ex(
        &format!("otp:{}", request.email),
        code.clone(),
        cfg.otp_ttl,
    )?;

    r_conn.set_ex(
        &format!("otp_pkce:{}", request.email),
        request.pkce_challenge.clone(),
        cfg.otp_ttl,
    )?;

    let from = format!("{} <{}>", cfg.email_from_name, cfg.email_from_email).as_str();
    let to = format!("<{}>", user.email).as_str();
    let html = generate_otp_email(&code, &user.email, cfg.otp_ttl);
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Your Bookeat OTP code")
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Your OTP code is: ") + &code),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(html)),
                ),
        )
        .unwrap();

    let creds = Credentials::new(cfg.smtp_user.clone(), cfg.smtp_password.clone());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.smtp_host)?
        .port(cfg.smtp_port)
        .credentials(creds)
        .build();

    mailer.send(email).await?;

    Ok(GenerateOtpResponse {})
}
