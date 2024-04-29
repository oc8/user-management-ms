use redis::Commands;
use tonic::{Status};
use protos::auth::{ValidateMagicLinkRequest, ValidateMagicLinkResponse};
use crate::database::PgPooledConnection;
use crate::errors;
use crate::models::user::{User};
use crate::services::auth::generate_tokens;
use crate::validations::validate_magic_link_request;

pub fn validate_magic_link(
    request: ValidateMagicLinkRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<ValidateMagicLinkResponse, Status> {
    validate_magic_link_request(&request)?;

    let user = User::find_by_email(conn, &request.email)
        .ok_or_else(|| Status::not_found(errors::USER_NOT_FOUND))?;

    let code: String = r_conn.get(
        &format!("magic:{}", user.email),
    ).map_err(|e| {
        if e.kind() == redis::ErrorKind::TypeError {
            return Status::invalid_argument(errors::INVALID_MAGIC_CODE);
        }
        log::error!("Failed to get magic code: {:?}", e);
        Status::internal(errors::INTERNAL)
    })?;

    if code != request.code {
        return Err(Status::invalid_argument(errors::INVALID_MAGIC_CODE));
    }

    let tokens = generate_tokens(&user)
        .map_err(|e| {
            log::error!("Failed to generate tokens: {:?}", e);
            Status::internal(errors::INTERNAL)
        })?;

    r_conn.del(&format!("magic:{}", user.email))
        .map_err(|e| {
            log::error!("Failed to delete magic code: {:?}", e);
            Status::internal(errors::INTERNAL)
        })?;


    Ok(ValidateMagicLinkResponse {
        tokens: Some(tokens),
    })
}
