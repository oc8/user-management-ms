use std::env;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use diesel::PgConnection;

use protos::auth::{auth_service_server::AuthService, RegisterRequest, Tokens, LoginRequest, RegisterResponse, LoginResponse, ValidateOtpRequest, ValidateOtpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::{user::{User, NewUser}};
use crate::validations::{validate_register_request, validate_login_request};

pub struct Service {
    database: Arc<Mutex<PgConnection>>
}

impl Service {
    pub fn new(database: PgConnection) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
        }
    }
}

#[tonic::async_trait]
impl AuthService for Service {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        let conn = self.database.lock();
        let request = request.into_inner();
        validate_register_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let user = NewUser {
            email: &request.email,
        };

        let _user = User::create(&mut conn.unwrap(), user)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RegisterResponse {}))
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let conn = self.database.lock();
        let request = request.into_inner();
        validate_login_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let user = User::find_by_email(&mut conn.unwrap(), &request.email)
            .ok_or_else(|| Status::not_found("User not found"))?;

        Ok(Response::new(LoginResponse {}))
    }

    async fn validate_otp(&self, request: Request<ValidateOtpRequest>) -> Result<Response<ValidateOtpResponse>, Status> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    iat: usize,
    exp: usize,
}

fn generate_tokens(user_id: Uuid) -> Tokens {
    Tokens {
        access_token: generate_access_token(user_id),
        refresh_token: generate_refresh_token(user_id),
    }
}

fn generate_access_token(user_id: Uuid) -> String {
    let access_token_expiration = env::var("ACCESS_TOKEN_EXPIRATION").expect("ACCESS_TOKEN_EXPIRATION must be set");
    let issuer = env::var("JWT_ISSUER").expect("JWT_ISSUER must be set");
    let claims = Claims {
        sub: user_id.to_string(),
        iss: issuer,
        iat: chrono::Utc::now().timestamp() as usize,
        exp: access_token_expiration.parse().unwrap(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    token
}

fn generate_refresh_token(user_id: Uuid) -> String {
    let refresh_token_expiration = env::var("REFRESH_TOKEN_EXPIRATION").expect("REFRESH_TOKEN_EXPIRATION must be set");
    let issuer = env::var("JWT_ISSUER").expect("JWT_ISSUER must be set");
    let claims = Claims {
        sub: user_id.to_string(),
        iss: issuer,
        iat: chrono::Utc::now().timestamp() as usize,
        exp: refresh_token_expiration.parse().unwrap(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    token
}