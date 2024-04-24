use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
