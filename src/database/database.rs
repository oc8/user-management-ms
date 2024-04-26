use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> PgPool {
    let database_host = env::var("PG_HOST").expect("PG_HOST must be set");
    let database_port = env::var("PG_PORT").expect("PG_PORT must be set");
    let database_user = env::var("PG_USER").expect("PG_USER must be set");
    let database_password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
    let database_name = env::var("PG_DATABASE").expect("PG_DATABASE must be set");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_user, database_password, database_host, database_port, database_name
    );

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
