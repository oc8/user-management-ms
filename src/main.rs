use std::env;
use std::sync::Arc;

use dotenvy::dotenv;
use redis::Client;
use user_management::init_service_logging;
use crate::server::start_server;

mod services;
mod schema;
mod models;
mod validations;
mod server;
mod database;
mod rpcs;

// mod proto {
//     tonic::include_proto!("auth");
//
//     pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
//         tonic::include_file_descriptor_set!("auth_descriptor");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_service_logging();

    let pool = Arc::new(database::establish_pool());

    pool
        .get()
        .expect("Couldn't get a connection from the pool");

    let port = env::var("PORT").unwrap_or_else(|_| "50051".to_string()).parse().expect("PORT must be a number");

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let r_client = Client::open(redis_url)?;

    let server = start_server(pool.clone(), r_client, port);

    server?.handle.await?;

    Ok(())
}