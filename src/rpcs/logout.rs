use std::env;
use tonic::{Status};
use protos::auth::{LogoutRequest, LogoutResponse};
use user_management::store_token;
use crate::errors;
use crate::validations::validate_logout_request;

pub fn logout(
    request: LogoutRequest,
    r_conn: &mut redis::Connection,
) -> Result<LogoutResponse, Status> {
    validate_logout_request(&request)?;

    let refresh_token_ttl = env::var("REFRESH_TOKEN_TTL").expect("REFRESH_TOKEN_TTL must be set").parse::<usize>().unwrap();

    store_token(r_conn, &request.refresh_token, refresh_token_ttl)
        .map_err(|_| {
            Status::internal(errors::INTERNAL)
        })?;

    Ok(LogoutResponse {})
}