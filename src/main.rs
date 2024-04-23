use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use tonic::transport::Server;
use protos::auth::auth_service_server::AuthServiceServer;

mod gateways;
mod schema;
mod models;
mod validations;

// mod proto {
//     tonic::include_proto!("auth");
//
//     pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
//         tonic::include_file_descriptor_set!("auth_descriptor");
// }

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

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AuthServiceServer::new(gateways::auth::Service::new(database)))
        .serve(addr)
        .await?;

    Ok(())
}