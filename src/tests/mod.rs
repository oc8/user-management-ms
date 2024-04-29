use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use diesel::{Connection, PgConnection};
use diesel::migration::MigrationSource;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use redis::Client;
use tokio::io::Join;
use tokio::runtime::Handle;
use crate::database::establish_pool;
use crate::services::auth::{AuthService, get_connection};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub mod integration;

struct TestContext {
    db_url: String,
    db_name: String,
    addr: SocketAddr,
    url: String,
    service: AuthService,
}

impl TestContext {
    fn new(db_url: &str, db_name: &str, r_url: &str, port: u16) -> Self {
        let mut conn =
            PgConnection::establish(&format!("{}/postgres", db_url)).expect("Cannot connect to postgres database.");

        let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name).as_str());
        query
            .execute(&mut conn)
            .expect(format!("Could not create database {}", db_name).as_str());

        let pool = Arc::new(establish_pool(format!("{}/{}", db_url, db_name)));

        let mut conn = get_connection(&pool)
            .expect("Cannot connect to database");
        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
        let url = format!("http://{}:{}", addr.ip(), addr.port());

        let r_client = Client::open(r_url)
            .expect("Cannot connect to redis server");

        let auth = AuthService {
            pool,
            r_client,
        };

        Self {
            db_url: db_url.to_string(),
            db_name: db_name.to_string(),
            addr,
            url,
            service: auth,
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let mut conn =
            PgConnection::establish(&format!("{}/postgres", self.db_url)).expect("Cannot connect to postgres database.");

        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            self.db_name
        );

        diesel::sql_query(disconnect_users.as_str())
            .execute(&mut conn)
            .unwrap();


        let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());
        query
            .execute(&mut conn)
            .expect(&format!("Couldn't drop database {}", self.db_name));
    }
}
