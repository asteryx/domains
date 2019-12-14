pub mod models;
pub mod schema;

use crate::config::Config;
use crate::AppState;
use actix::{Actor, Addr, SyncArbiter, SyncContext};
use actix_web::web;
use diesel::prelude::PgConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Error, Pool};
use dotenv;
use num_cpus;
use rand::prelude::ThreadRng;
use rand::thread_rng;

pub enum DbError {
    GetConnectionError,
}

pub struct DbExecutor {
    pool: Pool<ConnectionManager<PgConnection>>,
}

unsafe impl Send for DbExecutor {}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new() -> DbExecutor {
        DbExecutor {
            pool: init_pool(&Config::from_file()),
        }
    }
    pub fn get_connection(
        &self,
    ) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, DbError> {
        match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(_) => Err(DbError::GetConnectionError),
        }
    }
}

pub fn init_pool(config: &Config) -> Pool<ConnectionManager<PgConnection>> {
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(res) => res,
        Err(_) => config.database_url.to_string(),
    };

    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool.")
}
