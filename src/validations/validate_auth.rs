use tonic::{Code, Status};
use protos::auth::{LoginRequest, LogoutRequest, RefreshTokenRequest, RegisterRequest, ValidateOtpRequest, ValidateTokenRequest};
use validator::{ValidateEmail};
use crate::errors;

pub fn validate_register_request(req: &RegisterRequest) -> Result<(), Status> {
    if req.email.validate_email() {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, errors::INVALID_EMAIL_FORMAT,
        ))
    }
}

pub fn validate_login_request(req: &LoginRequest) -> Result<(), Status> {
    if req.email.validate_email() {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, errors::INVALID_EMAIL_FORMAT,
        ))
    }
}


pub fn validate_otp_request(req: &ValidateOtpRequest) -> Result<(), Status> {
    if req.otp.len() == 6 {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, errors::INVALID_OTP_FORMAT,
        ))
    }
}

pub fn validate_token_request(req: &ValidateTokenRequest) -> Result<(), Status> {
    if req.access_token.len() > 0 {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, errors::INVALID_TOKEN_FORMAT,
        ))
    }
}

pub fn validate_refresh_token_request(req: &RefreshTokenRequest) -> Result<(), Status> {
    if req.refresh_token.len() > 0 {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, errors::INVALID_TOKEN_FORMAT,
        ))
    }
}

pub fn validate_logout_request(req: &LogoutRequest) -> Result<(), Status> {
    if req.refresh_token.len() > 0 {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, errors::INVALID_TOKEN_FORMAT,
        ))
    }
}