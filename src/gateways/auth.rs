use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use diesel::PgConnection;

use protos::auth::{auth_server::Auth, RegisterRequest, Token, LoginRequest};
use crate::models::{user::{User, NewUser}};

pub struct AuthService {
    database: Arc<Mutex<PgConnection>>
}

impl AuthService {
    pub fn new(database: PgConnection) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<Token>, Status> {
        let conn = self.database.lock();
        let request = request.into_inner();
        let user = NewUser {
            email: &request.email,
        };

        let _user = User::create(&mut conn.unwrap(), user)
            .map_err(|_| Status::already_exists("User already exists"))?;

        Ok(Response::new(Token {
            access_token: "access_token".to_string(),
        }))
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
        let conn = self.database.lock();
        let request = request.into_inner();
        let user = User::find_by_email(&mut conn.unwrap(), &request.email)
            .ok_or_else(|| Status::not_found("User not found"))?;

        Ok(Response::new(Token {
            access_token: user.id.to_string(),
        }))
    }
}