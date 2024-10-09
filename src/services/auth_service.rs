use std::env;
use std::sync::{Arc};
use autometrics::autometrics;
use chrono::Utc;
use tonic::{Request, Response, Status};
use jsonwebtoken::{encode, EncodingKey, Header};

use protos::auth::{auth_server::Auth, User as UserProto, *};
use serde::{Deserialize, Serialize};
use crate::models::user::User;

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};
use crate::database::pg_database::{PgPool, PgPooledConnection};
use crate::errors::ApiError;
use crate::handlers;

const API_SLO: Objective = Objective::new("auth_api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct AuthService {
    pub pool: Arc<PgPool>,
    pub cache: Arc<redis::Client>,
}

impl Clone for AuthService {
    fn clone(&self) -> Self {
        AuthService {
            pool: Arc::clone(&self.pool),
            cache: Arc::clone(&self.cache),
        }
    }
}

#[tonic::async_trait]
#[autometrics(objective = API_SLO)]
impl Auth for AuthService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        handlers::register(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn generate_otp(
        &self,
        request: Request<GenerateOtpRequest>,
    ) -> Result<Response<GenerateOtpResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let mut r_conn = get_redis_connection(&self.cache)?;
        handlers::generate_otp(request.into_inner(), &mut conn, &mut r_conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn generate_magic_link(&self, request: Request<GenerateMagicLinkRequest>) -> Result<Response<GenerateMagicLinkResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let mut r_conn = get_redis_connection(&self.cache)?;
        handlers::generate_magic_link(request.into_inner(), &mut conn, &mut r_conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn validate_magic_link(&self, request: Request<ValidateMagicLinkRequest>) -> Result<Response<ValidateMagicLinkResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let mut r_conn = get_redis_connection(&self.cache)?;
        handlers::validate_magic_link(request.into_inner(), &mut conn, &mut r_conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn validate_otp(
        &self,
        request: Request<ValidateOtpRequest>,
    ) -> Result<Response<ValidateOtpResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let mut r_conn = get_redis_connection(&self.cache)?;
        handlers::validate_otp(request.into_inner(), &mut conn, &mut r_conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn validate_token(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        handlers::validate_token(request.into_inner())
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<RefreshTokenResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let mut r_conn = get_redis_connection(&self.cache)?;
        handlers::refresh_token(request.into_inner(), &mut conn, &mut r_conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>
    ) -> Result<Response<LogoutResponse>, Status> {
        let mut r_conn = get_redis_connection(&self.cache)?;
        handlers::logout(request.into_inner(), &mut r_conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    iss: String,
    iat: usize,
    exp: usize,
}

pub(crate) struct Token {
    pub token: String,
    pub expires_in: u64,
}

pub(crate) fn generate_tokens(user: &User) -> Result<Tokens, ApiError> {
    let access_token = generate_access_token(user)?;
    let refresh_token = generate_refresh_token(user)?;

    Ok(Tokens {
        access_token: access_token.token,
        refresh_token: refresh_token.token,
        expires_in: access_token.expires_in,
    })
}

pub(crate) fn generate_access_token(user: &User) -> Result<Token, ApiError> {
    let access_token_expiration = env::var("ACCESS_TOKEN_TTL")?
        .parse()?;
    let issuer = env::var("JWT_ISSUER")?;
    let jwt_secret = env::var("ACCESS_TOKEN_SECRET")?;

    let exp = Utc::now().timestamp() + access_token_expiration as i64;

    let claims = Claims {
        sub: user.id.to_string(),
        iss: issuer,
        iat: Utc::now().timestamp() as usize,
        exp: exp as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;

    Ok(Token {
        token,
        expires_in: access_token_expiration,
    })
}

pub(crate) fn generate_refresh_token(user: &User) -> Result<Token, ApiError> {
    let refresh_token_expiration: u64 = env::var("REFRESH_TOKEN_TTL")?
        .parse()?;
    let issuer = env::var("JWT_ISSUER")?;
    let jwt_secret = env::var("REFRESH_TOKEN_SECRET")?;

    let exp = Utc::now().timestamp() + refresh_token_expiration as i64;

    let claims = Claims {
        sub: user.id.to_string(),
        iss: issuer,
        iat: Utc::now().timestamp() as usize,
        exp: exp as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;

    Ok(Token {
        token,
        expires_in: refresh_token_expiration,
    })
}

pub async fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, Status> {
    pool.acquire().await.map_err(|_| Status::internal("Failed to acquire connection"))
}

fn get_redis_connection(r_client: &redis::Client) -> Result<redis::Connection, ApiError> {
    match r_client.get_connection() {
        Err(_) => Err(ApiError::RedisConnectionFailure),
        Ok(conn) => Ok(conn),
    }
}
