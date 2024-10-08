use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;
use dotenvy::dotenv;

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub database_url: String,
    pub database_min_connections: u32,
    pub database_max_connections: u32,
    pub database_max_lifetime: u64,
    pub redis_hostname: String,
    pub redis_password: String,
    pub redis_tls: bool,
    pub access_token_ttl: usize,
    pub access_token_secret: String,
    pub refresh_token_ttl: usize,
    pub refresh_token_secret: String,
    pub jwt_issuer: String,
    pub otp_secret: String,
    pub otp_ttl: u64,
    pub magic_link_redirect_url: String,
    pub enable_ipv6: bool,
    pub port: u16,
    pub metrics_port: u16,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
    pub ca_cert: Option<String>,
    pub email_from_name: String,
    pub email_from_email: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenv().ok();

        let config = config::Config::builder()
            .add_source(config::Environment::default().try_parsing(true))
            .set_default("database_min_connections", 1).unwrap()
            .set_default("database_max_connections", 16).unwrap()
            .set_default("database_max_lifetime", 3600).unwrap()
            .set_default("redis_hostname", "").unwrap()
            .set_default("redis_password", "").unwrap()
            .set_default("redis_tls", false).unwrap()
            .set_default("cache_ttl", 60).unwrap()
            .set_default("enable_cache", false).unwrap()
            .set_default("enable_ipv6", false).unwrap()
            .set_default("port", 50051).unwrap()
            .set_default("metrics_port", 3000).unwrap()
            .set_default("tls_cert", None::<String>).unwrap()
            .set_default("tls_key", None::<String>).unwrap()
            .set_default("ca_cert", None::<String>).unwrap()
            .build()?;

        let cfg: Config = config.try_deserialize()?;

        log::debug!("Config: {:?}", cfg);

        Ok(cfg)
    }
}

pub static CONFIG: Lazy<Arc<Config>> = Lazy::new(|| {
    Arc::new(Config::from_env().expect("Failed to load configuration"))
});

#[macro_export]
macro_rules! get_config {
    () => {
        &*crate::config::CONFIG
    };
}
