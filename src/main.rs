use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use tonic::transport::Server;
use protos::auth::auth_server::AuthServer;

mod gateways;
mod schema;
mod models;

pub fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database = connect_db();
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(AuthServer::new(gateways::auth::AuthService::new(database)))
        .serve(addr)
        .await?;

    Ok(())
}