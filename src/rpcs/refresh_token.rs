use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use redis::{Commands, RedisResult};
use tonic::{Status};
use uuid::Uuid;
use protos::auth::{RefreshTokenRequest, RefreshTokenResponse};
use crate::services::auth::{Claims, generate_tokens};
use crate::validations::{validate_refresh_token_request};

fn store_refresh_token(client: &redis::Client, token: &str, expiration_seconds: usize) -> RedisResult<()> {
    let mut con = client.get_connection()?;

    con.set_ex(token, expiration_seconds, expiration_seconds as u64)?;
    Ok(())
}

fn is_refresh_token_valid(client: &redis::Client, token: &str) -> RedisResult<bool> {
    let mut con = client.get_connection()?;

    let exists: bool = con.exists(token)?;
    Ok(!exists)
}

pub fn refresh_token(
    request: RefreshTokenRequest,
    r_client: &redis::Client,
) -> Result<RefreshTokenResponse, Status> {
    validate_refresh_token_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

    let token_valid = is_refresh_token_valid(r_client, &request.refresh_token)
        .map_err(|_| Status::internal("Failed to validate refresh token"))?;

    if !token_valid {
        return Err(Status::invalid_argument("Invalid refresh token"));
    }

    let secret = env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");
    let token = decode::<Claims>(&request.refresh_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

    match token {
        Ok(decoded) => {
            let user_id = Uuid::parse_str(&decoded.claims.sub)
                .map_err(|_| Status::invalid_argument("Invalid refresh token"))?;

            let tokens = generate_tokens(user_id)
                .map_err(|_| Status::internal("Failed to generate tokens"))?;

            let refresh_token_ttl = env::var("REFRESH_TOKEN_TTL").expect("REFRESH_TOKEN_TTL must be set").parse::<usize>().unwrap();

            store_refresh_token(r_client, &request.refresh_token, refresh_token_ttl)
                .map_err(|_| Status::internal("Failed to validate refresh token"))?;

            Ok(RefreshTokenResponse {
                tokens: Some(tokens),
            })
        }
        Err(_) => Err(Status::invalid_argument("Invalid token")),
    }
}