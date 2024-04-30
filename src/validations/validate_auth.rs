use tonic::{Code, Status};
use protos::auth::{GenerateMagicLinkRequest, LoginRequest, LogoutRequest, RefreshTokenRequest, RegisterRequest, ValidateMagicLinkRequest, ValidateOtpRequest, ValidateTokenRequest};
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
    if req.email.validate_email() && req.otp.len() > 0 {
        Ok(())
    } else {
        if !req.email.validate_email() {
            return Err(Status::new(Code::InvalidArgument, errors::INVALID_EMAIL_FORMAT));
        }
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_OTP_FORMAT));
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

pub fn validate_generate_magic_link_request(req: &GenerateMagicLinkRequest) -> Result<(), Status> {
    if req.email.validate_email() {
        Ok(())
    } else {
        Err(Status::new(
            Code::InvalidArgument, errors::INVALID_EMAIL_FORMAT,
        ))
    }
}

pub fn validate_magic_link_request(req: &ValidateMagicLinkRequest) -> Result<(), Status> {
    if req.email.validate_email() && req.code.len() > 0 {
        Ok(())
    } else {
        if !req.email.validate_email() {
            return Err(Status::new(Code::InvalidArgument, errors::INVALID_EMAIL_FORMAT));
        }
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_MAGIC_CODE_FORMAT));
    }
}
