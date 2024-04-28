use tonic::{Status};
use protos::auth::{RegisterRequest, RegisterResponse};
use crate::models::user::{NewUser, User};
use crate::validations::{validate_register_request};
use crate::database::PgPooledConnection;
use crate::errors::errors;
use user_management::generate_secret;

pub fn register(
    request: RegisterRequest,
    conn: &mut PgPooledConnection,
) -> Result<RegisterResponse, Status> {
    validate_register_request(&request)?;

    let secret = generate_secret();

    let user = NewUser {
        email: &request.email,
        otp_secret: secret.as_str(),
    };

    let _user = User::create(conn, user)
        .map_err(|_| Status::already_exists(errors::EMAIL_ALREADY_EXISTS))?;

    log::info!("User registered: {}", request.email);

    Ok(RegisterResponse {})
}