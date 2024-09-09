use std::env;
use std::sync::{Arc};

use ::log::{info, warn};
use tokio::task::JoinHandle;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use protos::auth::auth_server::AuthServer;
use crate::database::pg_database::PgPool;
use crate::{create_socket_addr, get_config, report_error};
use crate::config::Config;
use crate::services::auth_service::AuthService;

pub struct TonicServer {
    pub handle: JoinHandle<()>,
    pub tls: bool,
}

pub fn start_server(
    pool: Arc<PgPool>,
    cache: Arc<redis::Client>,
) -> Result<TonicServer, Box<dyn std::error::Error>> {
    let auth = AuthService { pool,  cache };

    let cfg = get_config!();

    let (mut tonic_server, secure_mode) = match get_tls_config(cfg.tls_cert.clone(), cfg.tls_key.clone(), cfg.ca_cert.clone()) {
        Some(tls) => {
            info!("Configuring TLS...");
            match Server::builder().tls_config(tls) {
                Ok(server) => {
                    info!("TLS successfully configured.");
                    (server, true)
                }
                Err(details) => {
                    info!("Error configuring TLS. Connections are not secure.");
                    report_error(&details);
                    (Server::builder(), false)
                }
            }
        }
        _ => {
            warn!("No TLS keys available. Connections are not secure.");
            (Server::builder(), false)
        }
    };

    let reflect = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(protos::auth::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let tonic_router = tonic_server
        .add_service(reflect)
        .add_service(AuthServer::new(auth));

    let port = cfg.port;
    let server = tokio::spawn(async move {
        let tonic_addr = create_socket_addr(port);
        info!("Starting server on port {}", port);
        match tonic_router.serve(tonic_addr).await {
            Ok(_) => info!("Server finished on {}", tonic_addr),
            Err(e) => {
                warn!("Unable to start server on port {}", port);
                report_error(&e);
            }
        };
        ()
    });

    Ok(TonicServer {
        handle: server,
        tls: secure_mode,
    })
}

fn get_tls_config(tls_cert: Option<String>, tls_key: Option<String>, ca_cert: Option<String>) -> Option<ServerTlsConfig> {
    match (tls_cert, tls_key, ca_cert) {
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