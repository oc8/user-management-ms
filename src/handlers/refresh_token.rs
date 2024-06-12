use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use protos::auth::{RefreshTokenRequest, RefreshTokenResponse};
use crate::database::pg_database::PgPooledConnection;
use crate::errors::{ApiError};
use crate::{is_token_valid, store_token};
use crate::models::user::{UserRepository};
use crate::services::auth_service::{Claims, generate_tokens};
use crate::validations::{validate_refresh_token_request};

pub async fn refresh_token(
    request: RefreshTokenRequest,
    conn: &mut PgPooledConnection,
    r_conn: &mut redis::Connection,
) -> Result<RefreshTokenResponse, ApiError> {
    validate_refresh_token_request(&request)?;

    if !is_token_valid(r_conn, &request.refresh_token)? {
        return Err(ApiError::InvalidToken);
    }

    let secret = env::var("REFRESH_TOKEN_SECRET")?;
    let token = decode::<Claims>(
        &request.refresh_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    );

    match token {
        Ok(decoded) => {
            let user_id = Uuid::parse_str(&decoded.claims.sub)
                .map_err(|_| ApiError::InvalidRefreshToken)?;

            let user = conn.find_user_by_id(user_id)
                .await?;

            let tokens = generate_tokens(&user)?;

            let refresh_token_ttl = env::var("REFRESH_TOKEN_TTL")?
                .parse::<usize>()?;

            store_token(r_conn, &request.refresh_token, refresh_token_ttl)?;

            Ok(RefreshTokenResponse {
                tokens: Some(tokens),
            })
        }
        Err(_) => Err(ApiError::InvalidRefreshToken),
    }
}