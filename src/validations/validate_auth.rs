use protos::auth::{GenerateMagicLinkRequest, GenerateOtpRequest, LogoutRequest, RefreshTokenRequest, RegisterRequest, ValidateMagicLinkRequest, ValidateOtpRequest, ValidateTokenRequest};
use validator::{ValidateEmail};
use crate::errors::ApiError;

pub fn validate_register_request(req: &RegisterRequest) -> Result<(), ApiError> {
    if req.email.validate_email() {
        Ok(())
    } else {
        Err(ApiError::InvalidEmailFormat)
    }
}

pub fn validate_generate_otp_request(req: &GenerateOtpRequest) -> Result<(), ApiError> {
    if req.email.validate_email() {
        Ok(())
    } else {
        Err(ApiError::InvalidEmailFormat)
    }
}

pub fn validate_otp_request(req: &ValidateOtpRequest) -> Result<(), ApiError> {
    if req.email.validate_email() && req.otp.len() > 0 {
        Ok(())
    } else {
        if !req.email.validate_email() {
            return Err(ApiError::InvalidEmailFormat);
        }
        return Err(ApiError::InvalidOTPFormat);
    }
}

pub fn validate_token_request(req: &ValidateTokenRequest) -> Result<(), ApiError> {
    if req.access_token.len() > 0 {
        Ok(())
    } else {
        Err(ApiError::InvalidTokenFormat)
    }
}

pub fn validate_refresh_token_request(req: &RefreshTokenRequest) -> Result<(), ApiError> {
    if req.refresh_token.len() > 0 {
        Ok(())
    } else {
        Err(ApiError::InvalidTokenFormat)
    }
}

pub fn validate_logout_request(req: &LogoutRequest) -> Result<(), ApiError> {
    if req.refresh_token.len() > 0 {
        Ok(())
    } else {
        Err(ApiError::InvalidTokenFormat)
    }
}

pub fn validate_generate_magic_link_request(req: &GenerateMagicLinkRequest) -> Result<(), ApiError> {
    if req.email.validate_email() {
        Ok(())
    } else {
        Err(ApiError::InvalidEmailFormat)
    }
}

pub fn validate_magic_link_request(req: &ValidateMagicLinkRequest) -> Result<(), ApiError> {
    if req.email.validate_email() && req.code.len() > 0 {
        Ok(())
    } else {
        if !req.email.validate_email() {
            return Err(ApiError::InvalidEmailFormat);
        }
        return Err(ApiError::InvalidMagicCodeFormat);
    }
}
