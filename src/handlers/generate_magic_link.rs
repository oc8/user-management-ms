use std::env;
use redis::Commands;
use protos::auth::{GenerateMagicLinkRequest, GenerateMagicLinkResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::{generate_secret};
use crate::errors::ApiError;
use crate::models::user::{UserRegister, UserRepository};
use crate::validations::validate_generate_magic_link_request;

pub async fn generate_magic_link(
    request: GenerateMagicLinkRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<GenerateMagicLinkResponse, ApiError> {
    validate_generate_magic_link_request(&request)?;

    let code = generate_secret();

    match conn.find_user_by_email(&request.email).await {
        Err(ApiError::DatabaseError(sqlx::Error::RowNotFound)) => {
            conn.create_user(&UserRegister {
                email: request.email.clone(),
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
        &format!("magic:{}", request.email),
        &code,
        otp_ttl,
    )?;

    Ok(GenerateMagicLinkResponse {
        code,
    })
}
