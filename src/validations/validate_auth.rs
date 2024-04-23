use tonic::{Code, Status};
use protos::auth::{LoginRequest, RegisterRequest};
use validator::{ValidateEmail};

pub fn validate_register_request(user: &RegisterRequest) -> Result<(), Status> {
    let valid_email = user.email.validate_email();
    if user.email.validate_email() {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, "email_invalid_format".to_string(),
        ))
    }
}

pub fn validate_login_request(user: &LoginRequest) -> Result<(), Status> {
    if user.email.validate_email() {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, "email_invalid_format".to_string(),
        ))
    }
}