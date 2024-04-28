use redis::Commands;
use tonic::{Status};
use protos::auth::{GenerateMagicLinkRequest, GenerateMagicLinkResponse};
use crate::database::PgPooledConnection;
use crate::errors;
use crate::models::user::{NewUser, User};
use crate::validations::validate_generate_magic_link_request;
use user_management::generate_secret;

pub fn generate_magic_link(
    request: GenerateMagicLinkRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<GenerateMagicLinkResponse, Status> {
    validate_generate_magic_link_request(&request)?;

    let user = User::find_by_email(conn, &request.email);

    let secret = generate_secret();

    let (code, _user_created) = match user {
        Some(_) => {
            (secret, false)
        }
        None => {
            let user_secret = generate_secret();
            let new_user = NewUser {
                email: &request.email,
                otp_secret: user_secret.as_str(),
            };

            User::create(conn, new_user)
                .map_err(|_| Status::already_exists(errors::EMAIL_ALREADY_EXISTS))?;

            let code = generate_secret();
            (code, true)
        }
    };

    r_conn.set(
        &format!("magic:{}", request.email),
        code.clone(),
    ).map_err(|_| {
        Status::internal(errors::INTERNAL)
    })?;

    Ok(GenerateMagicLinkResponse {
        code,
    })
}
