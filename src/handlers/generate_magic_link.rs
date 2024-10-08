use std::env;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use redis::Commands;
use validator::ValidateEmail;
use protos::auth::{GenerateMagicLinkRequest, GenerateMagicLinkResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::{generate_secret, get_config, report_error};
use crate::errors::{ApiError, List, ValidationErrorKind};
use crate::errors::ApiError::ValidationError;
use crate::models::mails::{generate_magic_link_email, generate_otp_email};
use crate::models::user::{UserRegister, UserRepository};
use crate::validations::{ValidateRequest};

impl ValidateRequest for GenerateMagicLinkRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = vec![];

        if !self.email.validate_email() {
            errors.push(ValidationErrorKind::InvalidEmailFormat("email".to_string()));
        }

        if self.pkce_challenge.len() < 1 {
            errors.push(ValidationErrorKind::InvalidPkceChallengeFormat("pkce_challenge".to_string()));
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

    let cfg = get_config!();

    let code = generate_secret();
    let url = format!("{}?code={}", cfg.magic_link_redirect_url, code);
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

    let from = format!("{} <{}>", cfg.email_from_name, cfg.email_from_email);
    let to = format!("<{}>", email);
    let html = generate_magic_link_email(&url, &email, cfg.otp_ttl);
    let email = Message::builder()
        .from(from.parse().map_err(|e| {
            log::error!("{}" ,e);
            report_error(&e);
            ApiError::InternalServerError
        })?)
        .to(to.parse().unwrap())
        .subject("Your Bookeat magic link")
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Your magic link is: ") + &url),
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

    Ok(GenerateMagicLinkResponse {})
}
