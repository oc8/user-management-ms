use std::env;
use std::sync::{Arc};
use autometrics::autometrics;
use chrono::Utc;
use tonic::{Code, Request, Response, Status};
use jsonwebtoken::{encode, EncodingKey, Header};

use protos::auth::{auth_server::Auth, RegisterRequest, User as UserProto, Tokens, LoginRequest, RegisterResponse, LoginResponse, ValidateOtpRequest, ValidateOtpResponse, ValidateTokenRequest, ValidateTokenResponse, RefreshTokenRequest, RefreshTokenResponse};
use serde::{Deserialize, Serialize};
use crate::database::{PgPool, PgPooledConnection};
use crate::models::user::User;
use crate::{errors, rpcs};

pub struct AuthService {
    pub pool: Arc<PgPool>,
    pub r_client: redis::Client,
}

impl Clone for AuthService {
    fn clone(&self) -> Self {
        AuthService {
            pool: self.pool.clone(),
            r_client: self.r_client.clone(),
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    #[autometrics]
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    )-> Result<Response<RegisterResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::register(request.into_inner(), &mut conn).map(Response::new)
    }

    #[autometrics]
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let mut r_conn = get_redis_connection(&self.r_client)?;
        rpcs::login(request.into_inner(), &mut conn, &mut r_conn).map(Response::new)
    }

    #[autometrics]
    async fn validate_otp(&self, request: Request<ValidateOtpRequest>) -> Result<Response<ValidateOtpResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let mut r_conn = get_redis_connection(&self.r_client)?;
        rpcs::validate_otp(request.into_inner(), &mut conn, &mut r_conn).map(Response::new)
    }

    #[autometrics]
    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status> {
        rpcs::validate_token(request.into_inner()).map(Response::new)
    }

    #[autometrics]
    async fn refresh_token(&self, request: Request<RefreshTokenRequest>) -> Result<Response<RefreshTokenResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let mut r_conn = get_redis_connection(&self.r_client)?;
        rpcs::refresh_token(request.into_inner(), &mut conn, &mut r_conn).map(Response::new)
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
    pub expires_it: u64,
}

pub(crate) fn generate_tokens(user: &User) -> Result<Tokens, Box<dyn std::error::Error>> {
    let access_token = generate_access_token(user)?;
    let refresh_token = generate_refresh_token(user)?;

    Ok(Tokens {
        access_token: access_token.token,
        refresh_token: refresh_token.token,
        expires_in: access_token.expires_it,
        token_type: "Bearer".to_string(),
        user: Option::from(UserProto {
            id: user.id.to_string(),
            email: user.email.clone(),
        })
    })
}

pub(crate) fn generate_access_token(user: &User) -> Result<Token, Box<dyn std::error::Error>> {
    let access_token_expiration: u64 = env::var("ACCESS_TOKEN_TTL")
        .map_err(|_| "ACCESS_TOKEN_TTL must be set")?
        .parse()
        .map_err(|_| "Failed to parse ACCESS_TOKEN_TTL")?;

    let issuer = env::var("JWT_ISSUER").map_err(|_| "JWT_ISSUER must be set")?;

    let jwt_secret = env::var("ACCESS_TOKEN_SECRET").map_err(|_| "ACCESS_TOKEN_SECRET must be set")?;

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
    )
        .map_err(|_| "Failed to generate access token")?;

    Ok(Token {
        token,
        expires_it: access_token_expiration,
    })
}

pub(crate) fn generate_refresh_token(user: &User) -> Result<Token, Box<dyn std::error::Error>> {
    let refresh_token_expiration: u64 = env::var("REFRESH_TOKEN_TTL")
        .map_err(|_| "REFRESH_TOKEN_TTL must be set")?
        .parse()
        .map_err(|_| "Failed to parse REFRESH_TOKEN_TTL")?;

    let issuer = env::var("JWT_ISSUER").map_err(|_| "JWT_ISSUER must be set")?;

    let jwt_secret = env::var("REFRESH_TOKEN_SECRET").map_err(|_| "REFRESH_TOKEN_SECRET must be set")?;

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
    )
        .map_err(|_| "Failed to generate refresh token")?;

    Ok(Token {
        token,
        expires_it: refresh_token_expiration,
    })
}

fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, Status> {
    match pool.get() {
        Err(_) => Err(Status::new(Code::DataLoss, errors::DATABASE_CONNECTION_FAILURE)),
        Ok(conn) => Ok(conn),
    }
}

fn get_redis_connection(r_client: &redis::Client) -> Result<redis::Connection, Status> {
    match r_client.get_connection() {
        Err(_) => Err(Status::new(Code::DataLoss, errors::REDIS_CONNECTION_FAILURE)),
        Ok(conn) => Ok(conn),
    }
}
