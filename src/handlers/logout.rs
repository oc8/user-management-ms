use std::env;
use protos::auth::{LogoutRequest, LogoutResponse};
use crate::{store_token};
use crate::errors::ApiError;
use crate::validations::validate_logout_request;

pub async fn logout(
    request: LogoutRequest,
    r_conn: &mut redis::Connection,
) -> Result<LogoutResponse, ApiError> {
    validate_logout_request(&request)?;

    let refresh_token_ttl = env::var("REFRESH_TOKEN_TTL")?
        .parse::<usize>()?;

    store_token(r_conn, &request.refresh_token, refresh_token_ttl)?;

    Ok(LogoutResponse {})
}