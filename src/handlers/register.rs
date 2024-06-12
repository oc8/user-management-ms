use protos::auth::{RegisterRequest, RegisterResponse};
use crate::models::user::{UserRegister, UserRepository};
use crate::validations::{validate_register_request};
use crate::errors::{ApiError};
use crate::database::pg_database::PgPooledConnection;

pub async fn register(
    request: RegisterRequest,
    conn: &mut PgPooledConnection,
) -> Result<RegisterResponse, ApiError> {
    validate_register_request(&request)?;

    let user = conn.create_user(&UserRegister {
        email: request.email.clone(),
    }).await?;

    log::info!("User registered: {:?}", user);

    Ok(RegisterResponse {})
}