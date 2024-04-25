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

    let uri_scheme = match env::var("REDIS_TLS").unwrap_or_default().parse::<bool>() {
        Ok(bool) => if bool { "rediss" } else { "redis" },
        Err(_) => "redis",
    };

    let redis_host = env::var("REDIS_HOSTNAME").expect("REDIS_HOSTNAME must be set");
    let redis_pass = env::var("REDIS_PASSWORD").unwrap_or_default();
    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_pass, redis_host);
    let r_client = Client::open(redis_conn_url)?;

    let server = start_server(pool.clone(), r_client, port);

    server?.handle.await?;

    Ok(())
}