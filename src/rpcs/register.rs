use rand::Rng;
use tonic::{Status};
use protos::auth::{RegisterRequest, RegisterResponse};
use crate::models::user::{NewUser, User};
use crate::validations::{validate_register_request};
use crate::database::PgPooledConnection;

fn generate_opt_secret() -> String {
    let mut secret_key = vec![0u8; 20];
    rand::thread_rng().fill(&mut secret_key[..]);

    let base32_secret = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &secret_key);

    base32_secret
}

pub fn register(
    request: RegisterRequest,
    conn: &mut PgPooledConnection,
) -> Result<RegisterResponse, Status> {
    validate_register_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

    let secret = generate_opt_secret();

    let user = NewUser {
        email: &request.email,
        otp_secret: secret.as_str(),
    };

    let _user = User::create(conn, user)
        .map_err(|e| Status::internal(e.to_string()))?;

    log::info!("User registered: {}", request.email);

    Ok(RegisterResponse {})
}