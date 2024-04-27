use tonic::{Status};
use protos::auth::{GenerateMagicLinkRequest, GenerateMagicLinkResponse};
use crate::database::PgPooledConnection;
use crate::errors;
use crate::models::user::{NewUser, User};
use crate::services::auth::generate_refresh_token;
use crate::validations::validate_generate_magic_link_request;
use user_management::generate_opt_secret;

pub fn generate_magic_link(
    request: GenerateMagicLinkRequest,
    conn: &mut PgPooledConnection,
) -> Result<GenerateMagicLinkResponse, Status> {
    validate_generate_magic_link_request(&request)?;

    let user = User::find_by_email(conn, &request.email);

    let (refresh_token, _user_created) = match user {
        Some(existing_user) => {
            let refresh_token = generate_refresh_token(&existing_user)
                .map_err(|_| Status::internal(errors::INTERNAL))?;
            (refresh_token, false)
        }
        None => {
            let secret = generate_opt_secret();
            let new_user = NewUser {
                email: &request.email,
                otp_secret: secret.as_str(),
            };

            let created_user = User::create(conn, new_user)
                .map_err(|_| Status::already_exists(errors::EMAIL_ALREADY_EXISTS))?;

            let refresh_token = generate_refresh_token(&created_user)
                .map_err(|_| Status::internal(errors::INTERNAL))?;
            (refresh_token, true)
        }
    };

    Ok(GenerateMagicLinkResponse {
        refresh_token: refresh_token.token,
        expires_in: refresh_token.expires_in,
    })
}
