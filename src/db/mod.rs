pub mod models;
pub mod schema;

use crate::config::Config;
use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::prelude::SqliteConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Error, Pool};
use dotenv;
use num_cpus;

//pub struct ConnDsl(pub Pool<ConnectionManager<PgConnection>>);
pub struct ConnDsl(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}

pub fn init_pool(config: &Config) -> Pool<ConnectionManager<SqliteConnection>> {
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(res) => res,
        Err(_) => config.database_url.to_string(),
    };

    let manager: ConnectionManager<SqliteConnection> =
        ConnectionManager::<SqliteConnection>::new(db_url);
    Pool::builder()
        .max_size(6)
        .build(manager)
        .expect("Failed to create pool.")
}
