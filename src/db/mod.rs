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

pub fn init_pool(config: Config) -> Pool<ConnectionManager<SqliteConnection>> {
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(res) => res,
        Err(_) => config.db.database_url,
    };
    //    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //    dbg!(&db_url);
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.")
}
