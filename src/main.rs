use std::env;
use std::ops::Deref;
use std::sync::Arc;

use axum::{routing::get, Router};
use autometrics::prometheus_exporter;
use dotenvy::dotenv;
use redis::Client;
use user_service::{create_socket_addr, database, get_config, init_service_logging};
use user_service::config::{
    Config, self
};
use user_service::server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_service_logging();
    prometheus_exporter::init();

    let cfg = get_config!();

    // Set up the database connection
    // database::check_for_migrations(&database_url.clone())
    //     .await
    //     .expect("An error occurred while running migrations");

    let pool = database::connect(&cfg)
        .await
        .expect("Couldn't connect to the database");

    // Set up the Redis connection
    let uri_scheme = match cfg.redis_tls {
        true => "rediss",
        false => "redis",
    };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, cfg.redis_password, cfg.redis_hostname);
    log::info!("Connecting to Redis at {}", redis_conn_url);
    let r_client = Client::open(redis_conn_url)?;

    let _server = start_server(
        Arc::new(pool),
        Arc::new(r_client),
    );

    let app = Router::new().route(
        "/metrics",
        get(|| async { prometheus_exporter::encode_http_response() }),
    );

    let metrics_port = cfg.metrics_port;
    let metrics_addr = create_socket_addr(metrics_port);
    let listener = tokio::net::TcpListener::bind(metrics_addr).await.unwrap();
    log::info!("Metrics server listening on port {}", metrics_port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}