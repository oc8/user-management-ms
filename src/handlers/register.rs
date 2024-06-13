use validator::ValidateEmail;
use protos::auth::{RegisterRequest, RegisterResponse};
use crate::models::user::{UserRegister, UserRepository};
use crate::errors::{ApiError, List, ValidationErrorKind};
use crate::database::pg_database::PgPooledConnection;
use crate::validations::ValidateRequest;

impl ValidateRequest for RegisterRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if self.email.validate_email() {
            Ok(())
        } else {
            Err(ApiError::ValidationError(List::<ValidationErrorKind>(vec![ValidationErrorKind::InvalidEmailFormat("email".to_string())])))
        }
    }
}

pub async fn register(
    request: RegisterRequest,
    conn: &mut PgPooledConnection,
) -> Result<RegisterResponse, ApiError> {
    request.validate()?;

    let user = conn.create_user(&UserRegister {
        email: request.email.clone(),
    }).await?;

    log::info!("User registered: {:?}", user);

    Ok(RegisterResponse {})
}