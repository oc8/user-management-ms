use std::sync::Arc;

use ::log::{info, warn};
use std::net::SocketAddr;
use tokio::task::JoinHandle;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
// use tower_http::cors::CorsLayer;
use protos::auth::auth_server::AuthServer;
use user_management::{env_var, report_error};
use crate::database::PgPool;
use crate::services::auth::AuthService;

pub struct TonicServer {
    pub handle: JoinHandle<()>,
    pub tls: bool,
}

pub fn start_server(
    pool: Arc<PgPool>,
    r_client: redis::Client,
    port: u16,
) -> Result<TonicServer, Box<dyn std::error::Error>> {
    let auth = AuthService { pool,  r_client };

    let (tonic_server, secure_mode) = match get_tls_config() {
        Some(tls) => {
            info!("Configuring TLS...");
            match Server::builder().tls_config(tls) {
                Ok(server) => {
                    info!("TLS successfully configured.");
                    (server, true)
                }
                Err(details) => {
                    info!("Error configuring TLS. Connections are not secure.");
                    report_error(details);
                    (Server::builder(), false)
                }
            }
        }
        _ => {
            warn!("No TLS keys available. Connections are not secure.");
            (Server::builder(), false)
        }
    };

    let tonic_router = tonic_server
        .accept_http1(true)
        // .layer(CorsLayer::permissive())
        .add_service(AuthServer::new(auth));

    let server = tokio::spawn(async move {
        let tonic_addr = SocketAddr::from(([0, 0, 0, 0], port));
        info!("Starting server on port {}", port);
        match tonic_router.serve(tonic_addr).await {
            Ok(_) => info!("Server finished on {}", tonic_addr),
            Err(e) => {
                ::log::warn!("Unable to start server on port {}", port);
                report_error(e);
            }
        };
        ()
    });

    Ok(TonicServer {
        handle: server,
        tls: secure_mode,
    })
}

fn get_tls_config() -> Option<ServerTlsConfig> {
    let cert = env_var("TLS_CERT");
    let key = env_var("TLS_KEY");
    let ca_cert = env_var("CA_CERT");

    match (cert, key, ca_cert) {
        (Some(cert), Some(key), Some(ca_cert)) => {
            info!("Configuring TLS with custom CA...");
            Some(
                ServerTlsConfig::new()
                    .identity(Identity::from_pem(cert, key))
                    .client_ca_root(Certificate::from_pem(ca_cert)),
            )
        }
        (Some(cert), Some(key), None) => {
            info!("Configuring TLS with official CAs...");
            Some(ServerTlsConfig::new().identity(Identity::from_pem(cert, key)))
        }
        _ => None,
    }
}
