use tonic::{Status};
use protos::auth::{GenerateMagicLinkRequest, GenerateMagicLinkResponse};
use crate::database::PgPooledConnection;
use crate::errors;
use crate::models::user::User;
use crate::services::auth::generate_refresh_token;
use crate::validations::validate_generate_magic_link_request;

pub fn generate_magic_link(
    request: GenerateMagicLinkRequest,
    conn: &mut PgPooledConnection,
) -> Result<GenerateMagicLinkResponse, Status> {
    validate_generate_magic_link_request(&request)?;

    let user = User::find_by_email(conn, request.email.as_str())
        .ok_or_else(|| Status::not_found(errors::USER_NOT_FOUND))?;

    let refresh_token = generate_refresh_token(&user)
        .map_err(|_| Status::internal(errors::INTERNAL))?;

    Ok(GenerateMagicLinkResponse {
        refresh_token: refresh_token.token,
        expires_in: refresh_token.expires_in,
    })
}