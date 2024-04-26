use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use redis::{Commands, RedisResult};
use tonic::{Status};
use uuid::Uuid;
use protos::auth::{RefreshTokenRequest, RefreshTokenResponse};
use user_management::report_error;
use crate::database::PgPooledConnection;
use crate::errors::errors;
use crate::models::user::User;
use crate::services::auth::{Claims, generate_tokens};
use crate::validations::{validate_refresh_token_request};

fn store_refresh_token(conn: &mut redis::Connection, token: &str, expiration_seconds: usize) -> RedisResult<()> {
    conn.set_ex(token, expiration_seconds, expiration_seconds as u64)?;
    Ok(())
}

fn is_refresh_token_valid(conn: &mut redis::Connection, token: &str) -> RedisResult<bool> {
    let exists: bool = conn.exists(token)?;
    Ok(!exists)
}

pub fn refresh_token(
    request: RefreshTokenRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<RefreshTokenResponse, Status> {
    validate_refresh_token_request(&request)?;

    let token_valid = is_refresh_token_valid(r_conn, &request.refresh_token)
        .map_err(|_|  Status::internal(errors::INTERNAL))?;

    if !token_valid {
        return Err(Status::invalid_argument(errors::INVALID_REFRESH_TOKEN));
    }

    let secret = env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");
    let token = decode::<Claims>(&request.refresh_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

    match token {
        Ok(decoded) => {
            let user_id = Uuid::parse_str(&decoded.claims.sub)
                .map_err(|_| Status::invalid_argument(errors::INVALID_REFRESH_TOKEN))?;

            let user = User::find_by_id(conn, user_id)
                .ok_or_else(|| Status::not_found(errors::USER_NOT_FOUND))?;

            let tokens = generate_tokens(&user)
                .map_err(|_| {
                    Status::internal(errors::INTERNAL)
                })?;

            let refresh_token_ttl = env::var("REFRESH_TOKEN_TTL").expect("REFRESH_TOKEN_TTL must be set").parse::<usize>().unwrap();

            store_refresh_token(r_conn, &request.refresh_token, refresh_token_ttl)
                .map_err(|e| {
                    report_error(e);
                    Status::internal(errors::INTERNAL)
                })?;

            Ok(RefreshTokenResponse {
                tokens: Some(tokens),
            })
        }
        Err(_) => Err(Status::invalid_argument(errors::INVALID_REFRESH_TOKEN)),
    }
}