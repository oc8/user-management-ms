use redis::Commands;
use protos::auth::{ValidateMagicLinkRequest, ValidateMagicLinkResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::errors::ApiError;
use crate::models::user::{UserRepository};
use crate::services::auth_service::generate_tokens;
use crate::validations::validate_magic_link_request;

pub async fn validate_magic_link(
    request: ValidateMagicLinkRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<ValidateMagicLinkResponse, ApiError> {
    validate_magic_link_request(&request)?;

    let user = conn.find_user_by_email(&request.email).await?;

    let code: String = r_conn.get(
        &format!("magic:{}", user.email),
    ).map_err(|e| {
        if e.kind() == redis::ErrorKind::TypeError {
            return ApiError::InvalidMagicCode
        }
        ApiError::InternalServerError
    })?;

    if code != request.code {
        return Err(ApiError::InvalidMagicCode)
    }

    let tokens = generate_tokens(&user)?;

    r_conn.del(&format!("magic:{}", user.email))?;


    Ok(ValidateMagicLinkResponse {
        tokens: Some(tokens),
    })
}
