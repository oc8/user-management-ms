use std::env;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Utc;
use tonic::{Request, Response, Status};
use diesel::PgConnection;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use protos::auth::{auth_service_server::AuthService, RegisterRequest, Tokens, LoginRequest, RegisterResponse, LoginResponse, ValidateOtpRequest, ValidateOtpResponse, ValidateTokenRequest, ValidateTokenResponse};
use serde::{Deserialize, Serialize};
use totp_rs::{TOTP};
use uuid::Uuid;
use crate::models::{user::{User, NewUser}};
use crate::validations::{validate_register_request, validate_login_request, validate_token_request, validate_otp_request};

pub struct Service {
    database: Arc<Mutex<PgConnection>>,
    totp: TOTP
}

impl Service {
    pub fn new(database: PgConnection, totp: TOTP) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
            totp
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

        User::find_by_email(&mut conn.unwrap(), &request.email)
            .ok_or_else(|| Status::not_found("User not found"))?;

        let code = self.totp.generate(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

        println!("Code: {}", code);

        Ok(Response::new(LoginResponse {}))
    }

    async fn validate_otp(&self, request: Request<ValidateOtpRequest>) -> Result<Response<ValidateOtpResponse>, Status> {
        let request = request.into_inner();
        validate_otp_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;
        let timestamp2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let valid = self.totp.check(&request.otp, timestamp2);

        if valid {
            let user = User::find_by_email(&mut self.database.lock().unwrap(), &request.email)
                .ok_or_else(|| Status::not_found("User not found"))?;

            let tokens = generate_tokens(user.id);

            Ok(Response::new(ValidateOtpResponse {
                tokens: Some(tokens),
            }))
        } else {
            Err(Status::invalid_argument("Invalid code"))
        }
    }

    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status> {
        let request = request.into_inner();
        validate_token_request(&request).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token = decode::<Claims>(&request.token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

        if token.is_err() {
            return Err(Status::invalid_argument("Invalid token"));
        }

        println!("{:?}", token);

        Ok(Response::new(ValidateTokenResponse {
            valid: true,
        }))
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
    let access_token_expiration: u64 = env::var("ACCESS_TOKEN_EXPIRATION")
        .expect("ACCESS_TOKEN_EXPIRATION must be set")
        .parse()
        .expect("Failed to parse ACCESS_TOKEN_EXPIRATION");

    let issuer = env::var("JWT_ISSUER")
        .expect("JWT_ISSUER must be set");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let exp = Utc::now().timestamp() + access_token_expiration as i64;

    let claims = Claims {
        sub: user_id.to_string(),
        iss: issuer,
        iat: Utc::now().timestamp() as usize,
        exp: exp as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
        .expect("Failed to generate JWT");

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

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    token
}