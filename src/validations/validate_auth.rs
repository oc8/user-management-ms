use tonic::{Code, Status};
use protos::auth::{LoginRequest, RefreshTokenRequest, RegisterRequest, ValidateOtpRequest, ValidateTokenRequest};
use validator::{ValidateEmail};

pub fn validate_register_request(req: &RegisterRequest) -> Result<(), Status> {
    if req.email.validate_email() {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, "email_invalid_format".to_string(),
        ))
    }
}

pub fn validate_login_request(req: &LoginRequest) -> Result<(), Status> {
    if req.email.validate_email() {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, "email_invalid_format".to_string(),
        ))
    }
}

pub fn validate_otp_request(req: &ValidateOtpRequest) -> Result<(), Status> {
    if req.otp.len() == 6 {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, "otp_invalid_format".to_string(),
        ))
    }
}

pub fn validate_token_request(req: &ValidateTokenRequest) -> Result<(), Status> {
    if req.access_token.len() > 0 {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, "token_invalid_format".to_string(),
        ))
    }
}

pub fn validate_refresh_token_request(req: &RefreshTokenRequest) -> Result<(), Status> {
    if req.refresh_token.len() > 0 {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, "refresh_token_invalid_format".to_string(),
        ))
    }
}